#!/usr/bin/env python3
"""Run the prebuilt macOS fixture: python3 tests/run_dispatch2_fixture.py /absolute/path/to/dispatch2_pilot_fixture"""
import pathlib
import resource
import signal
import subprocess
import sys

if sys.platform != "darwin" or len(sys.argv) != 2:
    raise SystemExit("Requires macOS and one fixture executable path")
exe = pathlib.Path(sys.argv[1]).resolve(strict=True)
def no_core():
    resource.setrlimit(resource.RLIMIT_CORE, (0, 0))
for name, args, expected in [("normal", [], 0), ("panic", ["panic"], -signal.SIGABRT)]:
    try:
        result = subprocess.run([str(exe), *args], capture_output=True, text=True,
                                timeout=15, preexec_fn=no_core, check=False)
    except subprocess.TimeoutExpired as error:
        raise SystemExit(f"{name}: timed out after 15 seconds") from error
    print(f"{name}: returncode={result.returncode}")
    print(result.stdout, end="")
    print(result.stderr, end="", file=sys.stderr)
    if result.returncode != expected:
        raise SystemExit(f"{name}: expected return code {expected}")
    if name == "normal" and "PASS direct, queued borrowed return, marker, off-main drop" not in result.stdout:
        raise SystemExit("normal: expected success marker absent")
    if name == "panic" and "intentional queued panic" not in result.stderr:
        raise SystemExit("panic: expected panic marker absent")
