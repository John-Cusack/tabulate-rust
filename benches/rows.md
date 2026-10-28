# bench (measured-only)

machine: x86_64, python 3.12.3, tabulate 0.10.0 (pure) vs tabulate-rust 0.1.0.
wall = best-of-3 per `tabulate()` call over the frozen WORKLOAD.md 400-row
mixed-type table (grid/github/rst/plain, plus a mixed track of 3 formats);
maxrss = fresh-process peak (setup included, same shape both sides).
pure side runs the vendored `_pure_tabulate` (the exact fallback);
rust side runs the dispatcher (kernel, no fallback on these inputs).

| workload | pure wall (ms) | rust wall (ms) | speedup | pure RSS (MB) | rust RSS (MB) |
|---|---|---|---|---|---|
| grid | 14.59 | 1.44 | 10.12x | 19.0 | 20.2 |
| github | 14.55 | 1.29 | 11.29x | 19.2 | 19.7 |
| rst | 14.63 | 1.23 | 11.87x | 19.2 | 19.3 |
| plain | 14.62 | 1.26 | 11.62x | 19.5 | 19.1 |
| mixed | 44.21 | 4.01 | 11.03x | 19.2 | 19.5 |
