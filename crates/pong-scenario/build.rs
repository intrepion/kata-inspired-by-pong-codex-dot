use std::{collections::BTreeMap, env, fs, path::PathBuf};
fn main() {
    let path = "../../scenarios/smoke-v1.txt";
    println!("cargo:rerun-if-changed={path}");
    let text = fs::read_to_string(path).expect("portable scenario manifest");
    let mut fields = BTreeMap::new();
    let mut events = Vec::new();
    let mut inputs = Vec::new();
    let mut captures = Vec::new();
    let number = |s: &str| s.parse::<u32>().expect("unsigned integer");
    for line in text.lines() {
        let parts: Vec<_> = line.split('#').next().unwrap().split_whitespace().collect();
        match parts.as_slice() {
            [] => (),
            ["event", tick, action] => {
                assert!(["start", "pause", "toggle_pause"].contains(action));
                events.push((number(tick), *action));
            }
            ["input", from, to, value] => {
                let v = value.parse::<i32>().expect("signed input");
                assert!((-1..=1).contains(&v));
                inputs.push((number(from), number(to), v));
            }
            ["capture", tick] => captures.push(number(tick)),
            [key, value]
                if [
                    "schema",
                    "id",
                    "seed",
                    "step_hz",
                    "end_tick",
                    "report_every",
                ]
                .contains(key) =>
            {
                assert!(fields.insert(*key, *value).is_none(), "duplicate field");
            }
            _ => panic!("invalid scenario line: {line}"),
        }
    }
    assert_eq!(fields["schema"], "1");
    let id = fields["id"];
    assert!(id.chars().all(|c| c.is_ascii_alphanumeric() || c == '-'));
    let seed = number(fields["seed"]);
    let hz = number(fields["step_hz"]);
    let end = number(fields["end_tick"]);
    let report = number(fields["report_every"]);
    assert!(hz > 0 && end > 0 && report > 0);
    assert!(events.iter().all(|(t, _)| *t < end));
    assert!(events.windows(2).all(|w| w[0].0 < w[1].0));
    assert!(inputs.iter().all(|(a, b, _)| a < b && *b <= end));
    assert!(inputs.windows(2).all(|w| w[0].1 <= w[1].0));
    assert!(captures.iter().all(|t| *t > 0 && *t <= end));
    assert!(captures.windows(2).all(|w| w[0] < w[1]));
    let output = format!("pub const SCENARIO_ID: &str = {id:?};\npub const SEED: u64 = {seed};\npub const STEP: f32 = 1.0 / {hz}.0;\npub const END_TICK: u32 = {end};\npub const REPORT_EVERY: u32 = {report};\npub const CAPTURE_TICKS: [u32; {}] = {captures:?};\nconst EVENTS: &[(u32,&str)] = &{events:?};\nconst INPUTS: &[(u32,u32,i32)] = &{inputs:?};\n",captures.len());
    fs::write(
        PathBuf::from(env::var_os("OUT_DIR").unwrap()).join("scenario.rs"),
        output,
    )
    .unwrap();
}
