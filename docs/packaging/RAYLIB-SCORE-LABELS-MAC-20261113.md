# raylib score labels: macOS check (2026-11-13)

The raylib court score now identifies the left player and right CPU and centers the measured text. The adapter was based on Git blob `1af90de774ba30a3792a9e82805627accb69fe71` at checkpoint `b704f8a1f1b746dab5d4e873e63130ed3aeb71a7`; other files in the isolated build tree were not asserted to match that entire checkpoint.

On macOS, the offline locked release test suite passed all 18 tests, including score strings for 0:0 and 2:7, and the release build succeeded. The tested adapter SHA-256 was `148aefea8db6edfe5e2637ad083e15c4fadaba8d7cc8174dc8537fd90ca11dbe`; the candidate executable SHA-256 was `117017e38dd1ede4c2b8e269036dbe7d1889690faac6d3f402d4dc8097be3a49`. The native Ready window visibly rendered the centered “YOU 0 : 0 CPU” line. The test window was closed afterwards.

This is a default-size Ready-screen visual check and automated string/unit coverage. It does not establish physical input, a played match, nonzero score rendering, other window sizes, or release qualification. The separate native and system dependency/license review remains incomplete; see [the raylib Mac license screen](RAYLIB-MAC-LICENSE-SCREEN.md).
