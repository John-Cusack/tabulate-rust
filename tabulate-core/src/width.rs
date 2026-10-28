//! Visible-width computation.
//! Ports `U:1124 _visible_width`, `U:1149 _multiline_width`,
//! `U:1154 _choose_width_fn`, `U:1195 _align_column_choose_width_fn`,
//! `U:1211 _align_column_multiline_width` and the `wcwidth` dependency
//! (pinned 0.2.13; tables in `width_tables.rs`, generated from the
//! installed package).
//!
//! `wcwidth.wcswidth` returns -1 for control characters and upstream
//! does arithmetic with it. Emulating that exactly in unsigned
//! arithmetic is error-prone, so any control character in wide mode
//! reports [`Fallback`]: the pure-Python replay is exact, and the path
//! is vanishingly rare.
use crate::ansi;
use crate::width_tables;
use crate::Fallback;

#[derive(Debug, Clone, Copy)]
pub struct WidthMode {
    pub has_invisible: bool,
    pub wide: bool,
    pub multiline: bool,
    version: &'static str,
}

impl WidthMode {
    pub fn choose(has_invisible: bool, wide: bool, multiline: bool) -> Result<WidthMode, Fallback> {
        // `U:2379`: `enable_widechars = wcwidth is not None and
        // WIDE_CHARS_MODE`. The binding passes the live flag; a
        // `UNICODE_VERSION` value that would warn upstream (invalid or
        // below the supported range) bails here instead, once per call.
        let version = match std::env::var("UNICODE_VERSION") {
            Err(_) => "15.1.0",
            Ok(v) => {
                if v == "auto" || v == "latest" {
                    "15.1.0"
                } else if version_warns(&v) {
                    return Err(Fallback);
                } else {
                    nearest_version(&v)
                }
            }
        };
        Ok(WidthMode {
            has_invisible,
            wide,
            multiline,
            version,
        })
    }
    /// `U:1154` selection. Signed: `wcswidth` yields -1 for control
    /// characters and upstream does arithmetic with it (`max` saves
    /// every use; see the `minwidth < 0` bail in `lib.rs`).
    pub fn width(&self, s: &str) -> i64 {
        if self.multiline {
            let mut m = 0;
            for l in ansi::split_lines(s) {
                m = m.max(self.line_width(l));
            }
            m
        } else {
            self.line_width(s)
        }
    }

    /// `U:1211` list form (per physical line).
    pub fn widths(&self, s: &str) -> Vec<i64> {
        ansi::split_lines(s)
            .iter()
            .map(|l| self.line_width(l))
            .collect()
    }

    /// `U:1149` max form.
    pub fn max_width(&self, s: &str) -> i64 {
        self.width(s)
    }

    fn line_width(&self, s: &str) -> i64 {
        if self.has_invisible {
            let stripped = ansi::strip_ansi(s);
            if self.wide {
                wcs_width(&stripped, self.version)
            } else {
                stripped.chars().count() as i64
            }
        } else if self.wide {
            wcs_width(s, self.version)
        } else {
            s.chars().count() as i64
        }
    }
}

/// `wcwidth.wcswidth(s)` on an explicit table version: -1 on C0/C1
/// controls, exactly like upstream (callers fold it into `max`).
pub fn wcs_width(s: &str, version: &str) -> i64 {
    let mut width = 0i64;
    let mut chars = s.chars().peekable();
    let mut last_measured: Option<char> = None;
    while let Some(c) = chars.next() {
        if c == '\u{200d}' {
            // ZWJ: skip it and the next character.
            chars.next();
            continue;
        }
        if c == '\u{fe0f}' {
            if let Some(lm) = last_measured {
                if version_at_least_9(version) {
                    width += bisearch(lm as u32, vs16_table()) as i64;
                }
                last_measured = None;
            }
            continue;
        }
        let w = wcwidth_char(c, version);
        if w < 0 {
            return -1;
        }
        if w > 0 {
            last_measured = Some(c);
        }
        width += w as i64;
    }
    width
}

/// Single character `wcwidth(wc)` (`U:116-158`), -1/0/1/2.
fn wcwidth_char(c: char, version: &str) -> i32 {
    let ucs = c as u32;
    if (32..0x7f).contains(&ucs) {
        return 1;
    }
    if ucs != 0 && (ucs < 32 || (0x7f..0xa0).contains(&ucs)) {
        return -1;
    }
    let (zero, wide) = tables_for(version);
    if bisearch(ucs, zero) != 0 {
        return 0;
    }
    1 + bisearch(ucs, wide) as i32
}

/// `U:88 _bisearch`: 1 if `ucs` is in the interval table, else 0.
fn bisearch(ucs: u32, table: &[(u32, u32)]) -> u8 {
    if table.is_empty() || ucs < table[0].0 || ucs > table[table.len() - 1].1 {
        return 0;
    }
    let mut lbound = 0usize;
    let mut ubound = table.len() - 1;
    while ubound >= lbound {
        let mid = (lbound + ubound) / 2;
        if ucs > table[mid].1 {
            lbound = mid + 1;
        } else if ucs < table[mid].0 {
            if mid == 0 {
                break;
            }
            ubound = mid - 1;
        } else {
            return 1;
        }
    }
    0
}

/// Interval table for `bisearch` (generated from wcwidth 0.2.13).
type RangeTable = &'static [(u32, u32)];

fn tables_for(version: &str) -> (RangeTable, RangeTable) {
    // Versions from wcwidth 0.2.13 `list_versions()`.
    match version {
        "4.1.0" => (width_tables::ZERO_4_1_0, width_tables::WIDE_4_1_0),
        "5.0.0" => (width_tables::ZERO_5_0_0, width_tables::WIDE_5_0_0),
        "5.1.0" => (width_tables::ZERO_5_1_0, width_tables::WIDE_5_1_0),
        "5.2.0" => (width_tables::ZERO_5_2_0, width_tables::WIDE_5_2_0),
        "6.0.0" => (width_tables::ZERO_6_0_0, width_tables::WIDE_6_0_0),
        "6.1.0" => (width_tables::ZERO_6_1_0, width_tables::WIDE_6_1_0),
        "6.2.0" => (width_tables::ZERO_6_2_0, width_tables::WIDE_6_2_0),
        "6.3.0" => (width_tables::ZERO_6_3_0, width_tables::WIDE_6_3_0),
        "7.0.0" => (width_tables::ZERO_7_0_0, width_tables::WIDE_7_0_0),
        "8.0.0" => (width_tables::ZERO_8_0_0, width_tables::WIDE_8_0_0),
        "9.0.0" => (width_tables::ZERO_9_0_0, width_tables::WIDE_9_0_0),
        "10.0.0" => (width_tables::ZERO_10_0_0, width_tables::WIDE_10_0_0),
        "11.0.0" => (width_tables::ZERO_11_0_0, width_tables::WIDE_11_0_0),
        "12.0.0" => (width_tables::ZERO_12_0_0, width_tables::WIDE_12_0_0),
        "12.1.0" => (width_tables::ZERO_12_1_0, width_tables::WIDE_12_1_0),
        "13.0.0" => (width_tables::ZERO_13_0_0, width_tables::WIDE_13_0_0),
        "14.0.0" => (width_tables::ZERO_14_0_0, width_tables::WIDE_14_0_0),
        "15.0.0" => (width_tables::ZERO_15_0_0, width_tables::WIDE_15_0_0),
        _ => (width_tables::ZERO_15_1_0, width_tables::WIDE_15_1_0),
    }
}

fn vs16_table() -> &'static [(u32, u32)] {
    width_tables::VS16
}

fn version_at_least_9(version: &str) -> bool {
    version_value(version) >= (9, 0, 0)
}

fn version_value(v: &str) -> (u64, u64, u64) {
    let mut it = v.split('.').map(|p| p.parse::<u64>().unwrap_or(0));
    (
        it.next().unwrap_or(0),
        it.next().unwrap_or(0),
        it.next().unwrap_or(0),
    )
}

/// True when `UNICODE_VERSION=v` would make upstream warn (invalid
/// format or below the supported range): the kernel bails instead so
/// the fallback emits the identical warning.
fn version_warns(v: &str) -> bool {
    if v == "auto" || v == "latest" {
        return false;
    }
    if width_tables::VERSIONS.contains(&v) {
        return false;
    }
    let parsed: Option<(u64, u64, u64)> = (|| {
        let mut it = v.split('.');
        let a = it.next()?.parse().ok()?;
        let b = it.next().unwrap_or("0").parse().ok()?;
        let c = it.next().unwrap_or("0").parse().ok()?;
        if it.next().is_some() {
            return None;
        }
        Some((a, b, c))
    })();
    match parsed {
        None => true,
        Some(given) => {
            let earliest = version_value(width_tables::VERSIONS[0]);
            given <= earliest
        }
    }
}

#[allow(dead_code)]
fn nearest_version(v: &str) -> &'static str {
    // Nearest-at-or-below match for valid-but-unsupported version
    // strings (no warning upstream). Only reached for values that
    // parsed and exceed the earliest level.
    let given = version_value(v);
    let mut best = width_tables::VERSIONS[0];
    let mut best_val = version_value(best);
    for cand in width_tables::VERSIONS.iter() {
        let cv = version_value(cand);
        if cv <= given && cv >= best_val {
            best = cand;
            best_val = cv;
        }
    }
    best
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ascii_width() {
        let m = WidthMode {
            has_invisible: false,
            wide: false,
            multiline: false,
            version: "15.1.0",
        };
        assert_eq!(m.width("hello"), 5);
    }

    #[test]
    fn wide_cjk() {
        assert_eq!(wcs_width("配列", "15.1.0"), 4);
        assert_eq!(wcs_width("hello", "15.1.0"), 5);
        assert_eq!(wcs_width("", "15.1.0"), 0);
    }

    #[test]
    fn wide_controls_neg() {
        // Upstream yields -1; callers fold it into `max`.
        assert_eq!(wcs_width("a\tb", "15.1.0"), -1);
        assert_eq!(wcs_width("a\x01b", "15.1.0"), -1);
    }

    #[test]
    fn wcwidth_vectors() {
        // Spot checks against wcwidth 0.2.13.
        assert_eq!(wcwidth_char('a', "15.1.0"), 1);
        assert_eq!(wcwidth_char(' ', "15.1.0"), 1);
        assert_eq!(wcwidth_char('\u{a0}', "15.1.0"), 1);
        assert_eq!(wcwidth_char('\u{300}', "15.1.0"), 0);
        assert_eq!(wcwidth_char('\u{4e2d}', "15.1.0"), 2);
        assert_eq!(wcwidth_char('\u{200d}', "15.1.0"), 0);
    }
}
