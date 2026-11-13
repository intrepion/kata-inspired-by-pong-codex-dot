#!/usr/bin/env python3
"""Exercise actual Qt slots and production QML without desktop access."""
import os, pathlib, struct, subprocess, sys, tempfile
binary=pathlib.Path(sys.argv[1]).resolve()
env=dict(os.environ, QT_QPA_PLATFORM="offscreen", QT_QUICK_BACKEND="software")
for negative in (True,False):
    output=pathlib.Path(tempfile.mkdtemp(prefix="pong-qtbridge-test-"))
    result=subprocess.run([str(binary)]+(["--negative-control"] if negative else []),cwd=output,env=env,capture_output=True,timeout=15)
    image=output/"qtbridge-offscreen-court.png"
    if negative:
        assert not image.exists(), "Broken assertion incorrectly reached render completion"
        print("PASS negative control: deliberately wrong snapshot schema did not render")
    else:
        assert result.returncode==0, result.stderr.decode(errors="replace")
        data=image.read_bytes()
        assert data[:8]==b'\x89PNG\r\n\x1a\n', "PNG signature"
        assert struct.unpack('>II',data[16:24])==(900,500), "Court dimensions"
        print("PASS actual Qt slots/movement/pause/resume/restart/resize/render:",image)
