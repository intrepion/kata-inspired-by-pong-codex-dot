use base64::Engine;
use std::{io::Write, path::PathBuf, sync::Mutex};

struct Scripted(Mutex<Option<Option<PathBuf>>>);

fn validate_evidence(value: &serde_json::Value) -> Result<(), String> {
    if value["finished"] != true {
        return Err("scenario not finished".into());
    }
    let reports = value["reports"].as_array().ok_or("missing reports")?;
    let mut scenario = pong_scenario::Scenario::default();
    let mut game = pong_core::Game::new(pong_scenario::SEED);
    let mut expected =
        vec![serde_json::from_str::<serde_json::Value>(&scenario.report(&game)).unwrap()];
    while !scenario.finished() {
        scenario.advance(&mut game);
        if scenario.tick().is_multiple_of(120) {
            expected.push(serde_json::from_str(&scenario.report(&game)).unwrap());
        }
    }
    if reports != &expected {
        return Err("shared scenario telemetry mismatch".into());
    }
    let captures = value["captures"].as_array().ok_or("missing captures")?;
    if captures.len() != pong_scenario::CAPTURE_TICKS.len() {
        return Err("wrong capture count".into());
    }
    for (capture, tick) in captures.iter().zip(pong_scenario::CAPTURE_TICKS) {
        if capture["tick"] != tick {
            return Err("wrong capture tick".into());
        }
        decode_png(capture)?;
    }
    Ok(())
}
fn decode_png(capture: &serde_json::Value) -> Result<Vec<u8>, String> {
    let text = capture["png"].as_str().ok_or("missing PNG")?;
    let encoded = text
        .strip_prefix("data:image/png;base64,")
        .ok_or("not PNG data URL")?;
    let bytes = base64::engine::general_purpose::STANDARD
        .decode(encoded)
        .map_err(|e| e.to_string())?;
    if !bytes.starts_with(b"\x89PNG\r\n\x1a\n") {
        return Err("bad PNG header".into());
    }
    Ok(bytes)
}
fn write_new(path: PathBuf, bytes: &[u8]) -> Result<(), String> {
    std::fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .and_then(|mut f| f.write_all(bytes))
        .map_err(|e| e.to_string())
}
#[tauri::command]
fn finish_scripted(
    app: tauri::AppHandle,
    state: tauri::State<'_, Scripted>,
    evidence: String,
) -> Result<(), String> {
    if evidence.len() > 8 * 1024 * 1024 {
        return Err("evidence too large".into());
    }
    let mut enabled = state.0.lock().map_err(|e| e.to_string())?;
    let directory = enabled
        .as_ref()
        .ok_or("scripted export disabled or already completed")?;
    let value: serde_json::Value = serde_json::from_str(&evidence).map_err(|e| e.to_string())?;
    validate_evidence(&value)?;
    let reports = value["reports"].as_array().unwrap();
    let jsonl = reports.iter().map(|r| format!("{r}\n")).collect::<String>();
    if let Some(directory) = directory {
        for capture in value["captures"].as_array().unwrap() {
            let tick = capture["tick"].as_u64().unwrap();
            write_new(
                directory.join(format!("tick-{tick:04}.png")),
                &decode_png(capture)?,
            )?;
        }
        write_new(directory.join("telemetry.jsonl"), jsonl.as_bytes())?;
    }
    print!("{jsonl}");
    *enabled = None;
    app.exit(0);
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let args: Vec<_> = std::env::args().collect();
    let scripted = args.iter().any(|a| a == "--scripted-test");
    let directory = args
        .iter()
        .position(|a| a == "--scripted-captures")
        .map(|i| {
            assert!(scripted, "--scripted-captures requires --scripted-test");
            let path = PathBuf::from(args.get(i + 1).expect("missing NEW capture directory"));
            std::fs::create_dir(&path).expect("create NEW capture directory");
            path
        });
    // The only command accepts completed local app evidence, never arbitrary paths.
    // Disabled entirely for normal play; no privileged plugin capabilities added.
    tauri::Builder::default()
        .manage(Scripted(Mutex::new(scripted.then_some(directory))))
        .invoke_handler(tauri::generate_handler![finish_scripted])
        .on_page_load(move |webview, payload| {
            if scripted && matches!(payload.event(), tauri::webview::PageLoadEvent::Finished) {
                webview.eval(r#"(() => {
                  if (!new URLSearchParams(location.search).has('scripted-test')) {
                    location.search = '?scripted-test'; return;
                  }
                  if (window.__nativePongDemoPoll) return;
                  window.__nativePongDemoPoll = setInterval(() => {
                    if (!window.pongDemo?.finished) return;
                    clearInterval(window.__nativePongDemoPoll);
                    window.__TAURI_INTERNALS__.invoke('finish_scripted', { evidence: JSON.stringify(window.pongDemo) })
                      .catch(error => console.error('Native scripted export failed', error));
                  }, 50);
                })()"#).expect("install local scripted evidence completion hook");
            }
        })
        .run(tauri::generate_context!())
        .expect("failed to run the Pong Tauri client");
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn accepts_shared_reports_and_rejects_changed_capture_ticks() {
        let mut scenario = pong_scenario::Scenario::default();
        let mut game = pong_core::Game::new(pong_scenario::SEED);
        let mut reports =
            vec![serde_json::from_str::<serde_json::Value>(&scenario.report(&game)).unwrap()];
        while !scenario.finished() {
            scenario.advance(&mut game);
            if scenario.tick().is_multiple_of(120) {
                reports.push(serde_json::from_str(&scenario.report(&game)).unwrap());
            }
        }
        let png = format!(
            "data:image/png;base64,{}",
            base64::engine::general_purpose::STANDARD.encode(b"\x89PNG\r\n\x1a\n")
        );
        let captures: Vec<_> = pong_scenario::CAPTURE_TICKS
            .iter()
            .map(|tick| serde_json::json!({"tick":tick,"png":png}))
            .collect();
        let mut evidence =
            serde_json::json!({"finished":true,"reports":reports,"captures":captures});
        assert!(validate_evidence(&evidence).is_ok());
        evidence["captures"][0]["tick"] = serde_json::json!(999);
        assert!(validate_evidence(&evidence).is_err());
    }
    #[test]
    fn rejects_incomplete_or_untrusted_evidence() {
        assert!(validate_evidence(&serde_json::json!({"finished":false})).is_err());
        assert!(
            validate_evidence(&serde_json::json!({"finished":true,"reports":[],"captures":[]}))
                .is_err()
        );
        assert!(decode_png(&serde_json::json!({"png":"file:///etc/passwd"})).is_err());
        assert!(decode_png(&serde_json::json!({"png":"data:image/png;base64,aGVsbG8="})).is_err());
    }
}
