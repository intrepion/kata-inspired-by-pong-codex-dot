# Piston longer automated start attempt: Mac (2026-11-13)

After the separate raylib diagnostic found that 500 ms automated presses succeeded, the uninstrumented Piston Ready build from `PISTON-INDICATOR-BACKING-MAC-20261113.md` (executable SHA-256 `c090965d0365fc33707b423fee6b9e01a90ddd3c4770b439bd3fb24ff2f1cb4b`) was reopened. A court click followed by a 500 ms automated Return left the window visibly Ready.

This does not establish the cause or physical-user behavior. The prior instrumented event log in `PISTON-FOCUS-DIAGNOSTIC-MAC-20261113.md` recorded input while its focus flag was false, but that log is from a separate run. A confirmed focus regain and physical input remain open; no focus gate was removed.
