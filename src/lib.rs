//! Python bindings for `tabulate-core` (PyO3).
//!
//! Seam convention: normalization (dicts/numpy/pandas/dataclasses,
//! `showindex`, headers, separating-line removal) stays in Python and
//! crosses as plain lists; the whole formatting kernel
//! (sniff → measure → layout) runs in Rust exactly once per call.
//! Anything the kernel cannot represent exactly raises [`Fallback`],
//! and the Python shim replays the vendored pure-Python
//! implementation: identical output by construction, slower path only.

use pyo3::exceptions::{PyAssertionError, PyIndexError, PyTypeError};
use pyo3::prelude::*;
use pyo3::types::{PyBool, PyBytes, PyFloat, PyInt, PyIterator, PyList, PyString};
use tabulate_core::{
    Align, Cell, ColAlign, ColAlignArg, HeaderAlign, HeadersAlignArg, InputCell, KernelInput,
    RowAlign, TableFmt,
};

pyo3::create_exception!(_core, FallbackError, pyo3::exceptions::PyException);

fn fb(msg: &str) -> PyErr {
    FallbackError::new_err(msg.to_string())
}

fn py_str(s: &Bound<'_, PyString>) -> PyResult<String> {
    s.to_str().map(|x| x.to_string())
}
fn extract_cell<'a>(
    obj: &Bound<'a, PyAny>,
    py_int: &Bound<'a, PyAny>,
    py_float: &Bound<'a, PyAny>,
) -> PyResult<InputCell<'a>> {
    if obj.is_none() {
        return Ok(InputCell {
            cell: Cell::Null,
            delegated: None,
        });
    }
    if obj.is_exact_instance_of::<PyBool>() {
        return Ok(InputCell {
            cell: Cell::Bool(obj.extract::<bool>()?),
            delegated: None,
        });
    }
    if obj.is_exact_instance_of::<PyInt>() {
        match obj.extract::<i64>() {
            Ok(v) => {
                return Ok(InputCell {
                    cell: Cell::Int(v),
                    delegated: None,
                })
            }
            // i64 overflow (or other exotic longs): the fallback
            // formats arbitrary-precision ints exactly.
            Err(_) => return Err(fb("int overflow")),
        }
    }
    if obj.is_exact_instance_of::<PyFloat>() {
        return Ok(InputCell {
            cell: Cell::Float(obj.extract::<f64>()?),
            delegated: None,
        });
    }
    if obj.is_exact_instance_of::<PyString>() {
        let bnd = obj.downcast::<PyString>()?;
        // SAFETY: `bnd` is an exact `str` (checked above). The UTF-8
        // cache pointer stays valid while the object lives (`'a`);
        // the GIL is held for the whole kernel call. Mirrors pyo3's
        // own `Borrowed::to_str`.
        let s: &str = unsafe {
            let mut size: pyo3::ffi::Py_ssize_t = 0;
            let ptr = pyo3::ffi::PyUnicode_AsUTF8AndSize(bnd.as_ptr(), &mut size) as *const u8;
            if ptr.is_null() {
                return Err(fb("utf8"));
            }
            let bytes = std::slice::from_raw_parts(ptr, size as usize);
            std::str::from_utf8(bytes).map_err(|_| fb("utf8"))?
        };
        let delegated = if s.is_ascii() {
            None
        } else {
            Some(delegate_numeric(obj, py_int, py_float)?)
        };
        return Ok(InputCell {
            cell: Cell::Str(s),
            delegated,
        });
    }
    if obj.is_exact_instance_of::<PyBytes>() {
        let bnd = obj.downcast::<PyBytes>()?;
        // SAFETY: `bnd` is an exact `PyBytes` (checked above), whose
        // buffer is immutable and lives as long as `obj` (`'a`). The
        // GIL is held for the whole kernel call, so no mutation or
        // deallocation can happen. This is exactly what pyo3's own
        // (pub(crate)) `Borrowed::as_bytes` does internally; the public
        // `Bound::as_bytes` cannot name this lifetime on 0.24.
        let b: &[u8] = unsafe {
            let ptr = pyo3::ffi::PyBytes_AsString(bnd.as_ptr()) as *const u8;
            let len = pyo3::ffi::PyBytes_Size(bnd.as_ptr()) as usize;
            std::slice::from_raw_parts(ptr, len)
        };
        return Ok(InputCell {
            cell: Cell::Bytes(b),
            delegated: None,
        });
    }
    Err(fb("exotic cell"))
}

/// Resolve `int(s)` / `float(s)` via CPython for a non-ASCII string
/// (unicode digits/whitespace). Exact by construction; runs only for
/// non-ASCII cells.
fn delegate_numeric(
    obj: &Bound<'_, PyAny>,
    py_int: &Bound<'_, PyAny>,
    py_float: &Bound<'_, PyAny>,
) -> PyResult<tabulate_core::sniff::DelegatedNum> {
    let int_ok = py_int.call1((obj,)).is_ok();
    let (float_ok, float_is_inf_or_nan) = match py_float.call1((obj,)) {
        Ok(v) => match v.extract::<f64>() {
            Ok(f) => (true, f.is_infinite() || f.is_nan()),
            Err(_) => (false, false),
        },
        Err(_) => (false, false),
    };
    Ok(tabulate_core::sniff::DelegatedNum {
        int_ok,
        float_ok,
        float_is_inf_or_nan,
    })
}

/// Map an alignment *value* (`numalign`, `colalign` elements, ...).
/// Falsy non-strings (`None`, `False`, `0`, `""`, `b""`) mean "no
/// alignment" (`U:1186`, `U:1405`); unknown strings are `Other`.
fn map_align(obj: &Bound<'_, PyAny>) -> PyResult<Align> {
    if let Ok(s) = obj.downcast::<PyString>() {
        let s = py_str(s)?;
        if s.is_empty() {
            return Ok(Align::None_);
        }
        return Ok(Align::from_name(&s));
    }
    if obj.is_none() {
        return Ok(Align::None_);
    }
    if !obj.is_truthy()? {
        return Ok(Align::None_);
    }
    Ok(Align::Other)
}

/// Whole-argument `colalign` (`U:2431`).
fn extract_colalign(obj: &Option<Bound<'_, PyAny>>) -> PyResult<ColAlignArg> {
    let Some(obj) = obj else {
        return Ok(ColAlignArg::List(Vec::new()));
    };
    if let Ok(s) = obj.downcast::<PyString>() {
        return Ok(ColAlignArg::Str(py_str(s)?));
    }
    let iter = obj
        .try_iter()
        .map_err(|_| PyAssertionError::new_err("isinstance(colalign, Iterable)"))?;
    let mut out = Vec::new();
    for item in iter {
        let item = item?;
        if let Ok(s) = item.downcast::<PyString>() {
            out.push(ColAlign::Str(py_str(s)?));
        } else if item.is_none() {
            out.push(ColAlign::None_);
        } else {
            out.push(ColAlign::Other);
        }
    }
    Ok(ColAlignArg::List(out))
}

/// Whole-argument `headersalign` (`U:2475`).
fn extract_headersalign(obj: &Option<Bound<'_, PyAny>>) -> PyResult<HeadersAlignArg> {
    let Some(obj) = obj else {
        return Ok(HeadersAlignArg::List(Vec::new()));
    };
    if let Ok(s) = obj.downcast::<PyString>() {
        return Ok(HeadersAlignArg::Str(py_str(s)?));
    }
    let iter = obj
        .try_iter()
        .map_err(|_| PyAssertionError::new_err("isinstance(headersalign, Iterable)"))?;
    let mut out = Vec::new();
    for item in iter {
        let item = item?;
        if let Ok(s) = item.downcast::<PyString>() {
            out.push(HeaderAlign::Str(py_str(s)?));
        } else if item.is_none() {
            out.push(HeaderAlign::None_);
        } else {
            out.push(HeaderAlign::Other);
        }
    }
    Ok(HeadersAlignArg::List(out))
}

/// Whole-argument `rowalign`: `str` (scalar default), `list` (expand),
/// other iterables (upstream `TypeError`), other scalars (default).
fn extract_rowalign(obj: &Bound<'_, PyAny>) -> PyResult<(Vec<RowAlign>, RowAlign)> {
    if let Ok(s) = obj.downcast::<PyString>() {
        return Ok((Vec::new(), RowAlign::from_name(&py_str(s)?)));
    }
    if obj.is_instance_of::<PyList>() {
        let list = obj.downcast::<PyList>()?;
        let mut out = Vec::with_capacity(list.len());
        for item in list.iter() {
            if let Ok(s) = item.downcast::<PyString>() {
                out.push(RowAlign::from_name(&py_str(s)?));
            } else {
                out.push(RowAlign::Top);
            }
        }
        return Ok((out, RowAlign::Top));
    }
    // `U:2541`: tuples/dicts/generators fail on `original + [default]*n`.
    if obj.try_iter().is_ok() {
        return Err(fb("rowalign not a list"));
    }
    Ok((Vec::new(), RowAlign::Top))
}

/// Expand `floatfmt`/`intfmt`/`missingval` to per-column strings
/// (`U:2395-2422`). Non-str elements and non-iterables bail out so the
/// fallback reproduces the exact upstream error (or success).
fn expand_per_col(obj: &Bound<'_, PyAny>, ncols: usize, default: &str) -> PyResult<Vec<String>> {
    if let Ok(s) = obj.downcast::<PyString>() {
        return Ok(vec![py_str(s)?; ncols]);
    }
    let iter = PyIterator::from_object(obj).map_err(|_| fb("spec not iterable"))?;
    let mut out = Vec::new();
    for item in iter {
        let item = item.map_err(|_| fb("spec iter"))?;
        if let Ok(s) = item.downcast::<PyString>() {
            out.push(py_str(s)?);
        } else {
            return Err(fb("non-str spec"));
        }
    }
    if out.len() < ncols {
        out.resize(ncols, default.to_string());
    }
    Ok(out)
}

/// `_expand_numparse` (`U:2524`): bools/scalars negate; index lists
/// clear per column (`IndexError`/`TypeError` replicated exactly).
fn expand_numparse(obj: &Bound<'_, PyAny>, ncols: usize) -> PyResult<Vec<bool>> {
    match obj.try_iter() {
        Ok(iter) => {
            let mut numparses = vec![true; ncols];
            for item in iter {
                let item = item?;
                let idx: i64 = item.extract().map_err(|_| {
                    let tname = item
                        .get_type()
                        .name()
                        .map(|n| n.to_string())
                        .unwrap_or_else(|_| "unknown".to_string());
                    PyTypeError::new_err(format!(
                        "list indices must be integers or slices, not {tname}"
                    ))
                })?;
                // Negative wrap + bounds, exactly like list assignment.
                let n = ncols as i64;
                let pos = if idx < 0 { idx + n } else { idx };
                if pos < 0 || pos >= n {
                    return Err(PyIndexError::new_err("list assignment index out of range"));
                }
                numparses[pos as usize] = false;
            }
            Ok(numparses)
        }
        Err(_) => {
            // Non-iterable scalar (bool/int/None/float/...): `[not x]`.
            let truthy = obj.is_truthy()?;
            Ok(vec![!truthy; ncols])
        }
    }
}

/// Map a global alignment option (`None` = absent).
fn extract_global(obj: &Option<Bound<'_, PyAny>>) -> PyResult<Option<Align>> {
    match obj {
        None => Ok(None),
        Some(o) => Ok(Some(map_align(o)?)),
    }
}

#[allow(clippy::too_many_arguments)]
#[pyfunction]
#[pyo3(signature = (rows, headers, headers_pad, separating, tablefmt, floatfmt, intfmt, missingval, numalign, stralign, colglobalalign, colalign, headersglobalalign, headersalign, rowalign, disable_numparse, preserve_whitespace, min_padding, wide))]
fn tabulate_kernel(
    py: Python<'_>,
    rows: &Bound<'_, PyList>,
    headers: Vec<String>,
    headers_pad: usize,
    separating: Vec<usize>,
    tablefmt: &str,
    floatfmt: &Bound<'_, PyAny>,
    intfmt: &Bound<'_, PyAny>,
    missingval: &Bound<'_, PyAny>,
    numalign: &Bound<'_, PyAny>,
    stralign: &Bound<'_, PyAny>,
    colglobalalign: Option<Bound<'_, PyAny>>,
    colalign: Option<Bound<'_, PyAny>>,
    headersglobalalign: Option<Bound<'_, PyAny>>,
    headersalign: Option<Bound<'_, PyAny>>,
    rowalign: &Bound<'_, PyAny>,
    disable_numparse: &Bound<'_, PyAny>,
    preserve_whitespace: &Bound<'_, PyAny>,
    min_padding: Bound<'_, PyAny>,
    wide: bool,
) -> PyResult<(String, bool, Option<String>, Option<String>)> {
    let fmt = TableFmt::from_name(tablefmt).ok_or_else(|| fb("unknown tablefmt"))?;
    let builtins = py.import("builtins")?;
    let py_int = builtins.getattr("int")?;
    let py_float = builtins.getattr("float")?;

    // Rows + cells.
    let mut krows: Vec<Vec<InputCell>> = Vec::with_capacity(rows.len());
    for row in rows.iter() {
        let row = row.downcast::<PyList>().map_err(|_| fb("row not a list"))?;
        let mut krow = Vec::with_capacity(row.len());
        for item in row.iter() {
            krow.push(extract_cell(&item, &py_int, &py_float)?);
        }
        krows.push(krow);
    }
    let ncols = krows.iter().map(Vec::len).max().unwrap_or(0);

    let float_specs = expand_per_col(floatfmt, ncols, "g")?;
    let int_specs = expand_per_col(intfmt, ncols, "")?;
    let missing_vals = expand_per_col(missingval, ncols, "")?;
    // Validate specs eagerly (unsupported → fallback before any output).
    for s in float_specs.iter().chain(int_specs.iter()) {
        tabulate_core::numfmt::parse_spec(s).map_err(|_| fb("bad spec"))?;
    }
    // `U:2346-2348`: `pretty` forces `disable_numparse = True` before
    // the `U:2393` expansion.
    let numparses = if tablefmt == "pretty" {
        vec![false; ncols]
    } else {
        expand_numparse(disable_numparse, ncols)?
    };
    let (rowalign_list, rowalign_default) = extract_rowalign(rowalign)?;
    let min_padding: i64 = min_padding.extract().map_err(|_| fb("bad min_padding"))?;
    if min_padding < 0 {
        return Err(fb("negative min_padding"));
    }

    let input = KernelInput {
        rows: krows,
        headers: headers
            .iter()
            .map(|h| std::borrow::Cow::Borrowed(h.as_str()))
            .collect(),
        headers_pad,
        separating,
        fmt,
        floatfmt: float_specs,
        intfmt: int_specs,
        missingval: missing_vals,
        numparses,
        numalign: map_align(numalign)?,
        stralign: map_align(stralign)?,
        colglobalalign: extract_global(&colglobalalign)?,
        colalign: extract_colalign(&colalign)?,
        headersglobalalign: extract_global(&headersglobalalign)?,
        headersalign: extract_headersalign(&headersalign)?,
        rowalign: rowalign_list,
        rowalign_default,
        preserve_whitespace: preserve_whitespace.is_truthy()?,
        min_padding: min_padding as usize,
        wide,
    };
    match tabulate_core::tabulate(input) {
        Ok(out) => Ok((
            out.text,
            out.is_html,
            out.warn_colalign,
            out.warn_headersalign,
        )),
        Err(_) => Err(fb("kernel fallback")),
    }
}

#[pymodule]
fn _core(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(tabulate_kernel, m)?)?;
    m.add("Fallback", m.py().get_type::<FallbackError>())?;
    Ok(())
}
