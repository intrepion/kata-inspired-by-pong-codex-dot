Pong Vulkan / ash: local Linux portable packaging pilot

Run ./pong-ash from this extracted folder. Space starts/pauses; W/S or arrow keys
move; R restarts; Escape quits. --scripted-test runs a bounded seeded replay.
--scripted-captures NEW_DIRECTORY additionally writes six app framebuffer PNGs.

Requires compatible architecture, Linux runtime libraries, Vulkan loader/driver
and a display session. The archive does not install or bundle system drivers.
This archive is not a public release or cross-distro compatibility guarantee.
Root MIT license included; complete dependency notices remain a release gate.

Extraction is the local installation: no system directories, PATH, services or
settings are changed. To remove, delete only the four files listed in SHA256SUMS
plus SHA256SUMS itself from this dedicated directory, then remove the empty
folder. Do not delete unrelated user files. Verify hashes before launching.
