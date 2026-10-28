//! CPython-compatible number formatting.
//! Ports the `format(value, spec)` calls in `U:1325 _format`
//! (`U:1362`, `U:1373`, `U:1381`) for the spec subset the kernel
//! accepts. Anything outside the subset reports [`Fallback`] so the
//! pure-Python path reproduces upstream behavior (including errors)
//! exactly.
//!
//! The engine is verified against 1400+ CPython-computed vectors
//! (`fmt_vectors_test.rs`, generated — see README).

use crate::Fallback;

pub const DEFAULT_FLOAT: &str = "g";
pub const DEFAULT_INT: &str = "";

/// Parse one spec per column (already expanded by the binding).
pub fn parse_specs(specs: &[String]) -> Result<Vec<Spec>, Fallback> {
    specs.iter().map(|s| parse_spec(s)).collect()
}

impl Spec {
    fn plain() -> Spec {
        Spec {
            fill: ' ',
            align: None,
            sign: Sign::Minus,
            alternate: false,
            zero: false,
            width: 0,
            grouping: Grouping::None,
            precision: None,
            ty: FType::None,
        }
    }

    /// The default float spec (`"g"`).
    pub fn default_float() -> Spec {
        Spec {
            ty: FType::GenLower,
            ..Spec::plain()
        }
    }

    /// The default int spec (`""`).
    pub fn default_int() -> Spec {
        Spec::plain()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AlignKind {
    Left,
    Right,
    Center,
    Numeric,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Sign {
    Minus,
    Plus,
    Space,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Grouping {
    None,
    Comma,
    Underscore,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FType {
    None,
    Bin,
    Oct,
    Dec,
    HexLower,
    HexUpper,
    Char,
    Number,
    ExpLower,
    ExpUpper,
    FixedLower,
    FixedUpper,
    GenLower,
    GenUpper,
    Percent,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Spec {
    pub fill: char,
    pub align: Option<AlignKind>,
    pub sign: Sign,
    pub alternate: bool,
    pub zero: bool,
    pub width: usize,
    pub grouping: Grouping,
    pub precision: Option<usize>,
    pub ty: FType,
}

/// Parse a format spec (`[[fill]align][sign][#][0][width][,][.prec][type]`).
/// Returns `Err(Fallback)` for anything outside the supported subset;
/// the fallback reproduces the exact upstream error.
pub fn parse_spec(s: &str) -> Result<Spec, Fallback> {
    // [fill]align: one align char, optionally preceded by a fill char.
    let mut chars = s.chars();
    let c0 = chars.next();
    let c1 = chars.next();
    let is_align = |c: Option<char>| matches!(c, Some('<') | Some('>') | Some('=') | Some('^'));
    let mut fill = ' ';
    let mut align: Option<AlignKind> = None;
    let i: usize;
    if is_align(c1) {
        fill = c0.unwrap();
        align = Some(match c1.unwrap() {
            '<' => AlignKind::Left,
            '>' => AlignKind::Right,
            '=' => AlignKind::Numeric,
            _ => AlignKind::Center,
        });
        i = c0.unwrap().len_utf8() + 1;
    } else if is_align(c0) {
        align = Some(match c0.unwrap() {
            '<' => AlignKind::Left,
            '>' => AlignKind::Right,
            '=' => AlignKind::Numeric,
            _ => AlignKind::Center,
        });
        i = 1;
    } else {
        i = 0;
    }
    let rest = &s[i.min(s.len())..];
    let b = rest.as_bytes();
    let mut j = 0;
    // sign
    let mut sign = Sign::Minus;
    if j < b.len() && matches!(b[j], b'+' | b'-' | b' ') {
        sign = match b[j] {
            b'+' => Sign::Plus,
            b' ' => Sign::Space,
            _ => Sign::Minus,
        };
        j += 1;
    }
    // alternate
    let mut alternate = false;
    if j < b.len() && b[j] == b'#' {
        alternate = true;
        j += 1;
    }
    // zero flag (only when no explicit fill/align)
    let mut zero = false;
    if j < b.len() && b[j] == b'0' && align.is_none() {
        zero = true;
        j += 1;
    }
    // width
    let mut width: usize = 0;
    let mut has_width = false;
    while j < b.len() && b[j].is_ascii_digit() {
        has_width = true;
        width = width
            .saturating_mul(10)
            .saturating_add((b[j] - b'0') as usize);
        if width > 1_000_000 {
            return Err(Fallback);
        }
        j += 1;
    }
    let _ = has_width;
    // grouping
    let mut grouping = Grouping::None;
    if j < b.len() && (b[j] == b',' || b[j] == b'_') {
        grouping = if b[j] == b',' {
            Grouping::Comma
        } else {
            Grouping::Underscore
        };
        j += 1;
    }
    // precision
    let mut precision: Option<usize> = None;
    if j < b.len() && b[j] == b'.' {
        j += 1;
        let mut p: usize = 0;
        let mut nd = 0;
        while j < b.len() && b[j].is_ascii_digit() {
            nd += 1;
            p = p.saturating_mul(10).saturating_add((b[j] - b'0') as usize);
            if p > 1_000_000 {
                return Err(Fallback);
            }
            j += 1;
        }
        if nd == 0 {
            return Err(Fallback);
        }
        precision = Some(p);
    }
    // type (exactly one char, then end)
    let ty = if j < b.len() {
        let t = b[j];
        j += 1;
        if j != b.len() {
            return Err(Fallback);
        }
        match t {
            b'b' => FType::Bin,
            b'o' => FType::Oct,
            b'd' => FType::Dec,
            b'x' => FType::HexLower,
            b'X' => FType::HexUpper,
            b'c' => FType::Char,
            b'n' => FType::Number,
            b'e' => FType::ExpLower,
            b'E' => FType::ExpUpper,
            b'f' => FType::FixedLower,
            b'F' => FType::FixedUpper,
            b'g' => FType::GenLower,
            b'G' => FType::GenUpper,
            b'%' => FType::Percent,
            _ => return Err(Fallback),
        }
    } else {
        FType::None
    };
    if matches!(ty, FType::Char) && (alternate || zero || precision.is_some()) {
        // CPython ignores '#'/'0'/precision for 'c'? Actually
        // format(65, '05c') works ('    A'?). Only '#' is... to stay
        // safe, bail on any modifier with 'c' except width/align/fill.
        if alternate || zero || precision.is_some() {
            return Err(Fallback);
        }
    }
    Ok(Spec {
        fill,
        align,
        sign,
        alternate,
        zero,
        width,
        grouping,
        precision,
        ty,
    })
}

fn group_int(digits: &str, sep: char) -> String {
    if digits.len() <= 3 {
        return digits.to_string();
    }
    let mut out = String::with_capacity(digits.len() + digits.len() / 3);
    let rem = digits.len() % 3;
    let mut first = true;
    if rem > 0 {
        out.push_str(&digits[..rem]);
        first = false;
    }
    let mut i = rem;
    while i < digits.len() {
        if !first {
            out.push(sep);
        }
        first = false;
        out.push_str(&digits[i..i + 3]);
        i += 3;
    }
    out
}

fn pad(spec: &Spec, sign: &str, body: &str, is_numeric: bool) -> String {
    // Width counts characters, not bytes.
    let total = sign.chars().count() + body.chars().count();
    if total >= spec.width {
        let mut out = String::with_capacity(sign.len() + body.len());
        out.push_str(sign);
        out.push_str(body);
        return out;
    }
    let n = spec.width - total;
    // Zero flag without explicit align means numeric padding with '0'.
    let (align, fill) = if spec.zero && spec.align.is_none() {
        (AlignKind::Numeric, "0".repeat(n))
    } else {
        let align = spec.align.unwrap_or(if is_numeric {
            AlignKind::Right
        } else {
            AlignKind::Left
        });
        let fill: String = std::iter::repeat_n(spec.fill, n).collect();
        (align, fill)
    };
    match align {
        AlignKind::Left => format!("{sign}{body}{fill}"),
        AlignKind::Right => format!("{fill}{sign}{body}"),
        AlignKind::Center => {
            let left = n / 2;
            let right = n - left;
            let l: String = std::iter::repeat_n(spec.fill, left).collect();
            let r: String = std::iter::repeat_n(spec.fill, right).collect();
            format!("{l}{sign}{body}{r}")
        }
        AlignKind::Numeric => format!("{sign}{fill}{body}"),
    }
}

fn sign_str(spec: &Spec, negative: bool) -> &'static str {
    if negative {
        "-"
    } else {
        match spec.sign {
            Sign::Plus => "+",
            Sign::Space => " ",
            Sign::Minus => "",
        }
    }
}

/// Format an i64 with an int-compatible spec.
pub fn format_int(v: i64, spec: &Spec) -> Result<String, Fallback> {
    let negative = v < 0;
    let sign = sign_str(spec, negative);
    let a = v.wrapping_abs() as u64;
    // For i64::MIN, wrapping_abs gives 2^63, correct digits.
    let body = match spec.ty {
        FType::None => {
            if spec.alternate || spec.grouping != Grouping::None {
                // `format(42, '#')` -> ValueError upstream; grouping with
                // no type is also an error.
                if spec.alternate {
                    return Err(Fallback);
                }
            }
            let d = a.to_string();
            match spec.grouping {
                Grouping::None => d,
                Grouping::Comma => group_int(&d, ','),
                Grouping::Underscore => group_int(&d, '_'),
            }
        }
        FType::Dec | FType::Number => {
            let d = a.to_string();
            match spec.grouping {
                Grouping::None => d,
                Grouping::Comma => group_int(&d, ','),
                Grouping::Underscore => group_int(&d, '_'),
            }
        }
        FType::Bin => {
            let d = format!("{:b}", a);
            let d = grouped(d, spec);
            if spec.alternate {
                format!("0b{d}")
            } else {
                d
            }
        }
        FType::Oct => {
            let d = format!("{:o}", a);
            let d = grouped(d, spec);
            if spec.alternate {
                format!("0o{d}")
            } else {
                d
            }
        }
        FType::HexLower => {
            let d = format!("{:x}", a);
            let d = grouped(d, spec);
            if spec.alternate {
                format!("0x{d}")
            } else {
                d
            }
        }
        FType::HexUpper => {
            let d = format!("{:X}", a);
            let d = grouped(d, spec);
            if spec.alternate {
                format!("0X{d}")
            } else {
                d
            }
        }
        FType::Char => {
            if negative {
                return Err(Fallback);
            }
            let c = char::from_u32(v as u32).ok_or(Fallback)?;
            // '#'/'0'/precision already rejected in the parser.
            return Ok(pad(spec, "", &c.to_string(), false));
        }
        // Int with float presentation: convert exactly like float(int).
        FType::ExpLower
        | FType::ExpUpper
        | FType::FixedLower
        | FType::FixedUpper
        | FType::GenLower
        | FType::GenUpper
        | FType::Percent => {
            return format_float(v as f64, spec);
        }
    };
    Ok(pad(spec, sign, &body, true))
}

fn grouped(digits: String, spec: &Spec) -> String {
    match spec.grouping {
        Grouping::None => digits,
        Grouping::Comma => group_int(&digits, ','),
        Grouping::Underscore => group_int(&digits, '_'),
    }
}

/// Shortest-round-trip repr of an f64, Python style (`U:1343 f"{val}"`).
pub fn float_repr(v: f64) -> String {
    if v.is_nan() {
        return "nan".to_string();
    }
    if v.is_infinite() {
        return if v.is_sign_negative() {
            "-inf".to_string()
        } else {
            "inf".to_string()
        };
    }
    let neg = v.is_sign_negative();
    let a = v.abs();
    if a == 0.0 {
        return if neg {
            "-0.0".to_string()
        } else {
            "0.0".to_string()
        };
    }
    let (digits, exp) = shortest_exp_parts(a);
    let body = if (-4..16).contains(&exp) {
        fixed_from_parts(&digits, exp, false, true)
    } else {
        sci_from_parts(&digits, exp, false, false)
    };
    if neg {
        format!("-{body}")
    } else {
        body
    }
}

/// Shortest digits + decimal exponent of `v > 0` via Rust's
/// correctly-rounded shortest `{:e}` formatting: value =
/// `0.digits x 10^(exp+1)`, i.e. `d.ddd x 10^exp`.
fn shortest_exp_parts(v: f64) -> (String, i32) {
    let s = format!("{:e}", v);
    let epos = s.find('e').unwrap();
    let mant = &s[..epos];
    let exp: i32 = s[epos + 1..].parse().unwrap();
    let digits: String = mant.chars().filter(|c| *c != '.').collect();
    (digits, exp)
}

/// Fixed notation from significant digits + exponent: point position
/// `P = exp + 1`. `alternate` forces a decimal point; `force_point`
/// appends `.0` for integral values (Python `repr` style, not `g` style).
fn fixed_from_parts(digits: &str, exp: i32, alternate: bool, force_point: bool) -> String {
    let p = exp + 1;
    if p <= 0 {
        let zeros = "0".repeat((-p) as usize);
        format!("0.{zeros}{digits}")
    } else if (p as usize) < digits.len() {
        let p = p as usize;
        format!("{}.{}", &digits[..p], &digits[p..])
    } else {
        let zeros = "0".repeat((p as usize) - digits.len());
        let intpart = format!("{digits}{zeros}");
        if alternate {
            format!("{intpart}.")
        } else if force_point {
            format!("{intpart}.0")
        } else {
            intpart
        }
    }
}

/// Scientific notation from significant digits + exponent.
fn sci_from_parts(digits: &str, exp: i32, upper: bool, alternate: bool) -> String {
    let mut mant = String::with_capacity(digits.len() + 1);
    mant.push(digits.chars().next().unwrap());
    let rest = &digits[1..];
    if !rest.is_empty() || alternate {
        mant.push('.');
        mant.push_str(rest);
    }
    let e = if upper { 'E' } else { 'e' };
    let (sign, mag) = if exp < 0 {
        ('-', (-exp) as i64)
    } else {
        ('+', exp as i64)
    };
    format!("{mant}{e}{sign}{mag:02}")
}

/// Format an f64 with a float-compatible spec.
pub fn format_float(v: f64, spec: &Spec) -> Result<String, Fallback> {
    // Int-only codes on floats raise upstream (`U:1381` is inside
    // try/except only in the colored branch; the plain branch catches
    // ValueError too and falls back to f"{val}"!). Re-read:
    // `U:1377-1383`: try format(float(val), floatfmt) except
    // (ValueError, TypeError): return f"{val}". So a float with 'd'
    // spec renders as str(float)! NOT an error!
    if matches!(
        spec.ty,
        FType::Bin | FType::Oct | FType::Dec | FType::HexLower | FType::HexUpper | FType::Char
    ) {
        return Ok(float_repr(v));
    }
    if v.is_nan() || v.is_infinite() {
        let neg = v.is_sign_negative();
        let sign = sign_str(spec, neg);
        let upper = matches!(
            spec.ty,
            FType::ExpUpper | FType::FixedUpper | FType::GenUpper
        );
        let mut body = if v.is_nan() {
            if upper {
                "NAN".to_string()
            } else {
                "nan".to_string()
            }
        } else if upper {
            "INF".to_string()
        } else {
            "inf".to_string()
        };
        if spec.ty == FType::Percent {
            body.push('%');
        }
        // Zero-padding is ignored for inf/nan (spaces instead).
        let width = spec.width;
        let total = sign.len() + body.len();
        if total >= width {
            return Ok(format!("{sign}{body}"));
        }
        let n = width - total;
        let (align, fill) = match spec.align {
            Some(AlignKind::Left) => (AlignKind::Left, spec.fill),
            Some(AlignKind::Center) => (AlignKind::Center, spec.fill),
            _ => (AlignKind::Right, ' '),
        };
        let fillstr: String = std::iter::repeat_n(fill, n).collect();
        return Ok(match align {
            AlignKind::Left => format!("{sign}{body}{fillstr}"),
            AlignKind::Center => {
                let l: String = std::iter::repeat_n(fill, n / 2).collect();
                let r: String = std::iter::repeat_n(fill, n - n / 2).collect();
                format!("{l}{sign}{body}{r}")
            }
            _ => format!("{fillstr}{sign}{body}"),
        });
    }
    let neg = v.is_sign_negative();
    let sign = sign_str(spec, neg).to_string();
    let a = v.abs();
    let upper = matches!(
        spec.ty,
        FType::ExpUpper | FType::FixedUpper | FType::GenUpper
    );
    let body = match spec.ty {
        // `format(1.5, "+10")`: repr body with sign/width applied.
        FType::None => float_repr(a),
        FType::FixedLower | FType::FixedUpper => {
            let prec = spec.precision.unwrap_or(6);
            let mut s = format!("{:.prec$}", a, prec = prec.min(1_000_000));
            if spec.alternate && !s.contains('.') {
                s.push('.');
            }
            apply_grouping_fixed(&s, spec)
        }
        FType::ExpLower | FType::ExpUpper => {
            let prec = spec.precision.unwrap_or(6);
            let s = format!("{:.*e}", prec.min(1_000_000), a);
            exp_reformat(&s, spec.alternate, upper)
        }
        FType::GenLower | FType::GenUpper | FType::Number => {
            let prec = spec.precision.unwrap_or(6).max(1);
            if a == 0.0 {
                if spec.alternate {
                    format!("0.{}", "0".repeat(prec - 1))
                } else {
                    "0".to_string()
                }
            } else {
                gen_format(a, prec, spec.alternate, upper, spec.grouping)
            }
        }
        FType::Percent => {
            let prec = spec.precision.unwrap_or(6);
            let s = format!("{:.prec$}", a * 100.0, prec = prec.min(1_000_000));
            let mut s = apply_grouping_fixed(&s, spec);
            s.push('%');
            s
        }
        _ => unreachable!(),
    };
    Ok(pad(spec, &sign, &body, true))
}

fn apply_grouping_fixed(s: &str, spec: &Spec) -> String {
    if spec.grouping == Grouping::None {
        return s.to_string();
    }
    let sep = match spec.grouping {
        Grouping::Comma => ',',
        Grouping::Underscore => '_',
        Grouping::None => return s.to_string(),
    };
    match s.find('.') {
        Some(p) => format!("{}.{}", group_int(&s[..p], sep), &s[p + 1..]),
        None => group_int(s, sep),
    }
}

/// Reformat Rust `{:.*e}` output ("d.ddddeX") to Python style.
fn exp_reformat(rust: &str, alternate: bool, upper: bool) -> String {
    let epos = rust.find(['e', 'E']).unwrap();
    let mant = &rust[..epos];
    let exp: i32 = rust[epos + 1..].parse().unwrap();
    let mut mant = mant.to_string();
    if alternate && !mant.contains('.') {
        mant.push('.');
    }
    let e = if upper { 'E' } else { 'e' };
    let (sign, mag) = if exp < 0 {
        ('-', -exp as i64)
    } else {
        ('+', exp as i64)
    };
    format!("{mant}{e}{sign}{mag:02}")
}

/// 'g'-style formatting from an exact value.
fn gen_format(a: f64, prec: usize, alternate: bool, upper: bool, grouping: Grouping) -> String {
    // Round to `prec` significant digits via e-format, then strip.
    let s = format!("{:.*e}", prec - 1, a);
    let epos = s.find(['e', 'E']).unwrap();
    let mant = &s[..epos];
    let exp: i32 = s[epos + 1..].parse().unwrap();
    let mut digits: String = mant.chars().filter(|c| *c != '.').collect();
    if !alternate {
        while digits.len() > 1 && digits.ends_with('0') {
            digits.pop();
        }
    }
    if (-4..prec as i32).contains(&exp) {
        let mut out = fixed_from_parts(&digits, exp, alternate, false);
        if alternate && !out.contains('.') {
            out.push('.');
        }
        // Grouping only applies to fixed notation.
        if grouping != Grouping::None {
            let sep = if grouping == Grouping::Comma {
                ','
            } else {
                '_'
            };
            out = match out.find('.') {
                Some(p) => format!("{}.{}", group_int(&out[..p], sep), &out[p + 1..]),
                None => group_int(&out, sep),
            };
        }
        out
    } else {
        sci_from_parts(&digits, exp, upper, alternate)
    }
}

/// Format a cell value: dispatch int/float by spec compatibility.
/// `int_val`/`float_val`: exactly one is `Some`.
pub fn format_num(
    int_val: Option<i64>,
    float_val: Option<f64>,
    spec: &Spec,
) -> Result<String, Fallback> {
    match (int_val, float_val) {
        (Some(v), _) => format_int(v, spec),
        (_, Some(v)) => format_float(v, spec),
        _ => Err(Fallback),
    }
}
