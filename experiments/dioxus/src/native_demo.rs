//! Native export of common renderer SVG source, not webview or OS screenshots.
use pong_web_ui_common::Session;
use std::{io::Write, path::PathBuf, sync::OnceLock};
static OPTIONS: OnceLock<(bool, Option<PathBuf>)> = OnceLock::new();
pub fn configure() {
    let args: Vec<_> = std::env::args().collect();
    assert!(
        !args.iter().any(|a| a == "--scripted-captures"),
        "Use --scripted-scenes for SVG source, not screenshots"
    );
    let enabled = args.iter().any(|a| a == "--scripted-test");
    let directory = args.iter().position(|a| a == "--scripted-scenes").map(|i| {
        assert!(enabled, "--scripted-scenes requires --scripted-test");
        let path = PathBuf::from(args.get(i + 1).expect("missing NEW directory"));
        std::fs::create_dir(&path).expect("create NEW scene directory");
        path
    });
    OPTIONS
        .set((enabled, directory))
        .expect("configure native demo once");
}
pub fn enabled() -> bool {
    OPTIONS.get().is_some_and(|o| o.0)
}
pub fn finish_if_ready(session: &Session) {
    if !enabled() {
        return;
    }
    let Some(exports) = session.completed_demo_exports() else {
        return;
    };
    let reports = exports.reports.join("\n") + "\n";
    if let Some(directory) = &OPTIONS.get().unwrap().1 {
        for (tick, svg) in exports.captures {
            write_new(
                directory.join(format!("tick-{tick:04}.svg")),
                svg.as_bytes(),
            );
        }
        write_new(directory.join("telemetry.jsonl"), reports.as_bytes());
    }
    print!("{reports}");
    // Bounded test process only. Does not claim the final DOM update was presented.
    std::process::exit(0);
}
fn write_new(path: PathBuf, bytes: &[u8]) {
    std::fs::OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(path)
        .and_then(|mut f| f.write_all(bytes))
        .expect("write new native demo evidence");
}
