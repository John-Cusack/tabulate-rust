# tabulate-rust

A Rust port of [python-tabulate](https://github.com/astanin/python-tabulate)
(0.10.0, MIT) with a drop-in Python face: `import tabulate` keeps working,
`pip install tabulate-rust` replaces `pip install tabulate`.

- `tabulate-core` (this repo, `tabulate-core/`): pure-Rust formatting
  kernel — per-cell type sniffing, column measurement, numeric formatting,
  alignment, and layout for all 26 named table formats. No Python dependency.
- `tabulate` (this repo, `tabulate/`): Python face. Input normalization
  (dicts/numpy/pandas/dataclasses, `showindex`, headers) runs on the
  vendored pure-Python implementation (`tabulate/_pure.py`, byte-identical
  to upstream), then the whole kernel runs in Rust exactly once per call.
  Anything outside the kernel's exact domain falls back to the vendored
  implementation for that call: identical output, slower path only. The
  pure-Python fallback stays importable without the extension.

<!-- RUSTSMITH-PERF:BEGIN -->
<!-- measured benchmarks are injected here by release-prep -->
<!-- RUSTSMITH-PERF:END -->

## Correctness

- Full upstream 0.10.0 suite green (`tests/upstream/`, unmodified,
  incl. doctests), plus a differential equivalence corpus
  (`tests/test_equivalence.py`) comparing kernel vs pure over
  randomized tables × formats × options.
- 1300+ CPython-computed number-formatting vectors guard the
  `float`/`int` spec engine (`tabulate-core/tests/gen_*.rs`).

## License

MIT — see [LICENSE](LICENSE). Upstream attribution in [NOTICE](NOTICE).
