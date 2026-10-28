//! `tabulate-core`: pure-Rust port of the `astanin/python-tabulate`
//! formatting kernel (`tabulate/__init__.py`, MIT).
//!
//! Scope: everything downstream of input normalization —
//! per-cell type sniffing (`_type`/`_isconvertible`), column measurement,
//! numeric formatting, alignment, and layout for all 26 named table
//! formats. Input normalization (dicts/numpy/pandas/dataclasses,
//! `showindex`, headers handling), text wrapping (`maxcolwidths`), and the
//! CLI stay in Python. Anything the kernel cannot represent exactly
//! (exotic cell types, i64-overflowing ints, unsupported format specs,
//! user `TableFormat` objects) is reported as [`Fallback`] so the caller
//! can run the vendored pure-Python implementation instead: identical
//! output by construction, slower path only.
//!
//! Upstream line references (`U:<line>`) point at
//! `tabulate/__init__.py` 0.10.0.

pub mod align;
pub mod ansi;
pub mod layout;
pub mod numfmt;
pub mod sniff;
pub mod width;
mod width_tables;

use std::borrow::Cow;

/// Bail out to the pure-Python implementation. Carries no data: the
/// Python caller re-runs the vendored `tabulate()` with the original
/// arguments, which reproduces upstream behavior (including errors)
/// exactly.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Fallback;

/// A normalized input cell. Only exact Python types are represented;
/// subclasses, numpy scalars, datetimes, Fractions, and arbitrary
/// objects make the binding report [`Fallback`].
#[derive(Debug, Clone, Copy)]
pub enum Cell<'a> {
    Null,
    Bool(bool),
    Int(i64),
    Float(f64),
    Str(&'a str),
    Bytes(&'a [u8]),
}

/// Deduced column type (`U:1299 _column_type`, `U:988 _type`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColType {
    None,
    Bool,
    Int,
    Float,
    Bytes,
    Str,
}

/// A resolved per-column (or header) alignment.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Align {
    Left,
    Right,
    Center,
    Decimal,
    /// Upstream `"default"` (resolved against the format later).
    Default,
    /// `None`: no alignment, no whitespace stripping (`_padnone`).
    None_,
    /// Any other value: left-flushed with stripping (`_padright`).
    Other,
}

impl Align {
    pub fn from_name(s: &str) -> Align {
        match s {
            "left" => Align::Left,
            "right" => Align::Right,
            "center" => Align::Center,
            "decimal" => Align::Decimal,
            "default" => Align::Default,
            _ => Align::Other,
        }
    }

    fn or_default(self, dflt: Align) -> Align {
        match self {
            Align::Default => dflt,
            other => other,
        }
    }
}

/// One of the 26 named table formats (`U:362 _table_formats`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TableFmt {
    Simple,
    Plain,
    Grid,
    SimpleGrid,
    RoundedGrid,
    HeavyGrid,
    MixedGrid,
    DoubleGrid,
    FancyGrid,
    ColonGrid,
    Outline,
    SimpleOutline,
    RoundedOutline,
    HeavyOutline,
    MixedOutline,
    DoubleOutline,
    FancyOutline,
    Github,
    Pipe,
    Orgtbl,
    Jira,
    Presto,
    Pretty,
    Psql,
    Rst,
    Mediawiki,
    Moinmoin,
    Youtrack,
    Html,
    UnsafeHtml,
    Latex,
    LatexRaw,
    LatexBooktabs,
    LatexLongtable,
    Tsv,
    Textile,
    Asciidoc,
}

impl TableFmt {
    pub fn from_name(s: &str) -> Option<TableFmt> {
        Some(match s {
            "simple" => TableFmt::Simple,
            "plain" => TableFmt::Plain,
            "grid" => TableFmt::Grid,
            "simple_grid" => TableFmt::SimpleGrid,
            "rounded_grid" => TableFmt::RoundedGrid,
            "heavy_grid" => TableFmt::HeavyGrid,
            "mixed_grid" => TableFmt::MixedGrid,
            "double_grid" => TableFmt::DoubleGrid,
            "fancy_grid" => TableFmt::FancyGrid,
            "colon_grid" => TableFmt::ColonGrid,
            "outline" => TableFmt::Outline,
            "simple_outline" => TableFmt::SimpleOutline,
            "rounded_outline" => TableFmt::RoundedOutline,
            "heavy_outline" => TableFmt::HeavyOutline,
            "mixed_outline" => TableFmt::MixedOutline,
            "double_outline" => TableFmt::DoubleOutline,
            "fancy_outline" => TableFmt::FancyOutline,
            "github" => TableFmt::Github,
            "pipe" => TableFmt::Pipe,
            "orgtbl" => TableFmt::Orgtbl,
            "jira" => TableFmt::Jira,
            "presto" => TableFmt::Presto,
            "pretty" => TableFmt::Pretty,
            "psql" => TableFmt::Psql,
            "rst" => TableFmt::Rst,
            "mediawiki" => TableFmt::Mediawiki,
            "moinmoin" => TableFmt::Moinmoin,
            "youtrack" => TableFmt::Youtrack,
            "html" => TableFmt::Html,
            "unsafehtml" => TableFmt::UnsafeHtml,
            "latex" => TableFmt::Latex,
            "latex_raw" => TableFmt::LatexRaw,
            "latex_booktabs" => TableFmt::LatexBooktabs,
            "latex_longtable" => TableFmt::LatexLongtable,
            "tsv" => TableFmt::Tsv,
            "textile" => TableFmt::Textile,
            "asciidoc" => TableFmt::Asciidoc,
            _ => return None,
        })
    }

    /// Formats that fold multiline cells into subsequent rows
    /// (`U:746 multiline_formats`).
    pub fn is_multiline_capable(self) -> bool {
        !matches!(
            self,
            TableFmt::Mediawiki
                | TableFmt::Moinmoin
                | TableFmt::Youtrack
                | TableFmt::Html
                | TableFmt::UnsafeHtml
                | TableFmt::Latex
                | TableFmt::LatexRaw
                | TableFmt::LatexBooktabs
                | TableFmt::LatexLongtable
                | TableFmt::Tsv
                | TableFmt::Textile
                | TableFmt::Asciidoc
        )
    }

    pub fn is_html(self) -> bool {
        matches!(self, TableFmt::Html | TableFmt::UnsafeHtml)
    }
}

/// Whole-argument `colalign` (`U:2431`): either a string (warn, then
/// iterate its characters) or a list of per-column entries.
#[derive(Debug, Clone)]
pub enum ColAlignArg {
    Str(String),
    List(Vec<ColAlign>),
}

/// One `colalign` list element: strings (incl. unknown ones), `None`,
/// or anything else.
#[derive(Debug, Clone)]
pub enum ColAlign {
    Str(String),
    None_,
    Other,
}

/// Whole-argument `headersalign` (`U:2475`).
#[derive(Debug, Clone)]
pub enum HeadersAlignArg {
    Str(String),
    List(Vec<HeaderAlign>),
}

/// One `headersalign` list element.
#[derive(Debug, Clone)]
pub enum HeaderAlign {
    Str(String),
    None_,
    Other,
}

/// One `rowalign` element (`U:2588 _align_cell_veritically`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RowAlign {
    Bottom,
    Center,
    Top,
}

impl RowAlign {
    pub fn from_name(s: &str) -> RowAlign {
        match s {
            "bottom" => RowAlign::Bottom,
            "center" => RowAlign::Center,
            _ => RowAlign::Top,
        }
    }
}

/// One normalized input cell with its pre-resolved CPython answers for
/// non-ASCII strings (see [`sniff::DelegatedNum`]).
#[derive(Debug, Clone, Copy)]
pub struct InputCell<'a> {
    pub cell: Cell<'a>,
    pub delegated: Option<sniff::DelegatedNum>,
}

/// Kernel input: normalized rows plus every `tabulate()` scalar option.
/// The Python binding fills this in; anything inexpressible is
/// [`Fallback`] raised during extraction.
pub struct KernelInput<'a> {
    pub rows: Vec<Vec<InputCell<'a>>>,
    pub headers: Vec<Cow<'a, str>>,
    pub headers_pad: usize,
    pub separating: Vec<usize>,
    pub fmt: TableFmt,
    pub floatfmt: Vec<String>,
    pub intfmt: Vec<String>,
    pub missingval: Vec<String>,
    pub numparses: Vec<bool>,
    pub numalign: Align,
    pub stralign: Align,
    /// `None` = option not provided. `Other` covers non-str values.
    pub colglobalalign: Option<Align>,
    pub colalign: ColAlignArg,
    pub headersglobalalign: Option<Align>,
    pub headersalign: HeadersAlignArg,
    pub rowalign: Vec<RowAlign>,
    pub rowalign_default: RowAlign,
    pub preserve_whitespace: bool,
    pub min_padding: usize,
    pub wide: bool,
}

/// Kernel output: rendered text, whether it needs the `JupyterHTMLStr`
/// wrapper (`U:2725`), and the raw strings to warn about (the shim
/// formats the exact messages) at the upstream points (`U:2434`, `U:2477`).
pub struct KernelOutput {
    pub text: String,
    pub is_html: bool,
    pub warn_colalign: Option<String>,
    pub warn_headersalign: Option<String>,
}

/// Run the formatting kernel (`U:2295-2521`, post-normalization body of
/// `tabulate()`).
pub fn tabulate(mut input: KernelInput) -> Result<KernelOutput, Fallback> {
    let fmt = input.fmt;

    // `U:2336-2339` rst first-column escape.
    if fmt == TableFmt::Rst {
        layout::rst_escape_first_column(&mut input.rows, &mut input.headers);
    }

    // `U:2345-2360` pretty/colon_grid tweaks + align defaults.
    let (numalign, stralign) = if fmt == TableFmt::Pretty {
        (
            input.numalign.or_default(Align::Center),
            input.stralign.or_default(Align::Center),
        )
    } else {
        (
            input.numalign.or_default(Align::Decimal),
            input.stralign.or_default(Align::Left),
        )
    };
    let min_padding = if fmt == TableFmt::Pretty {
        0
    } else {
        input.min_padding
    };
    let (mut colglobalalign, mut headersglobalalign) = (
        input.colglobalalign.clone(),
        input.headersglobalalign.clone(),
    );
    if fmt == TableFmt::ColonGrid {
        colglobalalign = Some(Align::Left);
        headersglobalalign = Some(Align::Left);
    }

    // `U:2362-2389` plain-text scan: ANSI detection, wide chars, multiline.
    let has_invisible = sniff::detect_invisible(&input.rows, &input.headers);
    let is_multiline =
        fmt.is_multiline_capable() && sniff::detect_multiline(&input.rows, &input.headers);
    let width_mode = width::WidthMode::choose(has_invisible, input.wide, is_multiline)?;

    // `U:2392-2422` transpose + column types + per-column formatting.
    let ncols = input.rows.iter().map(|r| r.len()).max().unwrap_or(0);
    let mut coltypes: Vec<ColType> = Vec::with_capacity(ncols);
    for c in 0..ncols {
        let mut t = ColType::None;
        for row in &input.rows {
            let ic = row.get(c).copied().unwrap_or(InputCell {
                cell: Cell::Null,
                delegated: None,
            });
            let np = input.numparses.get(c).copied().unwrap_or(true);
            t = sniff::more_generic(
                t,
                sniff::cell_type(ic.cell, has_invisible, np, ic.delegated)?,
            );
        }
        coltypes.push(t);
    }
    let float_specs = numfmt::parse_specs(&input.floatfmt)?;
    let int_specs = numfmt::parse_specs(&input.intfmt)?;
    let missing_vals = expand_missing(&input.missingval, ncols);

    // Format every cell to its display string.
    let mut cols: Vec<Vec<String>> = Vec::with_capacity(ncols);
    for (c, ct) in coltypes.iter().enumerate() {
        let fspec = float_specs
            .get(c)
            .copied()
            .unwrap_or(numfmt::Spec::default_float());
        let ispec = int_specs
            .get(c)
            .copied()
            .unwrap_or(numfmt::Spec::default_int());
        let mut col = Vec::with_capacity(input.rows.len());
        for row in &input.rows {
            let ic = row.get(c).copied().unwrap_or(InputCell {
                cell: Cell::Null,
                delegated: None,
            });
            let cell = ic.cell;
            col.push(sniff::format_cell(
                cell,
                *ct,
                &fspec,
                &ispec,
                missing_vals.get(c).map(String::as_str).unwrap_or(""),
                has_invisible,
            )?);
        }
        cols.push(col);
    }

    // `U:2424-2443` column alignment resolution (+colalign warning).
    let mut warn_colalign: Option<String> = None;
    let mut aligns: Vec<Align> = Vec::with_capacity(ncols);
    for ct in &coltypes {
        if let Some(g) = &colglobalalign {
            aligns.push(g.clone());
        } else if *ct == ColType::Int || *ct == ColType::Float {
            aligns.push(numalign.clone());
        } else {
            aligns.push(stralign.clone());
        }
    }
    match &input.colalign {
        // `U:2433-2443`: a string colalign warns, then its characters
        // override per-column.
        ColAlignArg::Str(s) => {
            warn_colalign = Some(s.clone());
            for (idx, ch) in s.chars().enumerate() {
                if idx >= aligns.len() {
                    break;
                }
                let one = ch.to_string();
                if one != "global" {
                    aligns[idx] = Align::from_name(&one);
                }
            }
        }
        ColAlignArg::List(items) => {
            for (idx, a) in items.iter().enumerate() {
                if idx >= aligns.len() {
                    break;
                }
                match a {
                    ColAlign::Str(s) if s == "global" => {}
                    ColAlign::Str(s) => aligns[idx] = Align::from_name(s),
                    ColAlign::None_ => aligns[idx] = Align::None_,
                    ColAlign::Other => aligns[idx] = Align::Other,
                }
            }
        }
    }

    // `U:2444-2463` widths + column alignment.
    let headers: Vec<String> = input.headers.iter().map(|h| h.to_string()).collect();
    let mut minwidths: Vec<i64> = if !headers.is_empty() {
        headers
            .iter()
            .map(|h| width_mode.width(h) + min_padding as i64)
            .collect()
    } else {
        vec![0; ncols]
    };
    // A negative `minwidth` (control-char header under `pretty`, whose
    // padding is 0) makes upstream raise `ValueError` from the pad
    // functions; bail so the fallback replays it exactly.
    if minwidths.iter().any(|&w| w < 0) {
        return Err(Fallback);
    }
    let mut aligns_for_data = aligns.clone();
    if fmt == TableFmt::ColonGrid {
        aligns_for_data = vec![Align::Left; aligns_for_data.len()];
    }
    // `zip` truncation is load-bearing (`U:2452`): ragged rows with a
    // short first row + headers drop trailing columns, exactly like
    // upstream.
    let n_aligned = minwidths.len().min(aligns_for_data.len()).min(cols.len());
    let cols: Vec<Vec<String>> = cols
        .into_iter()
        .zip(aligns_for_data.iter())
        .zip(minwidths.iter())
        .take(n_aligned)
        .map(|t| {
            align::align_column(
                &t.0 .0,
                t.0 .1,
                *t.1,
                &width_mode,
                input.preserve_whitespace,
            )
        })
        .collect::<Result<_, _>>()?;
    // NOTE: `aligns` and `minwidths` stay full-length here: `U:2512`
    // passes the untruncated lists to `_format_table` (latex `{lll}`
    // proves it); only the aligned *columns* truncate via the `take`
    // above, exactly like the `U:2452` zip.

    // `U:2465-2503` headers.
    let mut aligns_headers: Vec<Align> = Vec::new();
    let mut aligned_headers: Vec<String> = Vec::new();
    let mut rows: Vec<layout::Row> = Vec::new();
    let mut warn_headersalign: Option<String> = None;
    if !headers.is_empty() {
        let t_cols: Vec<Vec<String>> = if cols.is_empty() {
            vec![vec![String::new()]; headers.len()]
        } else {
            cols.clone()
        };
        if let Some(g) = &headersglobalalign {
            aligns_headers = vec![g.clone(); t_cols.len()];
        } else if aligns.is_empty() {
            aligns_headers = vec![stralign.clone(); headers.len()];
        } else {
            aligns_headers = aligns.clone();
        }
        match &input.headersalign {
            // `U:2477-2491`: string headersalign warns, then characters
            // override from `headers_pad`.
            HeadersAlignArg::Str(s) => {
                warn_headersalign = Some(s.clone());
                for (idx, ch) in s.chars().enumerate() {
                    let hidx = input.headers_pad + idx;
                    if hidx >= aligns_headers.len() {
                        break;
                    }
                    let one = ch.to_string();
                    if one == "same" {
                        if hidx < aligns.len() {
                            aligns_headers[hidx] = aligns[hidx].clone();
                        }
                    } else if one != "global" {
                        aligns_headers[hidx] = Align::from_name(&one);
                    }
                }
            }
            HeadersAlignArg::List(items) => {
                for (idx, a) in items.iter().enumerate() {
                    let hidx = input.headers_pad + idx;
                    if hidx >= aligns_headers.len() {
                        break;
                    }
                    match a {
                        HeaderAlign::Str(s) if s == "same" => {
                            if hidx < aligns.len() {
                                aligns_headers[hidx] = aligns[hidx].clone();
                            }
                        }
                        HeaderAlign::Str(s) if s == "global" => {}
                        HeaderAlign::Str(s) => aligns_headers[hidx] = Align::from_name(s),
                        HeaderAlign::None_ => aligns_headers[hidx] = Align::None_,
                        HeaderAlign::Other => aligns_headers[hidx] = Align::Other,
                    }
                }
            }
        }
        // `U:2492-2495`: column widths account for data cells too.
        for (minw, c) in minwidths.iter_mut().zip(t_cols.iter()) {
            let mut data_max = 0;
            for s in c.iter() {
                data_max = data_max.max(width_mode.width(s));
            }
            if data_max > *minw {
                *minw = data_max;
            }
        }
        for ((h, a), w) in headers
            .iter()
            .zip(aligns_headers.iter())
            .zip(minwidths.iter())
        {
            aligned_headers.push(align::align_header(
                h,
                a,
                *w,
                width_mode.width(h),
                is_multiline,
                &width_mode,
            )?);
        }
        // `U:2500` rows = transpose of aligned columns (`cols`, not the
        // width-only `t_cols` fallback above).
        let nrows = cols.first().map(Vec::len).unwrap_or(0);
        for r in 0..nrows {
            rows.push(layout::Row::Data(
                cols.iter().map(|c| c[r].clone()).collect(),
            ));
        }
    } else {
        // `U:2502` headerless: widths from data alone.
        for (minw, c) in minwidths.iter_mut().zip(cols.iter()) {
            let mut data_max = 0;
            for s in c.iter() {
                data_max = data_max.max(width_mode.width(s));
            }
            if data_max > *minw {
                *minw = data_max;
            }
        }
        let nrows = cols.first().map(Vec::len).unwrap_or(0);
        for r in 0..nrows {
            rows.push(layout::Row::Data(
                cols.iter().map(|c| c[r].clone()).collect(),
            ));
        }
    }
    // `aligns` is full-length by construction (see NOTE above).

    // `U:2508-2521` row alignment expansion, separator reinsertion, layout.
    // Expansion happens BEFORE reinsertion (`U:2509` uses the pre-insert
    // length); the layout zips, so length mismatch is exact, not an error.
    let rowaligns = expand_rowalign(&input.rowalign, input.rowalign_default, rows.len());
    for idx in &input.separating {
        if *idx <= rows.len() {
            rows.insert(*idx, layout::Row::Sep);
        }
    }
    let text = layout::format_table(
        fmt,
        &aligned_headers,
        &aligns_headers,
        &rows,
        &minwidths,
        &aligns,
        is_multiline,
        &rowaligns,
        &width_mode,
    )?;
    let is_html = fmt.is_html() && (!aligned_headers.is_empty() || !rows.is_empty());
    Ok(KernelOutput {
        text,
        is_html,
        warn_colalign,
        warn_headersalign,
    })
}

fn expand_rowalign(rowalign: &[RowAlign], dflt: RowAlign, n: usize) -> Vec<RowAlign> {
    // `U:2541 _expand_iterable` on a list: pad with the default.
    let mut out = rowalign.to_vec();
    out.resize(n, dflt);
    out.truncate(n);
    out
}

fn expand_missing(missingval: &[String], ncols: usize) -> Vec<String> {
    let mut out = missingval.to_vec();
    if out.len() < ncols {
        out.resize(ncols, String::new());
    }
    out
}
