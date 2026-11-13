# Windows Softbuffer and ash explicit-pause recheck

On 2026-11-13, Softbuffer and Vulkan/ash were rebuilt from `b220e8f2eaf1549611200ab70b09c8b1632b9674`, including pause-fix commit `81a86673f4a7113c9933f51a2cef247f3e38a8d3`, with locked offline Windows MSVC debug dependencies. Softbuffer passed all six frontend/action tests; ash passed all eight. These include W, Up, S and Down clearing on explicit pause, ignored repeats and fresh presses. They test mapped action logic, not OS or physical-key delivery.

Softbuffer executable SHA-256: `d2a23c7db7414ea73db54a947fc9b279599934c507fe3790fe6428c814a2a545`; ash executable SHA-256: `35c4d47717cd652dec2ca9c54da70d4b9d23134015ed3051913a1fb869b50ee5`. The previous Softbuffer executable was copied to the local evidence directory before rebuild; ash used a fresh target directory, preserving earlier artifacts.

Each foreground scripted run exited 0 with six app-owned PNGs and 13 records matching the contact-timing reference line-for-line: [Softbuffer](evidence/windows-softbuffer-pause-20261113.jsonl), [ash](evidence/windows-ash-pause-20261113.jsonl). Softbuffer's stderr was empty; ash identified the Vulkan device as Intel UHD Graphics 630. The inspected tick 480 frames show score, court, paddles, ball and pause marker. These are application-rendered outputs, not desktop screenshots. The scripted replay bypasses real held-key input and does not prove the fix in an OS event stream.

Native interaction, physical input, full match, focus/resize, packaging and release qualification remain open for these binaries. Previous and new local artifacts remain in their distinct directories. No prior desktop-approval timeout was retried.
