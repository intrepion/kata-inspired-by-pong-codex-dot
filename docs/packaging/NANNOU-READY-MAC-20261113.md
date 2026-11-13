# Nannou Ready prompt

2026-11-13. Nannou previously displayed the Ready state only in the window title; the court now displays `PRESS SPACE OR ENTER TO START` while Ready. The prompt disappears during play, pause and game over; the existing pause and winner messages remain. No new font or dependency was added.

On macOS ARM64, offline locked release-profile tests passed 13/13. The new model test covers Ready, Playing, Paused and Over prompt selection. A cached release build succeeded. Tested main.rs SHA-256 `363b7e7dc09e7a00158961107970d5951fdd0f90e1a3394b7a415c0885da35cf`; executable SHA-256 `5f26ca8d495856c2f3564795259f237431a026550250d3ce9ebd27b099b9a9c3`. Source was based on checkpoint `c01b901dece44c63dcdf9a1395f4aff0977d9859` and checked unchanged against `5877981faa5bce3c40b8604ce195304dbc688b40` before publication.

These are model and build checks, not actual rendered-window, full gameplay or physical-input verification. Prior artifacts were retained; isolated source and target remain under `/tmp/pong-mac-input-recheck-20261113-v1` and `/tmp/pong-mac-input-target-20261113-v1`.
