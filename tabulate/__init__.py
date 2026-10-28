"""`tabulate`: pretty-print tabular data (Rust-accelerated mirror).

This package is a drop-in mirror of
[astanin/python-tabulate](https://github.com/astanin/python-tabulate)
(MIT, see NOTICE): the same import name, the same API, the same output.

How it works: :func:`tabulate` normalizes its input with the vendored
pure-Python implementation (:mod:`tabulate._pure`, byte-identical to the
upstream 0.10.0 ``__init__.py``) and then runs the whole formatting
kernel (type sniffing, column measurement, layout) in Rust
(:mod:`tabulate._core`). Anything the kernel cannot represent exactly
(exotic cell types, user ``TableFormat`` objects, ``maxcolwidths``,
unsupported format specs) falls back to the vendored implementation
for that call: identical output by construction, slower path only. The
pure-Python implementation stays importable without the extension.

Running this file as a script (``python tabulate/__init__.py``, as the
CLI tests do) uses the pure-Python path when the extension is absent.
"""

__all__ = ["tabulate", "tabulate_formats", "simple_separated_format"]

try:
    # Normal package import.
    from tabulate import _pure as _pure_module
    from tabulate._pure import tabulate as _pure_tabulate
    from tabulate._pure import (
        _normalize_tabular_data,
        _remove_separating_lines,
        _table_formats,
        tabulate_formats,
        simple_separated_format,
        SEPARATING_LINE,
        TableFormat,
        Line,
        DataRow,
        JupyterHTMLStr,
        _main,
        _pprint_file,
    )
    try:
        from tabulate._core import tabulate_kernel as _kernel
        from tabulate._core import Fallback as _Fallback
    except ImportError:
        _kernel = None
        _Fallback = None
except ImportError:
    # Running as a script: `python tabulate/__init__.py` (CLI tests).
    # `sys.path[0]` is this directory, so absolute imports apply, and
    # the extension may be absent (pure path only, same output).
    import _pure as _pure_module  # noqa: F401
    from _pure import tabulate as _pure_tabulate  # noqa: F401
    from _pure import (  # noqa: F401
        _normalize_tabular_data,
        _remove_separating_lines,
        _table_formats,
        tabulate_formats,
        simple_separated_format,
        SEPARATING_LINE,
        TableFormat,
        Line,
        DataRow,
        JupyterHTMLStr,
        _main,
        _pprint_file,
    )
    try:
        from _core import tabulate_kernel as _kernel  # noqa: F401
        from _core import Fallback as _Fallback  # noqa: F401
    except ImportError:
        _kernel = None
        _Fallback = None

import warnings as _warnings

try:
    from importlib.metadata import version as _version_of
except ImportError:  # pragma: no cover (py3.8+ always has it)
    _version_of = None


def _dist_version():
    if _version_of is None:
        return "unknown"
    try:
        return _version_of("tabulate-rust")
    except Exception:
        return "unknown"


__version__ = _dist_version()
_pure_module.__version__ = __version__

# Names of the 26 formats the Rust kernel implements (mirrors
# `_table_formats` keys minus user-registered entries, which fall back).
_KNOWN_FORMATS = frozenset(
    [
        "simple",
        "plain",
        "grid",
        "simple_grid",
        "rounded_grid",
        "heavy_grid",
        "mixed_grid",
        "double_grid",
        "fancy_grid",
        "colon_grid",
        "outline",
        "simple_outline",
        "rounded_outline",
        "heavy_outline",
        "mixed_outline",
        "double_outline",
        "fancy_outline",
        "github",
        "pipe",
        "orgtbl",
        "jira",
        "presto",
        "pretty",
        "psql",
        "rst",
        "mediawiki",
        "moinmoin",
        "youtrack",
        "html",
        "unsafehtml",
        "latex",
        "latex_raw",
        "latex_booktabs",
        "latex_longtable",
        "tsv",
        "textile",
        "asciidoc",
    ]
)


def __getattr__(name):
    # Every other public/internal name lives in the vendored module
    # (`T._wrap_text_to_colwidths`, `T._table_formats`, ...).
    return getattr(_pure_module, name)


def _wide_mode():
    return _pure_module.wcwidth is not None and _pure_module.WIDE_CHARS_MODE


def tabulate(
    tabular_data,
    headers=(),
    tablefmt="simple",
    floatfmt="g",
    intfmt="",
    numalign="default",
    stralign="default",
    missingval="",
    showindex="default",
    disable_numparse=False,
    colglobalalign=None,
    colalign=None,
    preserve_whitespace=False,
    maxcolwidths=None,
    headersglobalalign=None,
    headersalign=None,
    rowalign=None,
    maxheadercolwidths=None,
    break_long_words=True,
    break_on_hyphens=True,
):
    """Format a fixed width table for pretty printing (see `_pure.tabulate`
    for the full documentation)."""
    if tabular_data is None:
        tabular_data = []
    if (
        _kernel is not None
        and isinstance(tablefmt, str)
        and tablefmt in _KNOWN_FORMATS
        and maxcolwidths is None
        and maxheadercolwidths is None
    ):
        rows, headers_n, headers_pad = _normalize_tabular_data(
            tabular_data, headers, showindex=showindex
        )
        rows, separating_lines = _remove_separating_lines(rows)
        try:
            out, is_html, warn_colalign, warn_headersalign = _kernel(
                rows,
                headers_n,
                headers_pad,
                separating_lines or [],
                tablefmt,
                floatfmt,
                intfmt,
                missingval,
                numalign,
                stralign,
                colglobalalign,
                colalign,
                headersglobalalign,
                headersalign,
                rowalign,
                disable_numparse,
                preserve_whitespace,
                _pure_module.MIN_PADDING,
                _wide_mode(),
            )
        except _Fallback:
            pass
        else:
            # Warnings are formatted here (exact `repr`, `stacklevel`)
            # but recorded by the kernel at the upstream points.
            if warn_colalign is not None:
                _warnings.warn(
                    f"As a string, `colalign` is interpreted as "
                    f"{[c for c in warn_colalign]}. Did you mean "
                    f'`colglobalalign = "{warn_colalign}"` or '
                    f'`colalign = ("{warn_colalign}",)`?',
                    stacklevel=2,
                )
            if warn_headersalign is not None:
                _warnings.warn(
                    f"As a string, `headersalign` is interpreted as "
                    f"{[c for c in warn_headersalign]}. Did you mean "
                    f'`headersglobalalign = "{warn_headersalign}"` or '
                    f'`headersalign = ("{warn_headersalign}",)`?',
                    stacklevel=2,
                )
            if is_html:
                return JupyterHTMLStr(out)
            return out
    return _pure_tabulate(
        tabular_data,
        headers,
        tablefmt,
        floatfmt,
        intfmt,
        numalign,
        stralign,
        missingval,
        showindex,
        disable_numparse,
        colglobalalign,
        colalign,
        preserve_whitespace,
        maxcolwidths,
        headersglobalalign,
        headersalign,
        rowalign,
        maxheadercolwidths,
        break_long_words,
        break_on_hyphens,
    )


# Route the vendored `_main`/`_pprint_file` (CLI) through the dispatcher.
_pure_module.tabulate = tabulate

if __name__ == "__main__":
    _main()
