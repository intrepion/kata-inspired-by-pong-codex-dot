# Windows GPUI contact-timing recheck

On 2026-11-13, GPUI was built from `1509cf7c134739fb4ad81d2307cec6559140e595` with locked offline dependencies in Windows MSVC debug mode, in a fresh target directory. The GPUI court geometry test and eight shared GUI-model tests passed. Executable SHA-256: `ad9fce9406a838b88db540d4e7e77cdbafa272c84f4058e03d0b78b37ad978b9`. Cargo repeated the existing `proc-macro-error2` 2.0.1 future-compatibility warning.

The foreground `--scripted-test` run exited 0 with empty stderr. Its 13 [telemetry records](evidence/windows-gpui-contact-timing-20261113.jsonl) match the contact-timing model reference line-for-line through finished tick 1440. This is scripted state evidence; GPUI does not support app-owned pixel export. A prior hidden launch timed out and was stopped; it is not counted as a replay pass.

The window was not visually inspected and normal native interaction, physical input, full match, focus/resize, packaging and release qualification remain open for this revision. Earlier binaries and evidence remain historical.
