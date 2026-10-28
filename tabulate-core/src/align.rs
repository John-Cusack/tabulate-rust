//! Column and header alignment.
//! Ports `U:1067` (`_padleft`), `U:1078` (`_padright`), `U:1089`
//! (`_padboth`), `U:1100` (`_padnone`), `U:1169`
//! (`_align_column_choose_padfn`), `U:1195`
//! (`_align_column_choose_width_fn` via [`WidthMode`]), `U:1226`
//! (`_align_column`), and `U:1388` (`_align_header`).

use crate::ansi;
use crate::sniff::afterpoint_str;
use crate::width::WidthMode;
use crate::{Align, Fallback};

fn char_len(s: &str) -> usize {
    s.chars().count()
}

/// `U:1067 _padleft`: flush right (no truncation).
pub fn pad_left(width: usize, s: &str) -> String {
    let n = char_len(s);
    if width <= n {
        return s.to_string();
    }
    format!("{}{}", " ".repeat(width - n), s)
}

/// `U:1078 _padright`: flush left.
pub fn pad_right(width: usize, s: &str) -> String {
    let n = char_len(s);
    if width <= n {
        return s.to_string();
    }
    format!("{}{}", s, " ".repeat(width - n))
}

/// `U:1089 _padboth`: centered (extra space goes right, like
/// `"{:^w}".format`).
pub fn pad_both(width: usize, s: &str) -> String {
    let n = char_len(s);
    if width <= n {
        return s.to_string();
    }
    let total = width - n;
    let left = total / 2;
    let right = total - left;
    format!("{}{}{}", " ".repeat(left), s, " ".repeat(right))
}

fn pad_fn_for(alignment: &Align) -> fn(usize, &str) -> String {
    match alignment {
        Align::Right | Align::Decimal => pad_left,
        Align::Center => pad_both,
        Align::None_ => pad_none,
        _ => pad_right,
    }
}

/// `U:1100 _padnone`.
fn pad_none(_width: usize, s: &str) -> String {
    s.to_string()
}

/// `U:1226 _align_column`: pad one column of display strings.
pub fn align_column(
    strings: &[String],
    alignment: &Align,
    minwidth: i64,
    width_mode: &WidthMode,
    preserve_whitespace: bool,
) -> Result<Vec<String>, Fallback> {
    // `U:1169` pad-function choice (with the decimal preparation).
    let prepared: Vec<String>;
    let padfn: fn(usize, &str) -> String;
    match alignment {
        Align::Decimal => {
            // `U:1180`: decimals from ANSI-stripped strings when invisible.
            let mut decimals = Vec::with_capacity(strings.len());
            for s in strings {
                let t = if width_mode.has_invisible {
                    ansi::strip_ansi(s)
                } else {
                    s.clone()
                };
                decimals.push(afterpoint_str(&t)?);
            }
            let maxdecimals = decimals.iter().copied().max().unwrap_or(-1);
            prepared = strings
                .iter()
                .zip(decimals.iter())
                .map(|(s, d)| format!("{s}{}", " ".repeat((maxdecimals - d).max(0) as usize)))
                .collect();
            padfn = pad_left;
        }
        Align::None_ => {
            prepared = strings.to_vec();
            padfn = pad_none;
        }
        other => {
            if preserve_whitespace {
                prepared = strings.to_vec();
            } else {
                prepared = strings.iter().map(|s| ansi::strip(s).to_string()).collect();
            }
            padfn = pad_fn_for(other);
        }
    }

    if width_mode.multiline {
        align_multiline(&prepared, padfn, minwidth, width_mode)
    } else {
        align_single(&prepared, padfn, minwidth, width_mode)
    }
}

fn align_single(
    strings: &[String],
    padfn: fn(usize, &str) -> String,
    minwidth: i64,
    width_mode: &WidthMode,
) -> Result<Vec<String>, Fallback> {
    let mut s_widths = Vec::with_capacity(strings.len());
    for s in strings {
        s_widths.push(width_mode.width(s));
    }
    let maxwidth = s_widths.iter().copied().max().unwrap_or(0).max(minwidth);
    // `maxwidth < 0` needs `minwidth < 0`, which the kernel rejects
    // before aligning (the fallback then replays any ValueError).
    debug_assert!(maxwidth >= 0);
    let maxwidth = maxwidth.max(0) as usize;
    if !width_mode.has_invisible && !width_mode.wide {
        return Ok(strings.iter().map(|s| padfn(maxwidth, s)).collect());
    }
    // Wide/invisible width corrections (`U:1269`).
    let mut out = Vec::with_capacity(strings.len());
    for (s, w) in strings.iter().zip(s_widths.iter()) {
        let l = char_len(s) as i64;
        let vw = maxwidth as i64 - (*w - l);
        debug_assert!(vw >= 0);
        out.push(padfn(vw.max(0) as usize, s));
    }
    Ok(out)
}

fn align_multiline(
    strings: &[String],
    padfn: fn(usize, &str) -> String,
    minwidth: i64,
    width_mode: &WidthMode,
) -> Result<Vec<String>, Fallback> {
    // Per-line widths (`U:1211` list form).
    let mut s_widths: Vec<Vec<i64>> = Vec::with_capacity(strings.len());
    for s in strings {
        s_widths.push(width_mode.widths(s));
    }
    let maxwidth = s_widths
        .iter()
        .flat_map(|v| v.iter().copied())
        .max()
        .unwrap_or(0)
        .max(minwidth);
    debug_assert!(maxwidth >= 0);
    let maxwidth = maxwidth.max(0) as usize;
    if !width_mode.has_invisible && !width_mode.wide {
        // `U:1247`: pad every physical line to maxwidth.
        return Ok(strings
            .iter()
            .map(|ms| {
                ansi::splitlines_py(ms)
                    .iter()
                    .map(|s| padfn(maxwidth, s))
                    .collect::<Vec<_>>()
                    .join("\n")
            })
            .collect());
    }
    // Wide/invisible per-line corrections (`U:1253`).
    let mut out = Vec::with_capacity(strings.len());
    for (ms, mw) in strings.iter().zip(s_widths.iter()) {
        // `U:1254`: physical lengths via [\r\n] split.
        let ml: Vec<usize> = ansi::split_lines(ms).iter().map(|s| char_len(s)).collect();
        let vw: Vec<usize> = mw
            .iter()
            .zip(ml.iter())
            .map(|(w, l)| (maxwidth as i64 - (*w - *l as i64)).max(0) as usize)
            .collect();
        // `(ms.splitlines() or ms)`: empty stays empty.
        let lines = ansi::splitlines_py(ms);
        let joined = if lines.is_empty() {
            String::new()
        } else {
            lines
                .iter()
                .zip(vw.iter())
                .map(|(s, w)| padfn(*w, s))
                .collect::<Vec<_>>()
                .join("\n")
        };
        out.push(joined);
    }
    Ok(out)
}

/// `U:1388 _align_header`.
pub fn align_header(
    header: &str,
    alignment: &Align,
    width: i64,
    visible_width: i64,
    is_multiline: bool,
    width_mode: &WidthMode,
) -> Result<String, Fallback> {
    if is_multiline {
        // `U:1393`: re.split(multiline_codes) == per-char [\r\n] split.
        let mut out = Vec::new();
        for h in ansi::split_lines(header) {
            out.push(align_header(
                h,
                alignment,
                width,
                width_mode.width(h),
                false,
                width_mode,
            )?);
        }
        return Ok(out.join("\n"));
    }
    // `U:1399`: `ninvisible` can be negative (wide chars have fewer
    // chars than visible width); the width absorbs the difference, and
    // stays non-negative because the column width anchors on this
    // header (`minwidth = width + pad`).
    let width = width + char_len(header) as i64 - visible_width;
    debug_assert!(width >= 0);
    let width = width.max(0) as usize;
    Ok(match alignment {
        Align::Left => pad_right(width, header),
        Align::Center => pad_both(width, header),
        Align::None_ => header.to_string(),
        _ => pad_left(width, header),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn plain() -> WidthMode {
        WidthMode::choose(false, false, false).unwrap()
    }

    #[test]
    fn pads() {
        assert_eq!(pad_left(5, "ab"), "   ab");
        assert_eq!(pad_right(5, "ab"), "ab   ");
        assert_eq!(pad_both(5, "ab"), " ab  ");
        assert_eq!(pad_left(1, "ab"), "ab");
    }

    #[test]
    fn column_left() {
        let m = plain();
        let out = align_column(
            &["a".to_string(), "bb".to_string()],
            &Align::Left,
            0,
            &m,
            false,
        )
        .unwrap();
        assert_eq!(out, vec!["a ".to_string(), "bb".to_string()]);
    }

    #[test]
    fn column_decimal() {
        let m = plain();
        let out = align_column(
            &["1.5".to_string(), "12.25".to_string(), "x".to_string()],
            &Align::Decimal,
            0,
            &m,
            false,
        )
        .unwrap();
        assert_eq!(
            out,
            vec![
                " 1.5 ".to_string(),
                "12.25".to_string(),
                " x   ".to_string()
            ]
        );
    }

    #[test]
    fn header_center() {
        let m = plain();
        assert_eq!(
            align_header("ab", &Align::Center, 5, 2, false, &m).unwrap(),
            " ab  "
        );
    }
}
