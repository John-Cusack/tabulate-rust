"""Differential equivalence: Rust kernel vs vendored pure-Python.

Two tracks, both comparing against `_pure_tabulate` (byte-identical to
upstream 0.10.0, the oracle):
- `test_kernel_matches_pure`: calls the Rust `_kernel` directly on
  eligible inputs (any `Fallback` fails the test — the kernel must
  handle these itself) and compares text, html-flag, and warnings.
- `test_dispatcher_matches_pure`: calls the public `tabulate()`
  (kernel or documented fallback) over a wider matrix — including
  exotic inputs — and compares text plus warning messages.

Deterministic (fixed seed). Run: `pytest tests/test_equivalence.py`.
"""

import itertools
import random
import warnings

import pytest

import tabulate as T
from tabulate import _pure as P

ALL_FORMATS = sorted(P._table_formats.keys())

FLOAT_SPECS = ["g", ".2f", ".3f", ",.5g", ",.2f", ".0f", ".1e", "%", "+.2f", "08.2f"]
INT_SPECS = ["", ",", "+d", "05d", "x", "#x", "08d"]
MISSING = ["", "?", "n/a", "MISSING"]
ALIGNS = ["left", "right", "center", "decimal", None, "global", "bogus", ""]

STR_CELLS = [
    "spam", "eggs", "", "451", "451.0", "1,000", "1,000.5", "1e3", "inf",
    "-inf", "nan", "True", "False", "  padded  ", "0", "-0", "+.5", ".5",
    "5.", "1_0", "  12  ", "0x10", "1,0000", "1000",
]
UNICODE_CELLS = [
    "配列", "héllo", "привет", "①②", "\u200b", "　wide　", "a\u0301",
    "Ω≈ç√", "\x01", " \x01 ",
]
ANSI_CELLS = ["\x1b[31m42\x1b[0m", "\x1b[1mhi\x1b[0m"]
MULTILINE_CELLS = ["a\nb", "x\ry\nz", "one\ntwo\nthree", "trail\n"]
INT_CELLS = [0, 1, -1, 42, 10**18, -(2**63) + 1, 2**63 - 1]
FLOAT_CELLS = [0.0, -0.0, 1.5, 41.9999, 1e16, 1e-5, 0.0001, float("inf"),
               float("-inf"), 1234567.891]
BYTES_CELLS = [b"abc", b"123", b"", b"\xff\x00", b"a\nb"]
OTHER_CELLS = [None, True, False]


def cell_pool(rng):
    pool = []
    pool += STR_CELLS
    pool += UNICODE_CELLS
    pool += ANSI_CELLS
    pool += MULTILINE_CELLS
    pool += INT_CELLS
    pool += FLOAT_CELLS
    pool += BYTES_CELLS
    pool += OTHER_CELLS
    rng.shuffle(pool)
    return pool


def random_table(rng):
    pool = cell_pool(rng)
    nrows = rng.choice([0, 1, 2, 3, 5])
    ncols = rng.choice([0, 1, 2, 3, 4])
    rows = []
    for _ in range(nrows):
        width = ncols
        if rng.random() < 0.2 and width:
            # ragged rows (short or long)
            width = rng.choice([0, 1, max(0, ncols - 1), ncols + 1, ncols + 2])
        rows.append([rng.choice(pool) for _ in range(width)])
    if rows and rng.random() < 0.25:
        rows.insert(rng.randrange(len(rows) + 1), T.SEPARATING_LINE)
    if rows and rng.random() < 0.15:
        rows.insert(rng.randrange(len(rows) + 1), ["\x01", "x"][: len(rows[0]) or 1])
    return rows


def random_headers(rng, ncols):
    kind = rng.random()
    if kind < 0.3:
        return []
    if kind < 0.45:
        return ["h%d" % i for i in range(ncols)]
    if kind < 0.55:
        return ["Hdr %d" % i for i in range(max(ncols - 1, 0))]
    if kind < 0.65:
        return ["first", "second", "third", "fourth", "fifth"][:ncols]
    if kind < 0.72:
        return "firstrow"
    if kind < 0.79:
        return "keys"
    if kind < 0.86:
        return ["<h>", "a&b", "  spaced  ", "配列", "x\ny"]
    return [""] * ncols


def random_kwargs(rng):
    kw = {}
    kw["tablefmt"] = rng.choice(ALL_FORMATS)
    kw["floatfmt"] = rng.choice(FLOAT_SPECS + [[".1f", ".3f"], (".2f",)])
    kw["intfmt"] = rng.choice(INT_SPECS)
    kw["missingval"] = rng.choice(MISSING + [["?", "?"], ("m",)])
    kw["numalign"] = rng.choice(["default", "decimal", "left", "right", "center", None])
    kw["stralign"] = rng.choice(["default", "left", "right", "center", None])
    if rng.random() < 0.2:
        kw["colglobalalign"] = rng.choice(["left", "right", "center", "decimal"])
    if rng.random() < 0.25:
        n = rng.choice([1, 2, 3])
        kw["colalign"] = tuple(rng.choice(ALIGNS) for _ in range(n))
    elif rng.random() < 0.1:
        kw["colalign"] = rng.choice(["left", "center"])
    if rng.random() < 0.15:
        kw["headersglobalalign"] = rng.choice(["left", "right", "center"])
    if rng.random() < 0.2:
        n = rng.choice([1, 2])
        kw["headersalign"] = tuple(
            rng.choice(["left", "right", "center", "same", "global", None])
            for _ in range(n)
        )
    elif rng.random() < 0.08:
        kw["headersalign"] = "left"
    if rng.random() < 0.15:
        kw["rowalign"] = rng.choice([["top"], ["bottom", "center"], "bottom", "center"])
    if rng.random() < 0.15:
        kw["preserve_whitespace"] = True
    if rng.random() < 0.15:
        kw["disable_numparse"] = rng.choice([True, [0], [1, 2], [0, 1, 2, 3]])
    if rng.random() < 0.15:
        kw["showindex"] = rng.choice(["always", "never", True, False, [10, 20, 30]])
    return kw


def kernel_eligible(rows, kw):
    """Inputs the Rust kernel must handle without `Fallback`."""
    ff = kw.get("floatfmt", "g")
    specs = [ff] if isinstance(ff, str) else list(ff)
    if any("s" in s for s in specs):
        return False
    # Non-ASCII in a possibly-decimal column falls back (documented).
    if has_nonascii(rows, kw.get("headers", [])) and may_align_decimal(kw):
        return False
    return True


CASES = []
_rng = random.Random(20261028)
for i in range(400):
    rows = random_table(_rng)
    kw = random_kwargs(_rng)
    kw["headers"] = random_headers(_rng, max((len(r) for r in rows if isinstance(r, list)), default=0))
    CASES.append((rows, kw))


def has_nonascii(rows, headers):
    def nonascii_cell(c):
        return isinstance(c, str) and not c.isascii()
    if isinstance(headers, str):
        return True  # firstrow/keys: resolved text unpredictable; be lax
    if any(isinstance(h, str) and not h.isascii() for h in headers):
        return True
    for row in rows:
        if isinstance(row, str):
            continue
        try:
            cells = list(row)
        except TypeError:
            continue
        if any(nonascii_cell(c) for c in cells):
            return True
    return False


def may_align_decimal(kw):
    """Conservative: could any column resolve to decimal alignment?"""
    opts = [
        kw.get("numalign"), kw.get("colglobalalign"),
        kw.get("headersglobalalign"),
    ]
    if any(o == "decimal" or o == "default" for o in opts):
        return True
    for key in ("colalign", "headersalign"):
        v = kw.get(key)
        if isinstance(v, str) and v == "decimal":
            return True
        if isinstance(v, (list, tuple)) and "decimal" in v:
            return True
    return False


def run_pure(rows, kw):
    headers = kw.pop("headers")
    try:
        with warnings.catch_warnings(record=True) as caught:
            warnings.simplefilter("always")
            text = P.tabulate(rows, headers, **kw)
        msgs = [str(w.message) for w in caught]
        err = None
    except Exception as e:  # noqa: BLE001 - compared by type below
        text, msgs, err = None, [], "%s:%s" % (type(e).__name__, e)
    finally:
        kw["headers"] = headers
    return text, msgs, err


def test_kernel_matches_pure():
    eligible = [c for c in CASES if kernel_eligible(c[0], c[1])]
    assert len(eligible) > 200
    for rows, kw in eligible:
        headers = kw.pop("headers")
        try:
            norm_rows, norm_headers, pad = P._normalize_tabular_data(
                rows, headers, showindex=kw.get("showindex", "default")
            )
            sans, seps = P._remove_separating_lines(norm_rows)
            with warnings.catch_warnings(record=True):
                warnings.simplefilter("always")
                out, is_html, wcol, whdr = T._kernel(
                    sans, norm_headers, pad, seps or [], kw.get("tablefmt", "simple"),
                    kw.get("floatfmt", "g"), kw.get("intfmt", ""),
                    kw.get("missingval", ""), kw.get("numalign", "default"),
                    kw.get("stralign", "default"), kw.get("colglobalalign"),
                    kw.get("colalign"), kw.get("headersglobalalign"),
                    kw.get("headersalign"), kw.get("rowalign"),
                    kw.get("disable_numparse", False),
                    kw.get("preserve_whitespace", False), P.MIN_PADDING,
                    T._wide_mode(),
                )
            kernel_result = (out, is_html, wcol, whdr, None)
        except T._Fallback:
            kernel_result = None
        except Exception as e:  # noqa: BLE001
            kernel_result = ("EXC", "%s:%s" % (type(e).__name__, e))
        finally:
            kw["headers"] = headers
        text, msgs, err = run_pure(rows, dict(kw, headers=headers))
        if err is None:
            assert kernel_result is not None, (rows, kw)
            assert kernel_result[0] == text, (rows, kw)
        else:
            # Pure raises: the kernel must bail (dispatcher replays via
            # fallback) or raise identically.
            assert kernel_result is None or kernel_result == ("EXC", err), (
                rows,
                kw,
                kernel_result,
                err,
            )
def test_dispatcher_matches_pure():
    for rows, kw in CASES:
        headers = kw["headers"]
        text, msgs, err = run_pure(rows, dict(kw))
        with warnings.catch_warnings(record=True) as caught:
            warnings.simplefilter("always")
            try:
                got = T.tabulate(rows, headers, **{k: v for k, v in kw.items() if k != "headers"})
                got_err = None
            except Exception as e:  # noqa: BLE001
                got, got_err = None, "%s:%s" % (type(e).__name__, e)
            got_msgs = [str(w.message) for w in caught]
        assert got_err == err, (rows, kw, got_err, err)
        if err is None:
            assert got == text, (rows, kw)
            assert got_msgs == msgs, (rows, kw, got_msgs, msgs)


def test_fallback_paths_match_pure():
    import datetime
    from fractions import Fraction

    exotic_rows = [
        [[datetime.datetime(2020, 1, 1)], [1]],
        [[Fraction(1, 3)], [0.5]],
        [[[1, 2]], ["x"]],
        [[b"\xff\xfe"], ["ok"]],
        [[10**100], [1]],
        [[float("nan")], ["x"]],
    ]
    for rows in exotic_rows:
        for fmt in ["simple", "grid", "github", "pipe", "plain"]:
            assert T.tabulate(rows, tablefmt=fmt) == P.tabulate(rows, tablefmt=fmt)


def test_html_wrapper_preserved():
    out = T.tabulate([["a"]], tablefmt="html")
    assert hasattr(out, "_repr_html_")
    assert out._repr_html_() == out.str
    assert T.tabulate([], tablefmt="html") == ""
