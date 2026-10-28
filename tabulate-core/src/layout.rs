//! Table layout: the 26 named formats and `_format_table`.
//! Ports `U:136`-`U:315` (row/line builders), `U:318`
//! (`LATEX_ESCAPE_RULES`), `U:362` (`_table_formats`), `U:2541`
//! (`_expand_iterable` use), `U:2555`-`U:2637` (row/line builders),
//! `U:2653` (`_format_table`).

use crate::ansi;
use crate::width::WidthMode;
use crate::{Align, Fallback, RowAlign, TableFmt};
use std::borrow::Cow;

/// A layout row: formatted data or a reinserted separator.
#[derive(Debug, Clone)]
pub enum Row {
    Data(Vec<String>),
    Sep,
}

/// Escape the first column of rst tables (`U:343 _rst_escape_first_column`).
pub fn rst_escape_first_column(rows: &mut [Vec<crate::InputCell>], headers: &mut [Cow<str>]) {
    if let Some(h0) = headers.first() {
        if ansi::strip(h0).is_empty() {
            headers[0] = Cow::Borrowed("..");
        }
    }
    for row in rows.iter_mut() {
        if row.is_empty() {
            continue;
        }
        // `U:344`: `isinstance(val, (str, bytes)) and not val.strip()`.
        let empty = match &row[0].cell {
            crate::Cell::Str(s) => ansi::strip(s).is_empty(),
            crate::Cell::Bytes(b) => std::str::from_utf8(b)
                .map(|s| ansi::strip(s).is_empty())
                .unwrap_or(false),
            _ => false,
        };
        if empty {
            row[0] = crate::InputCell {
                cell: crate::Cell::Str(".."),
                delegated: None,
            };
        }
    }
}

// ---------------------------------------------------------------------------
// Line builders
// ---------------------------------------------------------------------------

/// `U:136 _pipe_segment_with_colons`. Python `"x" * negative == ""`.
fn pipe_segment(align: &Align, colwidth: i64) -> String {
    let w = colwidth.max(0) as usize;
    match align {
        Align::Right | Align::Decimal => format!("{}:", "-".repeat(w.saturating_sub(1))),
        Align::Center => format!(":{}:", "-".repeat(w.saturating_sub(2))),
        Align::Left => format!(":{}", "-".repeat(w.saturating_sub(1))),
        _ => "-".repeat(w),
    }
}

/// `U:161 _grid_segment_with_colons`.
fn grid_segment(colwidth: i64, align: &Align) -> String {
    let width = colwidth.max(0) as usize;
    match align {
        Align::Right => format!("{}:", "=".repeat(width.saturating_sub(1))),
        Align::Center => format!(":{}:", "=".repeat(width.saturating_sub(2))),
        Align::Left => format!(":{}", "=".repeat(width.saturating_sub(1))),
        _ => "=".repeat(width),
    }
}
fn grid_line(colwidths: &[i64], colaligns: &[Align]) -> String {
    let aligns: Vec<Align> = if colaligns.is_empty() {
        vec![Align::Other; colwidths.len()]
    } else {
        colaligns.to_vec()
    };
    // `U:181`: `_grid_segment_with_colons(w, a) for a, w in
    // zip(colaligns, colwidths)` — (width, align) order.
    let segments: Vec<String> = aligns
        .iter()
        .zip(colwidths.iter())
        .map(|(a, w)| grid_segment(*w, a))
        .collect();
    format!("+{}+", segments.join("+"))
}

/// `U:150 _pipe_line_with_colons`.
fn pipe_line(colwidths: &[i64], colaligns: &[Align]) -> String {
    let aligns: Vec<Align> = if colaligns.is_empty() {
        vec![Align::Other; colwidths.len()]
    } else {
        colaligns.to_vec()
    };
    let segments: Vec<String> = aligns
        .iter()
        .zip(colwidths.iter())
        .map(|(a, w)| pipe_segment(a, *w))
        .collect();
    format!("|{}|", segments.join("|"))
}

// ---------------------------------------------------------------------------
// HTML helpers
// ---------------------------------------------------------------------------

/// Python `html.escape` (`U:17 htmlescape`): `&<>"'` ->
/// `&amp;&lt;&gt;&quot;&#x27;`.
fn html_escape(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            '\'' => out.push_str("&#x27;"),
            _ => out.push(c),
        }
    }
    out
}

/// `U:214 _html_row_with_attrs`.
fn html_row(celltag: &str, unsafe_: bool, cell_values: &[String], colaligns: &[Align]) -> String {
    fn style(a: &Align) -> &'static str {
        match a {
            Align::Right | Align::Decimal => " style=\"text-align: right;\"",
            Align::Center => " style=\"text-align: center;\"",
            _ => "",
        }
    }
    let values: Vec<String> = cell_values
        .iter()
        .zip(colaligns.iter())
        .map(|(c, a)| {
            let body = if unsafe_ { c.clone() } else { html_escape(c) };
            format!("<{celltag}{}>{body}</{celltag}>", style(a))
        })
        .collect();
    // `U:231`: rstrip the joined values (trailing whitespace only).
    let joined = ansi::rstrip(&values.concat()).to_string();
    let rowhtml = format!("<tr>{joined}</tr>");
    if celltag == "th" {
        format!("<table>\n<thead>\n{rowhtml}\n</thead>\n<tbody>")
    } else {
        rowhtml
    }
}

// ---------------------------------------------------------------------------
// LaTeX helpers
// ---------------------------------------------------------------------------

const LATEX_ESCAPE_RULES: &[(&str, &str)] = &[
    ("&", "\\&"),
    ("%", "\\%"),
    ("$", "\\$"),
    ("#", "\\#"),
    ("_", "\\_"),
    ("^", "\\^{}"),
    ("{", "\\{"),
    ("}", "\\}"),
    ("~", "\\textasciitilde{}"),
    ("\\", "\\textbackslash{}"),
    ("<", "\\ensuremath{<}"),
    (">", "\\ensuremath{>}"),
];

/// `U:334 _latex_row`.
fn latex_row(cell_values: &[String], raw: bool) -> String {
    let escaped: Vec<String> = cell_values
        .iter()
        .map(|cell| {
            if raw {
                return cell.clone();
            }
            let mut out = String::with_capacity(cell.len());
            for c in cell.chars() {
                let cs = c.to_string();
                let mut done = false;
                for (k, v) in LATEX_ESCAPE_RULES {
                    if cs == *k {
                        out.push_str(v);
                        done = true;
                        break;
                    }
                }
                if !done {
                    out.push(c);
                }
            }
            out
        })
        .collect();
    build_simple_row(&escaped, "", "&", "\\\\")
}

/// `U:251 _latex_line_begin_tabular`.
fn latex_begin(colaligns: &[Align], booktabs: bool, longtable: bool) -> String {
    fn col(a: &Align) -> char {
        match a {
            Align::Left => 'l',
            Align::Right | Align::Decimal => 'r',
            Align::Center => 'c',
            _ => 'l',
        }
    }
    let cols: String = colaligns.iter().map(col).collect();
    let begin = if longtable {
        format!("\\begin{{longtable}}{{{cols}}}")
    } else {
        format!("\\begin{{tabular}}{{{cols}}}")
    };
    let rule = if booktabs { "\\toprule" } else { "\\hline" };
    format!("{begin}\n{rule}")
}

// ---------------------------------------------------------------------------
// Misc row builders
// ---------------------------------------------------------------------------

/// `U:186 _mediawiki_row_with_attrs`.
fn mediawiki_row(separator: char, cell_values: &[String], colaligns: &[Align]) -> String {
    fn style(a: &Align) -> &'static str {
        match a {
            Align::Right | Align::Decimal => "style=\"text-align: right;\"| ",
            Align::Center => "style=\"text-align: center;\"| ",
            _ => "",
        }
    }
    let values: Vec<String> = cell_values
        .iter()
        .zip(colaligns.iter())
        .map(|(c, a)| format!(" {}{c} ", style(a)))
        .collect();
    let sep = separator.to_string();
    let colsep = format!("{sep}{sep}");
    ansi::rstrip(&format!("{}{}", sep, values.join(&colsep))).to_string()
}

/// `U:202 _textile_row_with_attrs` (mutates `cell_values[0] += " "`).
/// `U:203`: {"left": "<.", "right": ">.", "center": "=.", "decimal": ">."}.
fn textile_row(cell_values: &[String], colaligns: &[Align]) -> String {
    fn prefix(a: &Align) -> &'static str {
        match a {
            Align::Left => "<.",
            Align::Right | Align::Decimal => ">.",
            Align::Center => "=.",
            _ => "",
        }
    }
    let mut cells = cell_values.to_vec();
    if !cells.is_empty() {
        cells[0].push(' ');
    }
    let values: Vec<String> = colaligns
        .iter()
        .zip(cells.iter())
        .map(|(a, v)| format!("{}{v}", prefix(a)))
        .collect();
    format!("|{}|", values.join("|"))
}

/// `U:237 _moin_row_with_attrs`.
/// `"{}{} {} ".format(celltag, align, header + c + header)`, joined, + `"||"`.
fn moin_row(celltag: &str, cell_values: &[String], colaligns: &[Align], header: &str) -> String {
    fn style(a: &Align) -> &'static str {
        match a {
            Align::Right | Align::Decimal => "<style=\"text-align: right;\">",
            Align::Center => "<style=\"text-align: center;\">",
            _ => "",
        }
    }
    let values: Vec<String> = cell_values
        .iter()
        .zip(colaligns.iter())
        .map(|(c, a)| format!("{celltag}{} {header}{c}{header} ", style(a)))
        .collect();
    format!("{}||", values.concat())
}

/// `U:264 _asciidoc_row`: the header-line half (`make_header_line`).
/// Upstream dispatches on argument count; the layout calls this for
/// lines (`lineabove`) and `asciidoc_data` for rows.
fn asciidoc_head(
    is_header: bool,
    colwidths: &[i64],
    colaligns: &[Align],
) -> Result<String, Fallback> {
    fn colchar(a: &Align) -> Result<char, Fallback> {
        match a {
            Align::Left => Ok('<'),
            Align::Right | Align::Decimal => Ok('>'),
            Align::Center => Ok('^'),
            // `U:273`: `alignment[colalign]` raises KeyError upstream.
            _ => Err(Fallback),
        }
    }
    let specs: Vec<String> = colwidths
        .iter()
        .zip(colaligns.iter())
        .map(|(w, a)| Ok::<String, Fallback>(format!("{}{w}", colchar(a)?)))
        .collect::<Result<_, _>>()?;
    let mut header_list = vec![format!("cols=\"{}\"", specs.join(","))];
    if is_header {
        header_list.push("options=\"header\"".to_string());
    }
    Ok(format!("[{}]\n|====", header_list.join(",")))
}

fn asciidoc_data(
    is_header: bool,
    cell_values: &[String],
    colwidths: &[i64],
    colaligns: &[Align],
) -> Result<String, Fallback> {
    let data_line = format!("|{}", cell_values.join("|"));
    if is_header {
        let head = asciidoc_head(true, colwidths, colaligns)?;
        Ok(format!("{head}\n{data_line}"))
    } else {
        Ok(data_line)
    }
}

/// `U:2566 _build_simple_row`.
fn build_simple_row(padded_cells: &[String], begin: &str, sep: &str, end: &str) -> String {
    ansi::rstrip(&format!("{}{}{}", begin, padded_cells.join(sep), end)).to_string()
}

/// `U:2555 _pad_row`.
fn pad_row(cells: &[String], padding: usize) -> Vec<String> {
    if cells.is_empty() {
        return Vec::new();
    }
    let pad = " ".repeat(padding);
    cells.iter().map(|c| format!("{pad}{c}{pad}")).collect()
}

/// `U:2588 _align_cell_veritically`.
fn align_cell_vertically(
    text_lines: Vec<String>,
    num_lines: usize,
    column_width: i64,
    row_alignment: RowAlign,
) -> Vec<String> {
    let blank = " ".repeat(column_width.max(0) as usize);
    let delta = num_lines.saturating_sub(text_lines.len());
    match row_alignment {
        RowAlign::Bottom => {
            let mut out = vec![blank.clone(); delta];
            out.extend(text_lines);
            out
        }
        RowAlign::Center => {
            let top = delta / 2;
            let bottom = delta - top;
            let mut out = vec![blank.clone(); top];
            out.extend(text_lines);
            out.extend(vec![blank; bottom]);
            out
        }
        RowAlign::Top => {
            let mut out = text_lines;
            out.extend(vec![blank; delta]);
            out
        }
    }
}

// ---------------------------------------------------------------------------
// Format description table
// ---------------------------------------------------------------------------

#[derive(Debug, Clone)]
enum LineDef {
    None,
    Static {
        begin: &'static str,
        fill: &'static str,
        sep: &'static str,
        end: &'static str,
    },
    /// `_pipe_line_with_colons`.
    Pipe,
    /// `_grid_line_with_colons`.
    Grid,
    /// `_latex_line_begin_tabular`.
    LatexBegin {
        booktabs: bool,
        longtable: bool,
    },
    /// `_html_begin_table_without_header`.
    HtmlBegin,
    /// `_asciidoc_row(False)` as a line.
    AsciiLine,
    /// Literal line text.
    Text(&'static str),
}

#[derive(Debug, Clone)]
enum RowDef {
    Static {
        begin: &'static str,
        sep: &'static str,
        end: &'static str,
    },
    MediaWiki {
        sep: char,
    },
    Textile,
    Html {
        tag: &'static str,
        unsafe_: bool,
    },
    Moin {
        tag: &'static str,
        header: &'static str,
    },
    Latex {
        raw: bool,
    },
    AsciiData {
        header: bool,
    },
}

#[derive(Debug, Clone)]
struct FmtDef {
    lineabove: LineDef,
    linebelowheader: LineDef,
    linebetweenrows: Option<LineDef>,
    linebelow: Option<LineDef>,
    headerrow: RowDef,
    datarow: RowDef,
    padding: usize,
    with_header_hide: &'static [&'static str],
}

fn fmt_def(fmt: TableFmt) -> FmtDef {
    use LineDef as L;
    use RowDef as R;
    match fmt {
        TableFmt::Simple => FmtDef {
            lineabove: L::Static {
                begin: "",
                fill: "-",
                sep: "  ",
                end: "",
            },
            linebelowheader: L::Static {
                begin: "",
                fill: "-",
                sep: "  ",
                end: "",
            },
            linebetweenrows: None,
            linebelow: L::Static {
                begin: "",
                fill: "-",
                sep: "  ",
                end: "",
            }
            .into(),
            headerrow: R::Static {
                begin: "",
                sep: "  ",
                end: "",
            },
            datarow: R::Static {
                begin: "",
                sep: "  ",
                end: "",
            },
            padding: 0,
            with_header_hide: &["lineabove", "linebelow"],
        },
        TableFmt::Plain => FmtDef {
            lineabove: L::None,
            linebelowheader: L::None,
            linebetweenrows: None,
            linebelow: None,
            headerrow: R::Static {
                begin: "",
                sep: "  ",
                end: "",
            },
            datarow: R::Static {
                begin: "",
                sep: "  ",
                end: "",
            },
            padding: 0,
            with_header_hide: &[],
        },
        TableFmt::Grid => grid_def("+", "-", "+", "+", "|", "|", "|"),
        TableFmt::SimpleGrid => grid_def("┌", "─", "┬", "┐", "│", "│", "│")
            .with_between("├", "─", "┼", "┤")
            .with_below("└", "─", "┴", "┘")
            .with_header_line("├", "─", "┼", "┤"),
        TableFmt::RoundedGrid => grid_def("╭", "─", "┬", "╮", "│", "│", "│")
            .with_between("├", "─", "┼", "┤")
            .with_below("╰", "─", "┴", "╯")
            .with_header_line("├", "─", "┼", "┤"),
        TableFmt::HeavyGrid => grid_def("┏", "━", "┳", "┓", "┃", "┃", "┃")
            .with_between("┣", "━", "╋", "┫")
            .with_below("┗", "━", "┻", "┛")
            .with_header_line("┣", "━", "╋", "┫"),
        TableFmt::MixedGrid => grid_def("┍", "━", "┯", "┑", "│", "│", "│")
            .with_between("├", "─", "┼", "┤")
            .with_below("┕", "━", "┷", "┙")
            .with_header_line("┝", "━", "┿", "┥"),
        TableFmt::DoubleGrid => grid_def("╔", "═", "╦", "╗", "║", "║", "║")
            .with_between("╠", "═", "╬", "╣")
            .with_below("╚", "═", "╩", "╝")
            .with_header_line("╠", "═", "╬", "╣"),
        TableFmt::FancyGrid => grid_def("╒", "═", "╤", "╕", "│", "│", "│")
            .with_between("├", "─", "┼", "┤")
            .with_below("╘", "═", "╧", "╛")
            .with_header_line("╞", "═", "╪", "╡"),
        TableFmt::ColonGrid => FmtDef {
            lineabove: L::Static {
                begin: "+",
                fill: "-",
                sep: "+",
                end: "+",
            },
            linebelowheader: L::Grid,
            linebetweenrows: L::Static {
                begin: "+",
                fill: "-",
                sep: "+",
                end: "+",
            }
            .into(),
            linebelow: L::Static {
                begin: "+",
                fill: "-",
                sep: "+",
                end: "+",
            }
            .into(),
            headerrow: R::Static {
                begin: "|",
                sep: "|",
                end: "|",
            },
            datarow: R::Static {
                begin: "|",
                sep: "|",
                end: "|",
            },
            padding: 1,
            with_header_hide: &[],
        },
        TableFmt::Outline => FmtDef {
            lineabove: L::Static {
                begin: "+",
                fill: "-",
                sep: "+",
                end: "+",
            },
            linebelowheader: L::Static {
                begin: "+",
                fill: "=",
                sep: "+",
                end: "+",
            },
            linebetweenrows: None,
            linebelow: L::Static {
                begin: "+",
                fill: "-",
                sep: "+",
                end: "+",
            }
            .into(),
            headerrow: R::Static {
                begin: "|",
                sep: "|",
                end: "|",
            },
            datarow: R::Static {
                begin: "|",
                sep: "|",
                end: "|",
            },
            padding: 1,
            with_header_hide: &[],
        },
        TableFmt::SimpleOutline => outline_def(
            "┌", "─", "┬", "┐", "├", "─", "┼", "┤", "└", "─", "┴", "┘", "│", "│", "│",
        ),
        TableFmt::RoundedOutline => outline_def(
            "╭", "─", "┬", "╮", "├", "─", "┼", "┤", "╰", "─", "┴", "╯", "│", "│", "│",
        ),
        TableFmt::HeavyOutline => outline_def(
            "┏", "━", "┳", "┓", "┣", "━", "╋", "┫", "┗", "━", "┻", "┛", "┃", "┃", "┃",
        ),
        TableFmt::MixedOutline => outline_def(
            "┍", "━", "┯", "┑", "┝", "━", "┿", "┥", "┕", "━", "┷", "┙", "│", "│", "│",
        ),
        TableFmt::DoubleOutline => outline_def(
            "╔", "═", "╦", "╗", "╠", "═", "╬", "╣", "╚", "═", "╩", "╝", "║", "║", "║",
        ),
        TableFmt::FancyOutline => outline_def(
            "╒", "═", "╤", "╕", "╞", "═", "╪", "╡", "╘", "═", "╧", "╛", "│", "│", "│",
        ),
        TableFmt::Github => FmtDef {
            lineabove: L::Static {
                begin: "|",
                fill: "-",
                sep: "|",
                end: "|",
            },
            linebelowheader: L::Static {
                begin: "|",
                fill: "-",
                sep: "|",
                end: "|",
            },
            linebetweenrows: None,
            linebelow: None,
            headerrow: R::Static {
                begin: "|",
                sep: "|",
                end: "|",
            },
            datarow: R::Static {
                begin: "|",
                sep: "|",
                end: "|",
            },
            padding: 1,
            with_header_hide: &["lineabove"],
        },
        TableFmt::Pipe => FmtDef {
            lineabove: L::Pipe,
            linebelowheader: L::Pipe,
            linebetweenrows: None,
            linebelow: None,
            headerrow: R::Static {
                begin: "|",
                sep: "|",
                end: "|",
            },
            datarow: R::Static {
                begin: "|",
                sep: "|",
                end: "|",
            },
            padding: 1,
            with_header_hide: &["lineabove"],
        },
        TableFmt::Orgtbl => FmtDef {
            lineabove: L::None,
            linebelowheader: L::Static {
                begin: "|",
                fill: "-",
                sep: "+",
                end: "|",
            },
            linebetweenrows: None,
            linebelow: None,
            headerrow: R::Static {
                begin: "|",
                sep: "|",
                end: "|",
            },
            datarow: R::Static {
                begin: "|",
                sep: "|",
                end: "|",
            },
            padding: 1,
            with_header_hide: &[],
        },
        TableFmt::Jira => FmtDef {
            lineabove: L::None,
            linebelowheader: L::None,
            linebetweenrows: None,
            linebelow: None,
            headerrow: R::Static {
                begin: "||",
                sep: "||",
                end: "||",
            },
            datarow: R::Static {
                begin: "|",
                sep: "|",
                end: "|",
            },
            padding: 1,
            with_header_hide: &[],
        },
        TableFmt::Presto => FmtDef {
            lineabove: L::None,
            linebelowheader: L::Static {
                begin: "",
                fill: "-",
                sep: "+",
                end: "",
            },
            linebetweenrows: None,
            linebelow: None,
            headerrow: R::Static {
                begin: "",
                sep: "|",
                end: "",
            },
            datarow: R::Static {
                begin: "",
                sep: "|",
                end: "",
            },
            padding: 1,
            with_header_hide: &[],
        },
        TableFmt::Pretty => FmtDef {
            lineabove: L::Static {
                begin: "+",
                fill: "-",
                sep: "+",
                end: "+",
            },
            linebelowheader: L::Static {
                begin: "+",
                fill: "-",
                sep: "+",
                end: "+",
            },
            linebetweenrows: None,
            linebelow: L::Static {
                begin: "+",
                fill: "-",
                sep: "+",
                end: "+",
            }
            .into(),
            headerrow: R::Static {
                begin: "|",
                sep: "|",
                end: "|",
            },
            datarow: R::Static {
                begin: "|",
                sep: "|",
                end: "|",
            },
            padding: 1,
            with_header_hide: &[],
        },
        TableFmt::Psql => FmtDef {
            lineabove: L::Static {
                begin: "+",
                fill: "-",
                sep: "+",
                end: "+",
            },
            linebelowheader: L::Static {
                begin: "|",
                fill: "-",
                sep: "+",
                end: "|",
            },
            linebetweenrows: None,
            linebelow: L::Static {
                begin: "+",
                fill: "-",
                sep: "+",
                end: "+",
            }
            .into(),
            headerrow: R::Static {
                begin: "|",
                sep: "|",
                end: "|",
            },
            datarow: R::Static {
                begin: "|",
                sep: "|",
                end: "|",
            },
            padding: 1,
            with_header_hide: &[],
        },
        TableFmt::Rst => FmtDef {
            lineabove: L::Static {
                begin: "",
                fill: "=",
                sep: "  ",
                end: "",
            },
            linebelowheader: L::Static {
                begin: "",
                fill: "=",
                sep: "  ",
                end: "",
            },
            linebetweenrows: None,
            linebelow: L::Static {
                begin: "",
                fill: "=",
                sep: "  ",
                end: "",
            }
            .into(),
            headerrow: R::Static {
                begin: "",
                sep: "  ",
                end: "",
            },
            datarow: R::Static {
                begin: "",
                sep: "  ",
                end: "",
            },
            padding: 0,
            with_header_hide: &[],
        },
        TableFmt::Mediawiki => FmtDef {
            lineabove: L::Text(
                "{| class=\"wikitable\" style=\"text-align: left;\"\n|+ <!-- caption -->\n|-",
            ),
            linebelowheader: L::Text("|-"),
            linebetweenrows: L::Text("|-").into(),
            linebelow: L::Text("|}").into(),
            headerrow: R::MediaWiki { sep: '!' },
            datarow: R::MediaWiki { sep: '|' },
            padding: 0,
            with_header_hide: &[],
        },
        TableFmt::Moinmoin => FmtDef {
            lineabove: L::None,
            linebelowheader: L::None,
            linebetweenrows: None,
            linebelow: None,
            headerrow: R::Moin {
                tag: "||",
                header: "'''",
            },
            datarow: R::Moin {
                tag: "||",
                header: "",
            },
            padding: 1,
            with_header_hide: &[],
        },
        TableFmt::Youtrack => FmtDef {
            lineabove: L::None,
            linebelowheader: L::None,
            linebetweenrows: None,
            linebelow: None,
            headerrow: R::Static {
                begin: "|| ",
                sep: " || ",
                end: " || ",
            },
            datarow: R::Static {
                begin: "| ",
                sep: " | ",
                end: " |",
            },
            padding: 1,
            with_header_hide: &[],
        },
        TableFmt::Html => FmtDef {
            lineabove: L::HtmlBegin,
            linebelowheader: L::None,
            linebetweenrows: None,
            linebelow: L::Static {
                begin: "</tbody>\n</table>",
                fill: "",
                sep: "",
                end: "",
            }
            .into(),
            headerrow: R::Html {
                tag: "th",
                unsafe_: false,
            },
            datarow: R::Html {
                tag: "td",
                unsafe_: false,
            },
            padding: 0,
            with_header_hide: &["lineabove"],
        },
        TableFmt::UnsafeHtml => FmtDef {
            lineabove: L::HtmlBegin,
            linebelowheader: L::None,
            linebetweenrows: None,
            linebelow: L::Static {
                begin: "</tbody>\n</table>",
                fill: "",
                sep: "",
                end: "",
            }
            .into(),
            headerrow: R::Html {
                tag: "th",
                unsafe_: true,
            },
            datarow: R::Html {
                tag: "td",
                unsafe_: true,
            },
            padding: 0,
            with_header_hide: &["lineabove"],
        },
        TableFmt::Latex => latex_def(false, false, false),
        TableFmt::LatexRaw => latex_def(true, false, false),
        TableFmt::LatexBooktabs => latex_def(false, true, false),
        TableFmt::LatexLongtable => latex_def(false, false, true),
        TableFmt::Tsv => FmtDef {
            lineabove: L::None,
            linebelowheader: L::None,
            linebetweenrows: None,
            linebelow: None,
            headerrow: R::Static {
                begin: "",
                sep: "\t",
                end: "",
            },
            datarow: R::Static {
                begin: "",
                sep: "\t",
                end: "",
            },
            padding: 0,
            with_header_hide: &[],
        },
        TableFmt::Textile => FmtDef {
            lineabove: L::None,
            linebelowheader: L::None,
            linebetweenrows: None,
            linebelow: None,
            headerrow: R::Static {
                begin: "|_. ",
                sep: "|_.",
                end: "|",
            },
            datarow: R::Textile,
            padding: 1,
            with_header_hide: &[],
        },
        TableFmt::Asciidoc => FmtDef {
            lineabove: L::AsciiLine,
            linebelowheader: L::None,
            linebetweenrows: None,
            linebelow: L::Static {
                begin: "|====",
                fill: "",
                sep: "",
                end: "",
            }
            .into(),
            headerrow: R::AsciiData { header: true },
            datarow: R::AsciiData { header: false },
            padding: 1,
            with_header_hide: &["lineabove"],
        },
    }
}

fn grid_def(
    b: &'static str,
    f: &'static str,
    s: &'static str,
    e: &'static str,
    rb: &'static str,
    rs: &'static str,
    re_: &'static str,
) -> FmtDef {
    FmtDef {
        lineabove: LineDef::Static {
            begin: b,
            fill: f,
            sep: s,
            end: e,
        },
        linebelowheader: LineDef::Static {
            begin: b,
            fill: "=",
            sep: s,
            end: e,
        },
        // Every grid-family format draws between-rows and bottom lines;
        // fancy variants override the chars via `with_*`.
        linebetweenrows: LineDef::Static {
            begin: b,
            fill: f,
            sep: s,
            end: e,
        }
        .into(),
        linebelow: LineDef::Static {
            begin: b,
            fill: f,
            sep: s,
            end: e,
        }
        .into(),
        headerrow: RowDef::Static {
            begin: rb,
            sep: rs,
            end: re_,
        },
        datarow: RowDef::Static {
            begin: rb,
            sep: rs,
            end: re_,
        },
        padding: 1,
        with_header_hide: &[],
    }
}

impl FmtDef {
    fn with_between(
        mut self,
        b: &'static str,
        f: &'static str,
        s: &'static str,
        e: &'static str,
    ) -> Self {
        self.linebetweenrows = Some(LineDef::Static {
            begin: b,
            fill: f,
            sep: s,
            end: e,
        });
        self
    }
    fn with_below(
        mut self,
        b: &'static str,
        f: &'static str,
        s: &'static str,
        e: &'static str,
    ) -> Self {
        self.linebelow = Some(LineDef::Static {
            begin: b,
            fill: f,
            sep: s,
            end: e,
        });
        self
    }
    fn with_header_line(
        mut self,
        b: &'static str,
        f: &'static str,
        s: &'static str,
        e: &'static str,
    ) -> Self {
        self.linebelowheader = LineDef::Static {
            begin: b,
            fill: f,
            sep: s,
            end: e,
        };
        self
    }
}

#[allow(clippy::too_many_arguments)]
fn outline_def(
    ab: &'static str,
    af: &'static str,
    as_: &'static str,
    ae: &'static str,
    hb: &'static str,
    hf: &'static str,
    hs: &'static str,
    he: &'static str,
    bb: &'static str,
    bf: &'static str,
    bs: &'static str,
    be: &'static str,
    rb: &'static str,
    rs: &'static str,
    re_: &'static str,
) -> FmtDef {
    FmtDef {
        lineabove: LineDef::Static {
            begin: ab,
            fill: af,
            sep: as_,
            end: ae,
        },
        linebelowheader: LineDef::Static {
            begin: hb,
            fill: hf,
            sep: hs,
            end: he,
        },
        linebetweenrows: None,
        linebelow: Some(LineDef::Static {
            begin: bb,
            fill: bf,
            sep: bs,
            end: be,
        }),
        headerrow: RowDef::Static {
            begin: rb,
            sep: rs,
            end: re_,
        },
        datarow: RowDef::Static {
            begin: rb,
            sep: rs,
            end: re_,
        },
        padding: 1,
        with_header_hide: &[],
    }
}

fn latex_def(raw: bool, booktabs: bool, longtable: bool) -> FmtDef {
    FmtDef {
        lineabove: LineDef::LatexBegin {
            booktabs,
            longtable,
        },
        linebelowheader: if booktabs {
            LineDef::Text("\\midrule")
        } else if longtable {
            LineDef::Text("\\hline\n\\endhead")
        } else {
            LineDef::Text("\\hline")
        },
        linebetweenrows: None,
        linebelow: if booktabs {
            Some(LineDef::Text("\\bottomrule\n\\end{tabular}"))
        } else if longtable {
            Some(LineDef::Text("\\hline\n\\end{longtable}"))
        } else {
            Some(LineDef::Text("\\hline\n\\end{tabular}"))
        },
        headerrow: RowDef::Latex { raw },
        datarow: RowDef::Latex { raw },
        padding: 1,
        with_header_hide: &[],
    }
}

// ---------------------------------------------------------------------------
// _format_table
// ---------------------------------------------------------------------------

/// Build one line (`U:2623 _build_line`).
fn build_line(
    colwidths: &[i64],
    colaligns: &[Align],
    line: &LineDef,
) -> Result<Option<String>, Fallback> {
    match line {
        LineDef::None => Ok(None),
        LineDef::Static {
            begin,
            fill,
            sep,
            end,
        } => {
            // Python `"x" * negative == ""`.
            let cells: Vec<String> = colwidths
                .iter()
                .map(|w| {
                    if *w <= 0 {
                        String::new()
                    } else {
                        fill.repeat(*w as usize)
                    }
                })
                .collect();
            Ok(Some(build_simple_row(&cells, begin, sep, end)))
        }
        LineDef::Pipe => Ok(Some(pipe_line(colwidths, colaligns))),
        LineDef::Grid => Ok(Some(grid_line(colwidths, colaligns))),
        LineDef::LatexBegin {
            booktabs,
            longtable,
        } => Ok(Some(latex_begin(colaligns, *booktabs, *longtable))),
        LineDef::HtmlBegin => Ok(Some("<table>\n<tbody>".to_string())),
        LineDef::AsciiLine => {
            // `partial(_asciidoc_row, False)` with 2 args.
            Ok(Some(asciidoc_head(false, colwidths, colaligns)?))
        }
        LineDef::Text(t) => Ok(Some(t.to_string())),
    }
}

/// Build one row (`U:2572 _build_row`).
fn build_row(
    padded_cells: &[String],
    colwidths: &[i64],
    colaligns: &[Align],
    row: &RowDef,
) -> Result<Option<String>, Fallback> {
    match row {
        RowDef::Static { begin, sep, end } => {
            Ok(Some(build_simple_row(padded_cells, begin, sep, end)))
        }
        RowDef::MediaWiki { sep } => Ok(Some(mediawiki_row(*sep, padded_cells, colaligns))),
        RowDef::Textile => Ok(Some(textile_row(padded_cells, colaligns))),
        RowDef::Html { tag, unsafe_ } => Ok(Some(html_row(tag, *unsafe_, padded_cells, colaligns))),
        RowDef::Moin { tag, header } => Ok(Some(moin_row(tag, padded_cells, colaligns, header))),
        RowDef::Latex { raw } => Ok(Some(latex_row(padded_cells, *raw))),
        RowDef::AsciiData { header } => Ok(Some(asciidoc_data(
            *header,
            padded_cells,
            colwidths,
            colaligns,
        )?)),
    }
}

/// `U:2653 _format_table`.
#[allow(clippy::too_many_arguments)]
pub fn format_table(
    fmt: TableFmt,
    headers: &[String],
    aligns_headers: &[Align],
    rows: &[Row],
    colwidths: &[i64],
    colaligns: &[Align],
    is_multiline: bool,
    rowaligns: &[RowAlign],
    _width_mode: &WidthMode,
) -> Result<String, Fallback> {
    let def = fmt_def(fmt);
    // `U:2658`: hidden elements only when headers exist.
    let hidden: &[&str] = if !headers.is_empty() {
        def.with_header_hide
    } else {
        &[]
    };
    let pad = def.padding;
    let padded_widths: Vec<i64> = colwidths.iter().map(|w| w + 2 * pad as i64).collect();
    // Unpadded widths for multiline rows (`U:2602 colwidths = w - 2*pad`).
    let unpadded: Vec<i64> = padded_widths.iter().map(|w| w - 2 * pad as i64).collect();

    let mut lines: Vec<String> = Vec::new();

    // `U:2674`: lineabove.
    if !matches!(def.lineabove, LineDef::None) && !hidden.contains(&"lineabove") {
        if let Some(l) = build_line(&padded_widths, colaligns, &def.lineabove)? {
            lines.push(l);
        }
    }

    if !headers.is_empty() {
        // `U:2678`: header row.
        append_row(
            &mut lines,
            headers,
            &padded_widths,
            aligns_headers,
            &def.headerrow,
            is_multiline,
            pad,
            RowAlign::Top,
        )?;
        // `U:2679`: linebelowheader.
        if !matches!(def.linebelowheader, LineDef::None) && !hidden.contains(&"linebelowheader") {
            if let Some(l) = build_line(&padded_widths, colaligns, &def.linebelowheader)? {
                lines.push(l);
            }
        }
    }

    if !rows.is_empty() && def.linebetweenrows.is_some() && !hidden.contains(&"linebetweenrows") {
        // `U:2682`: every row (but the last) followed by a line.
        let between = def.linebetweenrows.clone().unwrap();
        let n = rows.len();
        for (idx, row) in rows.iter().enumerate() {
            let last = idx + 1 == n;
            if !last {
                // `U:2685`: `if row != SEPARATING_LINE`.
                if matches!(row, Row::Data(_)) {
                    let ralign = rowaligns.get(idx).copied().unwrap_or(RowAlign::Top);
                    append_row_data(
                        &mut lines,
                        row,
                        &padded_widths,
                        &unpadded,
                        colaligns,
                        &def.datarow,
                        is_multiline,
                        pad,
                        ralign,
                    )?;
                }
                if let Some(l) = build_line(&padded_widths, colaligns, &between)? {
                    lines.push(l);
                }
            } else {
                let ralign = rowaligns.last().copied().unwrap_or(RowAlign::Top);
                append_row_data(
                    &mut lines,
                    row,
                    &padded_widths,
                    &unpadded,
                    colaligns,
                    &def.datarow,
                    is_multiline,
                    pad,
                    ralign,
                )?;
            }
        }
    } else {
        // `U:2705`: separating-line selection.
        let separating: LineDef = def
            .linebetweenrows
            .clone()
            .or_else(|| match &def.linebelowheader {
                LineDef::None => None,
                l => Some(l.clone()),
            })
            .or_else(|| def.linebelow.clone())
            .or_else(|| match &def.lineabove {
                LineDef::None => None,
                l => Some(l.clone()),
            })
            .unwrap_or(LineDef::Static {
                begin: "",
                fill: "",
                sep: "",
                end: "",
            });
        for row in rows {
            match row {
                Row::Sep => {
                    // `U:2715`: `_is_separating_line(row)` on the
                    // reinserted marker.
                    if let Some(l) = build_line(&padded_widths, colaligns, &separating)? {
                        lines.push(l);
                    }
                }
                Row::Data(cells) => {
                    // `U:2715`: a *data* row whose first two padded
                    // cells strip to `\x01` (e.g. missingval="\x01")
                    // draws a separating line too.
                    if is_separating_data(cells) {
                        if let Some(l) = build_line(&padded_widths, colaligns, &separating)? {
                            lines.push(l);
                        }
                    } else {
                        append_row_data(
                            &mut lines,
                            row,
                            &padded_widths,
                            &unpadded,
                            colaligns,
                            &def.datarow,
                            is_multiline,
                            pad,
                            RowAlign::Top,
                        )?;
                    }
                }
            }
        }
    }

    // `U:2722`: linebelow.
    if let Some(below) = &def.linebelow {
        if !hidden.contains(&"linebelow") {
            if let Some(l) = build_line(&padded_widths, colaligns, below)? {
                lines.push(l);
            }
        }
    }

    // `U:2725`.
    if headers.is_empty() && rows.is_empty() {
        Ok(String::new())
    } else {
        Ok(lines.join("\n"))
    }
}

/// `U:2715` on a padded data row.
fn is_separating_data(cells: &[String]) -> bool {
    let v0 = cells
        .first()
        .map(|s| ansi::strip(s) == "\x01")
        .unwrap_or(false);
    let v1 = cells
        .get(1)
        .map(|s| ansi::strip(s) == "\x01")
        .unwrap_or(false);
    v0 || v1
}

#[allow(clippy::too_many_arguments)]
fn append_row(
    lines: &mut Vec<String>,
    cells: &[String],
    padded_widths: &[i64],
    aligns: &[Align],
    row: &RowDef,
    is_multiline: bool,
    pad: usize,
    ralign: RowAlign,
) -> Result<(), Fallback> {
    let data = Row::Data(cells.to_vec());
    append_row_data(
        lines,
        &data,
        padded_widths,
        padded_widths,
        aligns,
        row,
        is_multiline,
        pad,
        ralign,
    )
}

#[allow(clippy::too_many_arguments)]
fn append_row_data(
    lines: &mut Vec<String>,
    row: &Row,
    padded_widths: &[i64],
    unpadded: &[i64],
    colaligns: &[Align],
    rowdef: &RowDef,
    is_multiline: bool,
    pad: usize,
    ralign: RowAlign,
) -> Result<(), Fallback> {
    let cells: &[String] = match row {
        Row::Data(c) => c,
        Row::Sep => return Ok(()),
    };
    if !is_multiline {
        // `U:2582 _append_basic_row`.
        let padded = pad_row(cells, pad);
        if let Some(s) = build_row(&padded, padded_widths, colaligns, rowdef)? {
            lines.push(s);
        }
        return Ok(());
    }
    // `U:2601 _append_multiline_row`.
    let cells_lines: Vec<Vec<&str>> = cells.iter().map(|c| ansi::splitlines_py(c)).collect();
    let nlines = cells_lines.iter().map(Vec::len).max().unwrap_or(0);
    if nlines == 0 {
        return Ok(());
    }
    let mut aligned: Vec<Vec<String>> = Vec::with_capacity(cells_lines.len());
    for (cl, w) in cells_lines.iter().zip(unpadded.iter()) {
        aligned.push(align_cell_vertically(
            cl.iter().map(|s| s.to_string()).collect(),
            nlines,
            *w,
            ralign,
        ));
    }
    for i in 0..nlines {
        let ln: Vec<String> = aligned.iter().map(|cl| cl[i].clone()).collect();
        let padded_ln = pad_row(&ln, pad);
        if let Some(s) = build_row(&padded_ln, unpadded, colaligns, rowdef)? {
            lines.push(s);
        }
    }
    Ok(())
}
