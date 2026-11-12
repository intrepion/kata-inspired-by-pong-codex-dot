# Windows web collision checkpoint recheck

On 2026-11-12, Windows rebuilt Yew, Leptos and Dioxus web assets from `9cee3c3c1f7374c534c7be7765e7d1e8e2d48fee` using locked dependencies, the existing MSVC Rust toolchain and wasm-bindgen 0.2.130. Builds used an isolated checkout and existing target cache.

Served through localhost in Chrome, each app's scripted replay reached tick 1440/1440 and presented a telemetry link and six SVG links. The links were observed, not downloaded or compared. These app-generated SVGs are scene markup, not screenshots; the replay disables user input and is not an interactive or physical-touch pass.

In a separate normal Yew session, Start produced moving ball/score activity and a short mouse hold on Up moved the player paddle. This is a bounded browser smoke check, not a full match or collision-specific manual acceptance. Leptos and Dioxus normal interaction, native clients, packages, physical touch and fresh screenshots remain unqualified for this physics revision.

Separately, Windows model tests at this source passed 13 core, 8 scenario and 1 WASM test. The portable reference comparison reported maximum absolute delta zero against its 0.001 allowance. These model results do not establish rendered or physical-input behavior.
