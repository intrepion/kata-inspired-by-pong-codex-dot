# Active restart: ash, softbuffer, SDL3 and Blue Engine — Omarchy, 2026-11-13

Holding a movement key while pressing R during play continued moving the new paddle in each adapter. Ash and softbuffer now clear their event-held state before starting; SDL3 and Blue Engine mark their polling gate as not previously playing so the next sample suppresses keys still down. Release and a fresh press restore movement. Each added regression failed before the fix and passed afterward.

Full `cargo test --offline --locked` suites passed: ash 11, softbuffer 9, SDL3 9 and Blue Engine 8. These are handler/polling-gate tests on Omarchy, not tests of physical keys, rendered windows, rebuilt release packages or other platforms. Existing artifacts remain historical. This batch does not establish that all other adapters are covered.

Source Git blobs: ash `9355140bd8cf636e4187b4e9f5544bf26c477636`; softbuffer `b9e4a0c4e7e4e99a7927695eb587c8d73c572d0d`; SDL3 `ba94cf7802524e7b12061778e442339f1c420f33`; Blue Engine `474a26c4c584544d8b5b5708066e0ecc2a1482ef`.
