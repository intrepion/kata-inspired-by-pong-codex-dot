# Arcade metadata

`arcade/manifest.json` is this game’s v1 manifest. Image paths are relative to the published site root; each image has a SHA-256 and source provenance. An empty screenshot array means no verified images are available. Existing captures have unknown date and deployed-build identity; they are not fresh automated captures. Run `node scripts/verify-arcade.mjs` after edits.

Version 2 also permits reviewed `ci_gameplay_capture` entries. The PNG in `arcade/capture-evidence/` is the original CI artifact; the JPEG is the optimized gallery image. The manifest records both hashes, the capture commit and Actions run, the served input hashes, and the time the published files were verified against those inputs. This verification is historical and does not guarantee that the live game will remain unchanged. Capture artifacts are created manually by the capture workflow and are not published automatically.
