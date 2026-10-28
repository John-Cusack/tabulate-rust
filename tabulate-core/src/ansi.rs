//! ANSI escape handling, multiline detection, and string helpers.
//! Ports `U:787-834` (`_ansi_codes`), `U:1104` (`_strip_ansi`),
//! `U:1142` (`_is_multiline`), `U:784` (`_multiline_codes`),
//! `U:1211`/`U:1149` width-of-multiline helpers, and Python
//! `str.strip` semantics used by the aligners.

use crate::Fallback;

/// Python `str.isspace` over the full range, dumped from CPython 3.12:
/// `strip()`/`isspace()` treat exactly these code points as whitespace.
pub fn is_space(c: char) -> bool {
    matches!(
        c as u32,
        9..=13 | 28..=32 | 133 | 160 | 5760 | 8192..=8202 | 8232..=8233 | 8239 | 8287 | 12288
    )
}

/// Strip ASCII/Unicode whitespace from both ends (`str.strip()`).
pub fn strip(s: &str) -> &str {
    let b = s.as_bytes();
    let mut start = 0;
    let mut end = b.len();
    while start < end {
        let c = s[start..].chars().next().unwrap();
        if is_space(c) {
            start += c.len_utf8();
        } else {
            break;
        }
    }
    while end > start {
        let c = s[..end].chars().next_back().unwrap();
        if is_space(c) {
            end -= c.len_utf8();
        } else {
            break;
        }
    }
    &s[start..end]
}

/// Strip trailing whitespace (`str.rstrip()`).
pub fn rstrip(s: &str) -> &str {
    let mut end = s.len();
    while end > 0 {
        let c = s[..end].chars().next_back().unwrap();
        if is_space(c) {
            end -= c.len_utf8();
        } else {
            break;
        }
    }
    &s[..end]
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum AnsiMatch {
    /// CSI sequence: removed entirely.
    Csi { start: usize, end: usize },
    /// OSC-8 hyperlink: replaced by the link text.
    Hyperlink {
        start: usize,
        end: usize,
        text_start: usize,
        text_end: usize,
    },
}

/// Scan for the next ANSI match at or after `pos` (byte offsets).
/// Grammar (`U:790-832`): CSI `ESC [` params `[0x30-0x3f]*`
/// intermediates `[0x20-0x2f]*` final `[0x40-0x7e]`; OSC-8 hyperlink
/// `ESC ] 8 ; params ; URI ST TEXT ESC ] 8 ;; ST` with `ST = ESC \`.
fn next_ansi(bytes: &[u8], pos: usize) -> Option<AnsiMatch> {
    let mut i = pos;
    while i < bytes.len() {
        if bytes[i] != 0x1b {
            i += 1;
            continue;
        }
        if i + 1 >= bytes.len() {
            return None;
        }
        match bytes[i + 1] {
            // CSI
            b'[' => {
                let mut j = i + 2;
                while j < bytes.len() && (0x30..=0x3f).contains(&bytes[j]) {
                    j += 1;
                }
                while j < bytes.len() && (0x20..=0x2f).contains(&bytes[j]) {
                    j += 1;
                }
                if j < bytes.len() && (0x40..=0x7e).contains(&bytes[j]) {
                    return Some(AnsiMatch::Csi {
                        start: i,
                        end: j + 1,
                    });
                }
                // Not a valid CSI: the ESC is literal text; skip past it.
                // (Python `re` would try the next start position, so we
                // advance by one and keep scanning.)
                i += 1;
            }
            // OSC: only `8;...` hyperlinks are special, everything else
            // is literal text for this pattern.
            b']' => {
                // Need `]8;`
                if i + 3 < bytes.len() && bytes[i + 2] == b'8' && bytes[i + 3] == b';' {
                    if let Some(m) = match_hyperlink(bytes, i) {
                        return Some(m);
                    }
                }
                i += 1;
            }
            _ => {
                i += 1;
            }
        }
    }
    None
}

/// Try to match an OSC-8 hyperlink starting at `i`
/// (`bytes[i] == ESC`, `bytes[i+1] == ']'`).
fn match_hyperlink(bytes: &[u8], i: usize) -> Option<AnsiMatch> {
    // `ESC ] 8 ;` consumed; params `(\w+=\w+:?)*`, `;`, URI (no ESC),
    // ST, TEXT (no ESC), `ESC ] 8 ;; ST`.
    let mut j = i + 4; // past `ESC]8;`
    loop {
        if j >= bytes.len() || bytes[j] == 0x1b {
            return None;
        }
        if bytes[j] == b';' {
            j += 1;
            break;
        }
        // param `key=value` with word chars, optional trailing ':'
        let ks = j;
        while j < bytes.len() && is_word(bytes[j]) {
            j += 1;
        }
        if j == ks || j >= bytes.len() || bytes[j] != b'=' {
            return None;
        }
        j += 1;
        let vs = j;
        while j < bytes.len() && is_word(bytes[j]) {
            j += 1;
        }
        if j == vs {
            return None;
        }
        if j < bytes.len() && bytes[j] == b':' {
            j += 1;
        }
    }
    // URI: anything but ESC, then ST.
    let mut k = j;
    while k < bytes.len() && bytes[k] != 0x1b {
        k += 1;
    }
    if k + 1 >= bytes.len() || bytes[k + 1] != b'\\' {
        return None;
    }
    // TEXT: anything but ESC.
    let text_start = k + 2;
    let mut t = text_start;
    while t < bytes.len() && bytes[t] != 0x1b {
        t += 1;
    }
    let text_end = t;
    // Closing `ESC ] 8 ;; ST`.
    let close: &[u8] = b"\x1b]8;;\x1b\\";
    if t + close.len() > bytes.len() || &bytes[t..t + close.len()] != close {
        return None;
    }
    Some(AnsiMatch::Hyperlink {
        start: i,
        end: t + close.len(),
        text_start,
        text_end,
    })
}

fn is_word(b: u8) -> bool {
    b.is_ascii_alphanumeric() || b == b'_'
}

/// `_strip_ansi` (`U:1104`): CSI sequences are removed (the `re.sub`
/// replacement `r"\4"` is group 4, which is empty for CSI matches);
/// OSC-8 hyperlinks are replaced by their link text. Works on bytes;
/// `&str` is scanned as UTF-8 (all markers are ASCII, so byte ranges
/// are char-boundary safe).
pub fn strip_ansi_bytes(bytes: &[u8]) -> Vec<u8> {
    let mut out: Vec<u8> = Vec::with_capacity(bytes.len());
    let mut pos = 0;
    while let Some(m) = next_ansi(bytes, pos) {
        match m {
            AnsiMatch::Csi { start, end } => {
                out.extend_from_slice(&bytes[pos..start]);
                pos = end;
            }
            AnsiMatch::Hyperlink {
                start,
                end,
                text_start,
                text_end,
            } => {
                out.extend_from_slice(&bytes[pos..start]);
                out.extend_from_slice(&bytes[text_start..text_end]);
                pos = end;
            }
        }
    }
    out.extend_from_slice(&bytes[pos..]);
    out
}

pub fn strip_ansi(s: &str) -> String {
    String::from_utf8(strip_ansi_bytes(s.as_bytes())).unwrap_or_else(|_| s.to_string())
}

/// True if the string contains an ANSI match (used for the
/// colored-number bail-out: formatting must go through the
/// pure-Python dance in that case).
pub fn contains_ansi_bytes(bytes: &[u8]) -> bool {
    next_ansi(bytes, 0).is_some()
}

/// `_is_multiline` (`U:1142`): `re.search(r"\r|\n", s)`.
/// (The compiled `_multiline_codes` also lists `\r\n`, but alternation
/// tries `\r` first, so this is a per-character `\r`/`\n` test.)
pub fn is_multiline_str(s: &str) -> bool {
    s.as_bytes().iter().any(|&b| b == b'\r' || b == b'\n')
}

pub fn is_multiline_bytes(b: &[u8]) -> bool {
    b.iter().any(|&c| c == b'\r' || c == b'\n')
}

/// `re.split(r"[\r\n]", s)`: split on each `\r`/`\n` (`U:1211`, `U:1149`).
pub fn split_lines(s: &str) -> Vec<&str> {
    if s.is_empty() {
        return vec![s];
    }
    let mut out = Vec::new();
    let mut start = 0;
    for (i, b) in s.as_bytes().iter().enumerate() {
        if *b == b'\r' || *b == b'\n' {
            out.push(&s[start..i]);
            start = i + 1;
        }
    }
    out.push(&s[start..]);
    out
}

/// Python `str.splitlines()` semantics, needed by `_append_multiline_row`
/// (`U:2605`): boundaries are `\n`, `\r`, `\r\n`, `\v` (0x0b), `\f`
/// (0x0c), `\x1c`-`\x1e`, `\x85`, `\u2028`, `\u2029`.
pub fn splitlines_py(s: &str) -> Vec<&str> {
    // Python `"".splitlines() == []`.
    if s.is_empty() {
        return Vec::new();
    }
    let mut out = Vec::new();
    let mut start = 0;
    let bytes = s.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        let (len, is_break, skip_extra) = match bytes[i] {
            b'\n' => (1, true, 0),
            b'\r' => {
                if i + 1 < bytes.len() && bytes[i + 1] == b'\n' {
                    (1, true, 1)
                } else {
                    (1, true, 0)
                }
            }
            b'\x0b' | b'\x0c' | b'\x1c' | b'\x1d' | b'\x1e' => (1, true, 0),
            0xc2 if i + 1 < bytes.len() && bytes[i + 1] == 0x85 => (2, true, 0),
            0xe2 if i + 2 < bytes.len()
                && bytes[i + 1] == 0x80
                && (bytes[i + 2] == 0xa8 || bytes[i + 2] == 0xa9) =>
            {
                (3, true, 0)
            }
            _ => {
                let c = s[i..].chars().next().unwrap();
                (c.len_utf8(), false, 0)
            }
        };
        if is_break {
            out.push(&s[start..i]);
            i += len + skip_extra;
            start = i;
        } else {
            i += len;
        }
    }
    out.push(&s[start..]);
    // Python splitlines() drops the final empty piece after a trailing
    // boundary: "a\n".splitlines() == ["a"]. But `_append_multiline_row`
    // uses `(ms.splitlines() or ms)`: only emptiness matters there, plus
    // zip pairing in the caller. Match splitlines exactly:
    if out.len() > 1 && out.last() == Some(&"") {
        // A trailing boundary produced a final ""; Python drops it.
        out.pop();
    }
    out
}

/// Visible width of a single-line string given the line-width function.
pub fn multiline_max_width(s: &str, line_width: impl Fn(&str) -> usize) -> usize {
    split_lines(s)
        .iter()
        .map(|l| line_width(l))
        .max()
        .unwrap_or(0)
}

/// `str(cell)` for display in the ANSI-detection scan is unnecessary:
/// cells that need `str()` (non str/bytes) cannot contain `\r`/`\n`
/// (`"None"`, `"True"`, ints, shortest floats). Only str/bytes cells
/// and headers are scanned.
pub fn cell_needs_scan() -> bool {
    true
}

pub fn unreachable_fallback() -> Fallback {
    Fallback
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn strip_csi() {
        assert_eq!(strip_ansi("\x1b[31mred\x1b[0m text"), "red text");
    }

    #[test]
    fn strip_hyperlink_keeps_text() {
        assert_eq!(
            strip_ansi("\x1b]8;;https://example.com\x1b\\This is a link\x1b]8;;\x1b\\"),
            "This is a link"
        );
    }

    #[test]
    fn strip_noop() {
        assert_eq!(strip_ansi("plain"), "plain");
    }

    #[test]
    fn strip_broken_escape_is_literal() {
        // `\x1b[` + `x`: `x` is not a valid final byte... `x` IS in
        // 0x40-0x7e, so it IS a valid (unknown) CSI. Use `\x1b[1` (EOF):
        assert_eq!(strip_ansi("a\x1b"), "a\x1b");
        assert_eq!(strip_ansi("a\x1b[1"), "a\x1b[1");
    }

    #[test]
    fn splitlines_matches_python() {
        assert_eq!(splitlines_py("a\nb"), vec!["a", "b"]);
        assert_eq!(splitlines_py("a\r\nb"), vec!["a", "b"]);
        assert_eq!(splitlines_py("a\rb"), vec!["a", "b"]);
        assert_eq!(splitlines_py("a\n"), vec!["a"]);
        assert_eq!(splitlines_py(""), Vec::<&str>::new());
        assert_eq!(splitlines_py("a\x0bb"), vec!["a", "b"]);
        assert_eq!(splitlines_py("a\u{2028}b"), vec!["a", "b"]);
        assert_eq!(splitlines_py("a\u{2029}b"), vec!["a", "b"]);
    }

    #[test]
    fn rstrip_unicode_spaces() {
        assert_eq!(rstrip("ab\u{3000}"), "ab");
        assert_eq!(rstrip("ab\u{3000} \t"), "ab");
    }

    #[test]
    fn space_table_spot() {
        assert!(is_space(' '));
        assert!(is_space('\u{3000}'));
        assert!(is_space('\u{85}'));
        assert!(!is_space('\u{200b}'));
        assert!(!is_space('\x01'));
    }
}
