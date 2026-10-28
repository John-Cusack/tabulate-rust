#!/usr/bin/env python3
"""Two-track bench: upstream `tabulate` (pure Python) vs `tabulate-rust`.

Measured only: wall time (best-of-3, per-call) + peak RSS (fresh process
per side per workload). Workloads are the frozen WORKLOAD.md 400-row
mixed-type table in grid/github/rst/plain formats.

Usage: venv/bin/python benches/two_track.py [--iters N]

Outputs a Markdown table on stdout. No projections.
"""

import argparse
import os
import resource
import statistics
import subprocess
import sys
import tempfile

MIRROR = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
sys.path.insert(0, MIRROR)


def make_table():
    names = ["alpha", "bravo", "charlie", "delta", "echo", "foxtrot",
             "golf", "hotel", "india", "juliet"]
    rows = []
    for i in range(400):
        rows.append((
            i,
            names[i % 10] + "-" + str(i),
            float(i * i % 1000) / 7.0,
            (i % 13) - 6,
            "yes" if i % 3 else "no",
            None if i % 17 == 0 else "n=%d" % i,
        ))
    return rows, ("id", "name", "score", "ratio", "active", "note")


WORKLOADS = ["grid", "github", "rst", "plain", "mixed"]
def bench_once(python, impl, workload, iters, pure):
    """Run one side in a fresh process; print wall_ms + peak_kb."""
    import tabulate as T

    rows, headers = make_table()
    fmts = [workload] if workload != "mixed" else ["grid", "github", "rst"]
    if pure:
        # The exact function the fallback invokes (unpatched original).
        run_one = T._pure_tabulate
    else:
        run_one = T.tabulate
    for f in fmts:
        run_one(rows, headers, tablefmt=f)
    out = None
    import time
    t0 = time.perf_counter()
    for _ in range(iters):
        for f in fmts:
            out = run_one(rows, headers, tablefmt=f)
    wall = (time.perf_counter() - t0) / iters
    assert out and len(out) > 1000
    print(f"wall_ms={wall * 1000:.3f} peak_kb={resource.getrusage(resource.RUSAGE_SELF).ru_maxrss}")

def run_worker(python, workload, iters, pure, reps=3):
    walls, peaks = [], []
    for _ in range(reps):
        p = subprocess.run(
            [python, __file__, "--worker", workload, str(iters), "1" if pure else "0"],
            capture_output=True, text=True, timeout=900,
            cwd=MIRROR,
        )
        assert p.returncode == 0, p.stderr[-2000:]
        kv = dict(t.split("=") for t in p.stdout.split())
        walls.append(float(kv["wall_ms"]))
        peaks.append(int(kv["peak_kb"]))
    return min(walls), max(peaks)


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--worker", nargs=3, metavar=("WORKLOAD", "ITERS", "PURE"))
    ap.add_argument("--iters", type=int, default=20)
    ap.add_argument("--python", default=sys.executable)
    args = ap.parse_args()
    if args.worker:
        bench_once(args.python, None, args.worker[0], int(args.worker[1]), args.worker[2] == "1")
        return
    iters = args.iters
    print("| workload | pure wall (ms) | rust wall (ms) | speedup | pure RSS (MB) | rust RSS (MB) |")
    print("|---|---|---|---|---|---|")
    for wl in WORKLOADS:
        pw, prss = run_worker(args.python, wl, iters, pure=True)
        rw, rrss = run_worker(args.python, wl, iters, pure=False)
        print(f"| {wl} | {pw:.2f} | {rw:.2f} | {pw / rw:.2f}x | {prss / 1024:.1f} | {rrss / 1024:.1f} |")


if __name__ == "__main__":
    main()
