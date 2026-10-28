//! Per-cell type sniffing and formatting.
//! Ports `U:862` (`_isnumber_with_thousands_separator`), `U:895`
//! (`_isconvertible`), `U:903` (`_isnumber`), `U:955` (`_isint`),
//! `U:974` (`_isbool`), `U:988` (`_type`), `U:1038` (`_afterpoint`),
//! `U:1278` (`_more_generic`), `U:1299` (`_column_type`), `U:1325`
//! (`_format`), plus the plain-text scans (`U:2367`, `U:2377`, `U:2383`).

use crate::{ansi, numfmt, Cell, ColType, Fallback, InputCell};

/// `U:2377`: `has_invisible = _ansi_codes.search(plain_text) is not None`,
/// where `plain_text` is the tab-joined `_to_str` of headers + all cells.
/// `_to_str` of int/float/bool/None (`str(...)`) is ASCII and
/// ANSI-free; only str/bytes cells and headers are scanned.
pub fn detect_invisible(rows: &[Vec<InputCell>], headers: &[impl AsRef<str>]) -> bool {
    for h in headers {
        if ansi::contains_ansi_bytes(h.as_ref().as_bytes()) {
            return true;
        }
    }
    for row in rows {
        for ic in row {
            let bytes: &[u8] = match ic.cell {
                Cell::Str(s) => s.as_bytes(),
                Cell::Bytes(b) => b,
                _ => continue,
            };
            if ansi::contains_ansi_bytes(bytes) {
                return true;
            }
        }
    }
    false
}

/// `U:2383`: `_is_multiline(plain_text)`.
pub fn detect_multiline(rows: &[Vec<InputCell>], headers: &[impl AsRef<str>]) -> bool {
    for h in headers {
        if ansi::is_multiline_str(h.as_ref()) {
            return true;
        }
    }
    for row in rows {
        for ic in row {
            match ic.cell {
                Cell::Str(s) if ansi::is_multiline_str(s) => return true,
                Cell::Bytes(b) if ansi::is_multiline_bytes(b) => return true,
                _ => {}
            }
        }
    }
    false
}

/// `U:862 _isnumber_with_thousands_separator` on an already-decoded
/// `&str` (callers decode bytes first; undecodable bytes bail out
/// before reaching here so the `TypeError` surfaces via fallback).
/// Pattern: `^(([+-]?[0-9]{1,3})(?:,([0-9]{3}))*)?(?(1)\.[0-9]*|\.[0-9]+)?$`
pub fn is_number_with_thousands_separator(s: &str) -> bool {
    // Empty matches: group 1 absent + empty conditional group + `$`.
    if s.is_empty() {
        return true;
    }
    let b = s.as_bytes();
    let mut i = 0;
    let mut group1 = false;
    // Try group 1: [+-]?[0-9]{1,3} followed by (,ddd)*.
    let mut j = i;
    if j < b.len() && (b[j] == b'+' || b[j] == b'-') {
        j += 1;
    }
    let d0 = j;
    while j < b.len() && b[j].is_ascii_digit() && j - d0 < 3 {
        j += 1;
    }
    if j > d0 {
        // Must be followed by (,ddd)* then end-or-dot.
        let mut k = j;
        loop {
            if k < b.len() && b[k] == b',' {
                if k + 4 > b.len() {
                    break;
                }
                let g = &b[k + 1..k + 4];
                if g.iter().all(|c| c.is_ascii_digit()) {
                    k += 4;
                    continue;
                }
                break;
            }
            break;
        }
        // Valid iff what follows is end-of-string or a dot-fraction.
        // (Backtracking: if `,ddd` groups fail, plain `[0-9]{1,3}` still
        // counts as group 1.)
        if k == b.len() || (k < b.len() && b[k] == b'.') {
            // But a failed `,ddd` attempt means the comma must not be
            // consumed...: "1,00" — group1="1", then ",00" is not a
            // valid group, so k stays at 1 and b[1]==',' is neither end
            // nor dot -> group 1 does NOT match here. Fall through to
            // the no-group-1 branch only if the whole prefix is absent.
            // Upstream backtracking: group1="1" matches, then the
            // optional `\.[0-9]*` must match ",00..." -> fails, and the
            // overall `$` fails -> regex engine backtracks group1 to
            // absent -> `\.[0-9]+` must match "1,00" -> fails.
            // Net: only accept group1 when followed by end or dot.
            if k == b.len() || b[k] == b'.' {
                group1 = true;
                i = k;
            }
        }
        if !group1 {
            // Group 1 absent: the `[+-]?[0-9]{1,3}` prefix must not have
            // matched at all — i.e. this branch only applies when we
            // treat the whole numeric prefix as absent.
            return dot_fraction(b, 0);
        }
    }
    if group1 {
        // `(?(1)\.[0-9]*|\.[0-9]+)`: dot + zero or more digits, optional.
        if i < b.len() && b[i] == b'.' {
            i += 1;
            while i < b.len() && b[i].is_ascii_digit() {
                i += 1;
            }
        }
        i == b.len()
    } else {
        dot_fraction(b, 0)
    }
}

/// No-group-1 branch: `\.[0-9]+` must match at `start`, then end.
fn dot_fraction(b: &[u8], start: usize) -> bool {
    let mut i = start;
    if i < b.len() && b[i] == b'.' {
        i += 1;
        let d0 = i;
        while i < b.len() && b[i].is_ascii_digit() {
            i += 1;
        }
        i > d0 && i == b.len()
    } else {
        false
    }
}

/// ASCII-only fast check: all bytes are ASCII (used to pick the
/// hand-rolled parsers vs. delegating to CPython for exotic numerics).
fn is_ascii(s: &str) -> bool {
    s.is_ascii()
}

/// Python `int(s)` semantics on ASCII text: optional surrounding
/// whitespace, `[+-]?`, digits with single underscores between digits.
/// Returns the value or `None` if Python would raise.
pub fn parse_int_ascii(s: &str) -> Option<i64> {
    let t = ansi::strip(s);
    let b = t.as_bytes();
    if b.is_empty() {
        return None;
    }
    let mut i = 0;
    if b[0] == b'+' || b[0] == b'-' {
        i = 1;
    }
    let neg = b[0] == b'-';
    let mut v: i64 = 0;
    let mut ndigits = 0;
    let mut prev_underscore = true; // leading '_' invalid
    while i < b.len() {
        let c = b[i];
        if c.is_ascii_digit() {
            v = v.checked_mul(10)?.checked_add((c - b'0') as i64)?;
            ndigits += 1;
            prev_underscore = false;
        } else if c == b'_' && !prev_underscore {
            // underscore must be followed by a digit (checked next iter:
            // trailing '_' leaves prev_underscore=true -> reject below)
            prev_underscore = true;
        } else {
            return None;
        }
        i += 1;
    }
    if ndigits == 0 || prev_underscore {
        return None;
    }
    Some(if neg { -v } else { v })
}

/// What Python `float(s)` would do, classified without computing the
/// value in the common cases. ASCII inputs resolve fully in-core;
/// non-ASCII inputs must be delegated to CPython (unicode digits).
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FloatParse {
    Ascii(Option<f64>),
    NonAscii,
}

pub fn parse_float_ascii(s: &str) -> FloatParse {
    if !s.is_ascii() {
        return FloatParse::NonAscii;
    }
    let t = ansi::strip(s);
    if t.is_empty() {
        return FloatParse::Ascii(None);
    }
    let lower = t.to_ascii_lowercase();
    if let Some(v) = parse_inf_nan(&lower) {
        return FloatParse::Ascii(Some(v));
    }
    // General grammar: [+-]?(digits[.digits][e[+-]digits] | .digits[e...])
    // with single underscores between digits.
    match parse_float_digits(t) {
        Some(v) => FloatParse::Ascii(Some(v)),
        None => FloatParse::Ascii(None),
    }
}

fn parse_inf_nan(lower: &str) -> Option<f64> {
    let (neg, rest) = match lower.strip_prefix(['+', '-']) {
        Some(r) => (lower.starts_with('-'), r),
        None => (false, lower),
    };
    let v = match rest {
        "inf" | "infinity" => f64::INFINITY,
        "nan" => f64::NAN,
        _ => return None,
    };
    Some(if neg { -v } else { v })
}

/// Digits with single underscores strictly between digits.
/// Returns `(end_index, digit_count)`.
fn parse_num_run(b: &[u8], mut i: usize) -> (usize, usize) {
    let mut n = 0;
    let mut last_was_digit = false;
    while i < b.len() {
        if b[i].is_ascii_digit() {
            n += 1;
            last_was_digit = true;
            i += 1;
        } else if b[i] == b'_' && last_was_digit && i + 1 < b.len() && b[i + 1].is_ascii_digit() {
            last_was_digit = false;
            i += 1;
        } else {
            break;
        }
    }
    (i, n)
}

fn parse_float_digits(t: &str) -> Option<f64> {
    let b = t.as_bytes();
    let mut i = 0;
    if i < b.len() && (b[i] == b'+' || b[i] == b'-') {
        i += 1;
    }
    let (i2, int_digits) = parse_num_run(b, i);
    i = i2;
    let mut frac_digits = 0;
    if i < b.len() && b[i] == b'.' {
        i += 1;
        let (i3, f) = parse_num_run(b, i);
        i = i3;
        frac_digits = f;
    }
    if int_digits == 0 && frac_digits == 0 {
        return None;
    }
    if i < b.len() && (b[i] == b'e' || b[i] == b'E') {
        i += 1;
        if i < b.len() && (b[i] == b'+' || b[i] == b'-') {
            i += 1;
        }
        let (i4, exp_digits) = parse_num_run(b, i);
        i = i4;
        if exp_digits == 0 {
            return None;
        }
    }
    if i != b.len() {
        return None;
    }
    // Grammar validated; strip underscores (Rust's parser rejects them)
    // and compute with the standard parser (correctly rounded, handles
    // overflow to inf like CPython).
    t.replace('_', "").parse::<f64>().ok()
}

/// Adopt the `?` convention: caller's delegate hook for non-ASCII
/// numerics. The binding answers these two questions via CPython:
/// `int(x)` / `float(x)` success + value class. To keep the core
/// Python-free, non-ASCII cells use a tri-state answered by the
/// binding at extraction time... Instead, the core exposes the
/// question and the binding pre-resolves it: see `NumClass`.
pub fn is_ascii_text(s: &str) -> bool {
    is_ascii(s)
}

/// `U:955 _isint(string, inttype=int)` for an ASCII `&str` (the
/// `type(string) is inttype` and numpy branches are handled by the
/// binding; bytes are decoded to str before calling).
pub fn isint_str(s: &str) -> bool {
    parse_int_ascii(s).is_some()
}

/// `U:903 _isnumber(string)` for exact-type cells and ASCII strings.
/// `float_ok`/`float_val` carry the delegated CPython answer for
/// non-ASCII strings; `None` means "not delegated" (ASCII fast path).
pub fn isnumber_decided(
    is_float_or_int_type: bool,
    is_str_or_bytes: bool,
    float_ok: Option<bool>,
    float_is_inf_or_nan: bool,
    lowered_in_literal_list: bool,
) -> bool {
    if is_float_or_int_type {
        return true;
    }
    match float_ok {
        Some(ok) => ok && (!is_str_or_bytes || (!float_is_inf_or_nan || lowered_in_literal_list)),
        None => false,
    }
}

/// `U:974 _isbool`.
pub fn isbool_cell(cell: &Cell) -> bool {
    match cell {
        Cell::Bool(_) => true,
        Cell::Str(s) => *s == "True" || *s == "False",
        Cell::Bytes(b) => *b == b"True" || *b == b"False",
        _ => false,
    }
}

/// `U:1278 _more_generic`.
pub fn more_generic(t1: ColType, t2: ColType) -> ColType {
    fn rank(t: ColType) -> u8 {
        match t {
            ColType::None => 0,
            ColType::Bool => 1,
            ColType::Int => 2,
            ColType::Float => 3,
            ColType::Bytes => 4,
            ColType::Str => 5,
        }
    }
    fn unrank(r: u8) -> ColType {
        match r {
            0 => ColType::None,
            1 => ColType::Bool,
            2 => ColType::Int,
            3 => ColType::Float,
            4 => ColType::Bytes,
            _ => ColType::Str,
        }
    }
    unrank(rank(t1).max(rank(t2)))
}

/// The deduced type of one cell (`U:988 _type`).
/// `delegated` carries the CPython answers for non-ASCII strings:
/// `Some((int_ok, float_ok, float_is_inf_or_nan, lowered_literal_ok))`.
/// Returns `Err(Fallback)` for bytes that are not valid UTF-8 (upstream
/// raises `TypeError` from the regex; the fallback reproduces it).
pub fn cell_type(
    cell: Cell,
    has_invisible: bool,
    numparse: bool,
    delegated: Option<DelegatedNum>,
) -> Result<ColType, Fallback> {
    // `U:1009` strip ANSI first.
    // Borrowed form: work on a normalized view without allocating unless
    // ANSI is present.
    match cell {
        Cell::Null => return Ok(ColType::None),
        Cell::Bool(_) => return Ok(ColType::Bool),
        Cell::Int(_) => {
            // `U:1018`: _isint exact-int is true; bool already returned.
            if numparse {
                return Ok(ColType::Int);
            }
            // numparse=False: _isint still checked! `numparse and ...`
            // guards the whole int branch, so skip to float branch which
            // is also guarded... then bytes/str checks. An int with
            // numparse=False falls to `else: str`.
            return Ok(ColType::Str);
        }
        Cell::Float(_) => {
            if numparse {
                return Ok(ColType::Float);
            }
            return Ok(ColType::Str);
        }
        _ => {}
    }
    // Bytes never decode: `int()`/`float()` accept ASCII bytes and fail
    // everything else, and the thousands regex is str-only. The only
    // error is `_strip_ansi` on ANSI bytes (`TypeError` upstream).
    if let Cell::Bytes(b) = cell {
        if has_invisible && ansi::contains_ansi_bytes(b) {
            return Err(Fallback);
        }
        if b.is_empty() {
            return Ok(ColType::None);
        }
        if b == b"True" || b == b"False" {
            return Ok(ColType::Bool);
        }
        if numparse && b.is_ascii() {
            // Safe: ASCII bytes are valid UTF-8.
            let t = unsafe { std::str::from_utf8_unchecked(b) };
            if parse_int_ascii(t).is_some() {
                return Ok(ColType::Int);
            }
            match parse_float_ascii(t) {
                FloatParse::Ascii(Some(v)) if !(v.is_infinite() || v.is_nan()) => {
                    return Ok(ColType::Float);
                }
                _ => {}
            }
            // Overflow exclusion needs no literal check: bytes never
            // match the `str` literal list (`U:948`).
        }
        return Ok(ColType::Bytes);
    }
    // Str below.
    let s: &str = match cell {
        Cell::Str(s) => s,
        _ => unreachable!(),
    };
    let stripped: std::borrow::Cow<str> =
        if has_invisible && ansi::contains_ansi_bytes(s.as_bytes()) {
            std::borrow::Cow::Owned(ansi::strip_ansi(s))
        } else {
            std::borrow::Cow::Borrowed(s)
        };
    let st = stripped.as_ref();
    // `U:1012` empty/missing.
    if st.is_empty() {
        return Ok(ColType::None);
    }
    // `U:1016` bool check (exact-bool handled; "True"/"False" strings).
    if st == "True" || st == "False" {
        return Ok(ColType::Bool);
    }
    if numparse {
        // `U:1018` int branch (str only; bytes returned above).
        let int_ok = if st.is_ascii() {
            parse_int_ascii(st).is_some()
        } else {
            delegated.map(|d| d.int_ok).unwrap_or(false)
        };
        let thousands_int = is_number_with_thousands_separator(st) && !st.contains('.');
        if int_ok || thousands_int {
            return Ok(ColType::Int);
        }
        // `U:1027` float branch.
        let float_ok: Option<(bool, bool)> = if st.is_ascii() {
            match parse_float_ascii(st) {
                FloatParse::Ascii(Some(v)) => Some((true, v.is_infinite() || v.is_nan())),
                FloatParse::Ascii(None) => Some((false, false)),
                FloatParse::NonAscii => None,
            }
        } else {
            delegated.map(|d| (d.float_ok, d.float_is_inf_or_nan))
        };
        let thousands_float = is_number_with_thousands_separator(st);
        if let Some((ok, infnan)) = float_ok {
            if ok {
                if !infnan {
                    return Ok(ColType::Float);
                }
                // Overflow exclusion (`U:925`): infinities only count for
                // the literal spellings "inf", "-inf", "nan".
                let lowered = st.to_ascii_lowercase();
                if lowered == "inf" || lowered == "-inf" || lowered == "nan" {
                    return Ok(ColType::Float);
                }
            }
        }
        // Non-ASCII with no delegation answer: the caller must have
        // delegated; unreachable if the binding does its job.
        if thousands_float {
            return Ok(ColType::Float);
        }
    }
    Ok(ColType::Str)
}

/// CPython answers for a non-ASCII string cell, pre-resolved by the
/// binding with real `int()`/`float()` calls (exact by construction).
/// The thousands-separator regex is pure ASCII and computed in-core.
#[derive(Debug, Clone, Copy)]
pub struct DelegatedNum {
    pub int_ok: bool,
    pub float_ok: bool,
    pub float_is_inf_or_nan: bool,
}

/// `U:1038 _afterpoint`: symbols after a decimal point, -1 if none.
/// ASCII-only: the aligner bails out before calling this on non-ASCII
/// (byte and char counts then coincide).
pub fn afterpoint_str(s: &str) -> Result<i64, Fallback> {
    if !s.is_ascii() {
        return Err(Fallback);
    }
    let fio = match parse_float_ascii(s) {
        // ASCII checked above; NonAscii unreachable.
        FloatParse::Ascii(v) => v,
        FloatParse::NonAscii => return Err(Fallback),
    };
    let is_int = parse_int_ascii(s).is_some();
    let is_num = fio.is_some() || is_number_with_thousands_separator(s);
    if !is_num {
        return Ok(-1);
    }
    if is_int {
        return Ok(-1);
    }
    let pos = s.rfind('.');
    let pos = match pos {
        Some(p) => p as i64,
        None => s.to_lowercase().rfind('e').map(|p| p as i64).unwrap_or(-1),
    };
    if pos >= 0 {
        Ok(s.len() as i64 - pos - 1)
    } else {
        Ok(-1)
    }
}

/// Python `str(bytes)` repr (`U:1363-1367`, `U:1383`): ASCII decodes
/// directly, otherwise the `b'...'` repr with `\x` escapes.
pub fn bytes_repr(b: &[u8]) -> String {
    if b.is_ascii() {
        // `str(val, "ascii")` succeeds for pure ASCII.
        return String::from_utf8_lossy(b).into_owned();
    }
    let mut out = String::with_capacity(b.len() + 2);
    out.push_str("b'");
    for &c in b {
        match c {
            b'\'' => out.push_str("\\'"),
            b'\\' => out.push_str("\\\\"),
            b'\n' => out.push_str("\\n"),
            b'\r' => out.push_str("\\r"),
            b'\t' => out.push_str("\\t"),
            0x20..=0x7e => out.push(c as char),
            _ => {
                out.push_str(&format!("\\x{:02x}", c));
            }
        }
    }
    out.push('\'');
    out
}
/// Python `str(True)` == `"True"` (Rust `Display` gives lowercase).
fn bool_str(v: bool) -> &'static str {
    if v {
        "True"
    } else {
        "False"
    }
}
/// `format(b"...", "")` == `str(bytes)`: always the quoted `b'...'` repr
/// (unlike the bytes-column path, which decodes pure ASCII).
pub fn bytes_str_empty(b: &[u8]) -> String {
    let mut out = String::with_capacity(b.len() + 2);
    out.push_str("b'");
    for &c in b {
        match c {
            b'\'' => out.push_str("\\'"),
            b'\\' => out.push_str("\\\\"),
            b'\n' => out.push_str("\\n"),
            b'\r' => out.push_str("\\r"),
            b'\t' => out.push_str("\\t"),
            0x20..=0x7e => out.push(c as char),
            _ => {
                out.push_str(&format!("\\x{:02x}", c));
            }
        }
    }
    out.push('\'');
    out
}
/// `U:1345-1361`: format an ANSI-colored string in an int column.
/// `intfmt` is the *user* spec (the dance applies it to the digits).
pub fn colored_int_format(s: &str, int_spec: &numfmt::Spec) -> Result<String, Fallback> {
    let escaped = unicode_escape(s);
    if let Some((g1, digits, g3)) = match_colored_int(&escaped) {
        if !digits.is_empty() && digits.bytes().all(|c| c.is_ascii_digit()) {
            let n: i64 = digits.parse().map_err(|_| Fallback)?;
            let f = numfmt::format_int(n, int_spec)?;
            return unicode_unescape(&format!("{g1}{f}{g3}"));
        }
    }
    Ok(s.to_string())
}

/// `U:1369-1376`: format an ANSI-colored string in a float column.
pub fn colored_float_format(s: &str, float_spec: &numfmt::Spec) -> Result<String, Fallback> {
    let raw = ansi::strip_ansi(s);
    let v = match parse_float_ascii(&raw) {
        FloatParse::Ascii(Some(v)) => v,
        FloatParse::Ascii(None) => return Ok(s.to_string()),
        FloatParse::NonAscii => return Err(Fallback),
    };
    let formatted = numfmt::format_float(v, float_spec)?;
    Ok(s.replace(raw.as_str(), &formatted))
}

/// Python `str.encode('unicode_escape').decode()`: `\\t\\n\\r\\\\\\'`,
/// `\\xhh`, `\\uxxxx`, `\\Uxxxxxxxx`; everything else printable ASCII
/// passes through (incl. `"`).
pub fn unicode_escape(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '\t' => out.push_str("\\t"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\\' => out.push_str("\\\\"),
            '\'' => out.push_str("\\'"),
            '\0'..='\u{1f}' | '\u{7f}' => {
                out.push_str(&format!("\\x{:02x}", c as u32));
            }
            '\u{80}'..='\u{ff}' => {
                out.push_str(&format!("\\x{:02x}", c as u32));
            }
            '\u{100}'..='\u{ffff}' => {
                out.push_str(&format!("\\u{:04x}", c as u32));
            }
            c if c as u32 >= 0x10000 => {
                out.push_str(&format!("\\U{:08x}", c as u32));
            }
            _ => out.push(c),
        }
    }
    out
}

/// Reverse of [`unicode_escape`] (plus `\"`, `\\a\\b\\f\\v`, octal).
pub fn unicode_unescape(s: &str) -> Result<String, Fallback> {
    let b = s.as_bytes();
    let mut out = String::with_capacity(s.len());
    let mut i = 0;
    while i < b.len() {
        if b[i] != b'\\' {
            out.push(s[i..].chars().next().ok_or(Fallback)?);
            i += s[i..].chars().next().map(|c| c.len_utf8()).unwrap_or(1);
            continue;
        }
        i += 1;
        if i >= b.len() {
            return Err(Fallback);
        }
        match b[i] {
            b'n' => out.push('\n'),
            b'r' => out.push('\r'),
            b't' => out.push('\t'),
            b'\\' => out.push('\\'),
            b'\'' => out.push('\''),
            b'"' => out.push('"'),
            b'a' => out.push('\x07'),
            b'b' => out.push('\x08'),
            b'f' => out.push('\x0c'),
            b'v' => out.push('\x0b'),
            b'x' => {
                if i + 2 >= b.len() {
                    return Err(Fallback);
                }
                let h = std::str::from_utf8(&b[i + 1..i + 3]).map_err(|_| Fallback)?;
                let v = u32::from_str_radix(h, 16).map_err(|_| Fallback)?;
                out.push(char::from_u32(v).ok_or(Fallback)?);
                i += 2;
            }
            b'u' => {
                if i + 4 >= b.len() {
                    return Err(Fallback);
                }
                let h = std::str::from_utf8(&b[i + 1..i + 5]).map_err(|_| Fallback)?;
                let v = u32::from_str_radix(h, 16).map_err(|_| Fallback)?;
                out.push(char::from_u32(v).ok_or(Fallback)?);
                i += 4;
            }
            b'U' => {
                if i + 8 >= b.len() {
                    return Err(Fallback);
                }
                let h = std::str::from_utf8(&b[i + 1..i + 9]).map_err(|_| Fallback)?;
                let v = u32::from_str_radix(h, 16).map_err(|_| Fallback)?;
                out.push(char::from_u32(v).ok_or(Fallback)?);
                i += 8;
            }
            b'0'..=b'7' => {
                let mut j = i;
                while j < b.len() && j - i < 3 && matches!(b[j], b'0'..=b'7') {
                    j += 1;
                }
                let h = std::str::from_utf8(&b[i..j]).map_err(|_| Fallback)?;
                let v = u32::from_str_radix(h, 8).map_err(|_| Fallback)?;
                out.push(char::from_u32(v).ok_or(Fallback)?);
                i = j - 1;
            }
            _ => return Err(Fallback),
        }
        i += 1;
    }
    Ok(out)
}

/// Leftmost match of `U:1348`
/// `(\\[xX]+[0-9a-fA-F]+\[\d+[mM]+)([0-9.]+)(\\.*)$` on escaped text.
/// All runs are forced (no backtracking ambiguity), so a single
/// left-to-right scan is exact.
pub fn match_colored_int(e: &str) -> Option<(String, String, String)> {
    let b = e.as_bytes();
    let mut p = 0;
    while p < b.len() {
        if b[p] == b'\\' {
            let mut q = p + 1;
            let x0 = q;
            while q < b.len() && (b[q] == b'x' || b[q] == b'X') {
                q += 1;
            }
            if q == x0 {
                p += 1;
                continue;
            }
            let h0 = q;
            while q < b.len() && b[q].is_ascii_hexdigit() {
                q += 1;
            }
            if q == h0 || q >= b.len() || b[q] != b'[' {
                p += 1;
                continue;
            }
            q += 1;
            let d0 = q;
            while q < b.len() && b[q].is_ascii_digit() {
                q += 1;
            }
            if q == d0 || q >= b.len() || (b[q] != b'm' && b[q] != b'M') {
                p += 1;
                continue;
            }
            q += 1;
            let g1end = q;
            let n0 = q;
            while q < b.len() && (b[q].is_ascii_digit() || b[q] == b'.') {
                q += 1;
            }
            if q == n0 || q >= b.len() || b[q] != b'\\' {
                p += 1;
                continue;
            }
            return Some((
                e[p..g1end].to_string(),
                e[g1end..q].to_string(),
                e[q..].to_string(),
            ));
        }
        p += 1;
    }
    None
}

/// `U:1325 _format`: format one cell by its column type.
#[allow(clippy::too_many_arguments)]
pub fn format_cell(
    cell: Cell,
    valtype: ColType,
    float_spec: &numfmt::Spec,
    int_spec: &numfmt::Spec,
    missingval: &str,
    has_invisible: bool,
) -> Result<String, Fallback> {
    if matches!(cell, Cell::Null) {
        return Ok(missingval.to_string());
    }
    if let Cell::Str(s) = cell {
        if s.is_empty() {
            return Ok(String::new());
        }
    }
    if let Cell::Bytes(b) = cell {
        if b.is_empty() {
            return Ok(String::new());
        }
    }
    match valtype {
        ColType::Str => Ok(match cell {
            Cell::Str(s) => s.to_string(),
            // `U:1343 f"{val}"`: bytes/int/float/bool via str().
            Cell::Bytes(b) => bytes_repr(b),
            Cell::Int(v) => v.to_string(),
            Cell::Float(v) => numfmt::float_repr(v),
            Cell::Bool(v) => bool_str(v).to_string(),
            Cell::Null => unreachable!(),
        }),
        ColType::Int => match cell {
            Cell::Int(v) => Ok(numfmt::format_int(v, int_spec)?),
            Cell::Bool(v) => Ok(numfmt::format_int(i64::from(v), int_spec)?),
            Cell::Str(s) => {
                // `U:1345-1361`: ANSI-colored dance; `intfmt` is forced
                // to `""` for str cells (`U:1361`).
                if has_invisible && ansi::contains_ansi_bytes(s.as_bytes()) {
                    return colored_int_format(s, int_spec);
                }
                Ok(s.to_string())
            }
            Cell::Bytes(b) => {
                if has_invisible && ansi::contains_ansi_bytes(b) {
                    return Err(Fallback);
                }
                // `format(b"...", intfmt)`: the spec is NOT reset for
                // bytes; non-empty specs raise TypeError upstream.
                if *int_spec == numfmt::Spec::default_int() {
                    Ok(bytes_str_empty(b))
                } else {
                    Err(Fallback)
                }
            }
            Cell::Float(v) => {
                // Unreachable via _column_type (floats sniff as float),
                // but harmless: mirror `format(int_val, intfmt)`.
                let _ = v;
                Err(Fallback)
            }
            Cell::Null => unreachable!(),
        },
        ColType::Bytes => Ok(match cell {
            // `U:1363-1367`: `str(val, "ascii")` raises TypeError for
            // non-bytes (caught) and UnicodeDecodeError for non-ASCII
            // bytes (caught); both fall back to `str(val)`.
            Cell::Bytes(b) => bytes_repr(b),
            Cell::Str(s) => s.to_string(),
            Cell::Int(v) => v.to_string(),
            Cell::Float(v) => numfmt::float_repr(v),
            Cell::Bool(v) => bool_str(v).to_string(),
            Cell::Null => unreachable!(),
        }),
        ColType::Float => {
            // Resolve the numeric value.
            let v: f64 = match cell {
                Cell::Float(v) => v,
                Cell::Int(v) => v as f64,
                Cell::Bool(v) => f64::from(v),
                Cell::Str(s) => {
                    // `U:1369-1376`: ANSI-colored numbers format the
                    // stripped value, then splice it back in.
                    if has_invisible && ansi::contains_ansi_bytes(s.as_bytes()) {
                        return colored_float_format(s, float_spec);
                    }
                    let t = if s.contains(',') {
                        // `U:1378` thousands separators.
                        std::borrow::Cow::Owned(s.replace(',', ""))
                    } else {
                        std::borrow::Cow::Borrowed(s)
                    };
                    match parse_float_ascii(&t) {
                        FloatParse::Ascii(Some(v)) => v,
                        // `U:1380-1383`: conversion failure -> f"{val}".
                        FloatParse::Ascii(None) => return Ok(s.to_string()),
                        FloatParse::NonAscii => return Err(Fallback),
                    }
                }
                Cell::Bytes(b) => {
                    if has_invisible && ansi::contains_ansi_bytes(b) {
                        return Err(Fallback);
                    }
                    // `float(bytes)` only accepts ASCII; anything else
                    // raises, and upstream falls back to `f"{val}"`.
                    let t = match std::str::from_utf8(b) {
                        Ok(t) => t,
                        Err(_) => return Ok(bytes_repr(b)),
                    };
                    let t = if t.contains(',') {
                        std::borrow::Cow::Owned(t.replace(',', ""))
                    } else {
                        std::borrow::Cow::Borrowed(t)
                    };
                    match parse_float_ascii(&t) {
                        FloatParse::Ascii(Some(v)) => v,
                        FloatParse::Ascii(None) => return Ok(t.into_owned()),
                        // Non-ASCII `str` decoded from bytes: `float()`
                        // raises -> `f"{val}"` is the quoted repr.
                        FloatParse::NonAscii => return Ok(bytes_repr(b)),
                    }
                }
                Cell::Null => unreachable!(),
            };
            Ok(numfmt::format_float(v, float_spec)?)
        }
        ColType::Bool | ColType::None => Ok(match cell {
            Cell::Bool(v) => bool_str(v).to_string(),
            Cell::Str(s) => s.to_string(),
            Cell::Bytes(b) => bytes_repr(b),
            Cell::Int(v) => v.to_string(),
            Cell::Float(v) => numfmt::float_repr(v),
            Cell::Null => missingval.to_string(),
        }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn thousands_separator_vectors() {
        // Doctests from `U:862`.
        for s in [
            "1",
            "1.",
            ".1",
            "1,000",
            "1,000.1234",
            "+1,000.1234",
            "-1,000.1234",
            "",
        ] {
            assert!(is_number_with_thousands_separator(s), "{s}");
        }
        for s in [".", "1000", "1,0000", "12,34", "1234,567"] {
            assert!(!is_number_with_thousands_separator(s), "{s}");
        }
    }

    #[test]
    fn int_grammar() {
        assert_eq!(parse_int_ascii("123"), Some(123));
        assert_eq!(parse_int_ascii("  -12 "), Some(-12));
        assert_eq!(parse_int_ascii("1_0"), Some(10));
        assert_eq!(parse_int_ascii("123.45"), None);
        assert_eq!(parse_int_ascii(""), None);
        assert_eq!(parse_int_ascii("1__0"), None);
        assert_eq!(parse_int_ascii("1_"), None);
        assert_eq!(parse_int_ascii("0x10"), None);
        assert_eq!(parse_int_ascii("+"), None);
    }

    #[test]
    fn float_grammar() {
        assert!(matches!(
            parse_float_ascii("123.45"),
            FloatParse::Ascii(Some(_))
        ));
        assert_eq!(parse_float_ascii("123"), FloatParse::Ascii(Some(123.0)));
        assert_eq!(parse_float_ascii("1e2"), FloatParse::Ascii(Some(100.0)));
        assert_eq!(
            parse_float_ascii("inf"),
            FloatParse::Ascii(Some(f64::INFINITY))
        );
        assert_eq!(
            parse_float_ascii("-INF"),
            FloatParse::Ascii(Some(f64::NEG_INFINITY))
        );
        assert!(matches!(parse_float_ascii("nan"), FloatParse::Ascii(Some(v)) if v.is_nan()));
        assert_eq!(parse_float_ascii("spam"), FloatParse::Ascii(None));
        assert_eq!(parse_float_ascii(""), FloatParse::Ascii(None));
        assert_eq!(parse_float_ascii("1.2.3"), FloatParse::Ascii(None));
        assert_eq!(parse_float_ascii("1."), FloatParse::Ascii(Some(1.0)));
        assert_eq!(parse_float_ascii("1_0"), FloatParse::Ascii(Some(10.0)));
        assert_eq!(parse_float_ascii("1_0.5"), FloatParse::Ascii(Some(10.5)));
        assert_eq!(parse_float_ascii("1_"), FloatParse::Ascii(None));
        assert_eq!(parse_float_ascii("1__0"), FloatParse::Ascii(None));
        assert_eq!(parse_float_ascii("1_.5"), FloatParse::Ascii(None));
        assert_eq!(parse_float_ascii("1.5_"), FloatParse::Ascii(None));
        assert!(matches!(parse_float_ascii("héllo"), FloatParse::NonAscii));
    }

    #[test]
    fn afterpoint_vectors() {
        // Doctests from `U:1038`.
        assert_eq!(afterpoint_str("123.45"), Ok(2));
        assert_eq!(afterpoint_str("1001"), Ok(-1));
        assert_eq!(afterpoint_str("eggs"), Ok(-1));
        assert_eq!(afterpoint_str("123e45"), Ok(2));
        assert_eq!(afterpoint_str("123,456.78"), Ok(2));
    }

    #[test]
    fn bytes_repr_vectors() {
        assert_eq!(bytes_repr(b"abc"), "abc");
        assert_eq!(bytes_repr(b"\xff\x00"), "b'\\xff\\x00'");
        assert_eq!(bytes_repr(b"a'b"), "a'b");
        assert_eq!(bytes_str_empty(b"a'b"), "b'a\\'b'");
        assert_eq!(bytes_str_empty(b"ab"), "b'ab'");
    }
}
