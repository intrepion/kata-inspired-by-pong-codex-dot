//! Scoped opt-in export of the existing browser framework demo's own SVG sources.
use std::{io::Write, path::PathBuf, sync::Mutex};
struct Export(Mutex<Option<Option<PathBuf>>>);
fn validate(value: &serde_json::Value) -> Result<Vec<(String, String)>, String> {
    let entries = value.as_array().ok_or("expected export list")?;
    let mut names = vec!["telemetry.jsonl".to_owned()];
    names.extend(pong_scenario::CAPTURE_TICKS.map(|tick| format!("tick-{tick:04}.svg")));
    if entries.len() != names.len() {
        return Err("wrong export count".into());
    }
    let mut result = Vec::new();
    for (entry, expected) in entries.iter().zip(names) {
        if entry["name"] != expected {
            return Err("wrong export name/order".into());
        }
        result.push((
            expected,
            entry["text"].as_str().ok_or("missing text")?.to_owned(),
        ));
    }
    let reports = result[0]
        .1
        .lines()
        .map(serde_json::from_str::<serde_json::Value>)
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())?;
    let mut scenario = pong_scenario::Scenario::default();
    let mut game = pong_core::Game::new(pong_scenario::SEED);
    let mut expected =
        vec![serde_json::from_str::<serde_json::Value>(&scenario.report(&game)).unwrap()];
    while !scenario.finished() {
        scenario.advance(&mut game);
        if scenario.tick() % 120 == 0 {
            expected.push(serde_json::from_str(&scenario.report(&game)).unwrap());
        }
    }
    if reports != expected {
        return Err("shared scenario states differ".into());
    }
    if result
        .iter()
        .skip(1)
        .any(|(_, svg)| !svg.starts_with("<svg") || !svg.ends_with("</svg>"))
    {
        return Err("missing SVG scene".into());
    }
    Ok(result)
}
#[tauri::command]
fn finish_framework_demo(
    app: tauri::AppHandle,
    state: tauri::State<'_, Export>,
    evidence: String,
) -> Result<(), String> {
    if evidence.len() > 8 * 1024 * 1024 {
        return Err("evidence too large".into());
    }
    let mut enabled = state.0.lock().map_err(|e| e.to_string())?;
    let directory = enabled
        .as_ref()
        .ok_or("demo export disabled/already completed")?;
    let entries = validate(&serde_json::from_str(&evidence).map_err(|e| e.to_string())?)?;
    if let Some(directory) = directory {
        for (name, text) in &entries {
            std::fs::OpenOptions::new()
                .create_new(true)
                .write(true)
                .open(directory.join(name))
                .and_then(|mut f| f.write_all(text.as_bytes()))
                .map_err(|e| e.to_string())?;
        }
    }
    println!("{}", entries[0].1);
    *enabled = None;
    app.exit(0);
    Ok(())
}
pub fn run(context: tauri::Context<tauri::Wry>) {
    let args: Vec<_> = std::env::args().collect();
    assert!(
        !args.iter().any(|a| a == "--scripted-captures"),
        "Use --scripted-scenes: these are SVG sources, not screenshots"
    );
    let scripted = args.iter().any(|a| a == "--scripted-test");
    let directory = args.iter().position(|a| a == "--scripted-scenes").map(|i| {
        assert!(scripted, "--scripted-scenes requires --scripted-test");
        let p = PathBuf::from(args.get(i + 1).expect("missing NEW scene directory"));
        std::fs::create_dir(&p).expect("create NEW scene directory");
        p
    });
    tauri::Builder::default().manage(Export(Mutex::new(scripted.then_some(directory))))
    .invoke_handler(tauri::generate_handler![finish_framework_demo])
    .on_page_load(move|webview,payload|{
        if scripted && matches!(payload.event(),tauri::webview::PageLoadEvent::Finished){
            webview.eval(r#"(()=>{
              if(!new URLSearchParams(location.search).has('scripted-test')){location.search='?scripted-test';return;}
              if(window.__pongScenePoll)return;
              window.__pongScenePoll=setInterval(()=>{
                const links=[...document.querySelectorAll('a[download]')];
                if(links.length!==7)return;
                clearInterval(window.__pongScenePoll);
                const evidence=JSON.stringify(links.map(a=>({name:a.download,text:decodeURIComponent(a.getAttribute('href').split(',').slice(1).join(','))})));
                window.__TAURI_INTERNALS__.invoke('finish_framework_demo',{evidence}).catch(console.error);
              },50);
            })()"#).expect("local demo hook");
        }
    }).run(context).expect("Pong native web startup");
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn shared_reports_validate_but_changed_filename_does_not() {
        let mut scenario = pong_scenario::Scenario::default();
        let mut game = pong_core::Game::new(pong_scenario::SEED);
        let mut reports = vec![scenario.report(&game)];
        while !scenario.finished() {
            scenario.advance(&mut game);
            if scenario.tick() % 120 == 0 {
                reports.push(scenario.report(&game));
            }
        }
        let mut entries =
            vec![serde_json::json!({"name":"telemetry.jsonl","text":reports.join("\n")})];
        entries.extend(
            pong_scenario::CAPTURE_TICKS.map(
                |t| serde_json::json!({"name":format!("tick-{t:04}.svg"),"text":"<svg></svg>"}),
            ),
        );
        assert!(validate(&serde_json::json!(entries)).is_ok());
        entries[1]["name"] = serde_json::json!("../escape.svg");
        assert!(validate(&serde_json::json!(entries)).is_err());
    }
    #[test]
    fn rejects_wrong_count_or_path() {
        assert!(validate(&serde_json::json!([])).is_err());
        assert!(validate(&serde_json::json!([{"name":"../../file","text":"bad"}])).is_err());
    }
}
