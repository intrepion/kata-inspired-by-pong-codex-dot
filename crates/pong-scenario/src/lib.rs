//! Opt-in application-level input replay. Never injects operating-system input.
use pong_core::Game;

include!(concat!(env!("OUT_DIR"), "/scenario.rs"));

#[derive(Clone, Default)]
pub struct Scenario {
    tick: u32,
}

impl Scenario {
    pub fn tick(&self) -> u32 {
        self.tick
    }
    pub fn finished(&self) -> bool {
        self.tick >= END_TICK
    }

    /// Applies events before this fixed simulation step. The clock advances even
    /// while paused; callers must initialize Game with SEED for reproducibility.
    pub fn advance(&mut self, game: &mut Game) {
        if self.finished() {
            return;
        }
        for (_, action) in EVENTS.iter().filter(|(tick, _)| *tick == self.tick) {
            match *action {
                "start" => game.start(),
                "pause" => game.pause(),
                "toggle_pause" => game.toggle_pause(),
                _ => unreachable!("build-time validated action"),
            }
        }
        let input = INPUTS
            .iter()
            .find(|(from, to, _)| self.tick >= *from && self.tick < *to)
            .map_or(0.0, |(_, _, value)| *value as f32);
        game.step(STEP, input);
        self.tick += 1;
    }

    /// JSONL telemetry is state evidence, not evidence of rendered correctness.
    pub fn report(&self, game: &Game) -> String {
        format!("{{\"scenario\":\"{SCENARIO_ID}\",\"seed\":{SEED},\"tick\":{},\"state\":\"{:?}\",\"player\":{},\"cpu\":{},\"ball\":[{},{},{},{}],\"scores\":[{},{}],\"finished\":{}}}",
            self.tick, game.state, game.player, game.cpu, game.ball.x, game.ball.y,
            game.ball.vx, game.ball.vy, game.scores[0], game.scores[1], self.finished())
    }
}

/// Shared native demo clock and checkpoint policy. Rendering stays in adapters.
#[cfg(not(target_arch = "wasm32"))]
pub struct Demo {
    scenario: Scenario,
    accumulator: f32,
    directory: Option<std::path::PathBuf>,
    last_capture: u32,
    started: bool,
}
#[cfg(not(target_arch = "wasm32"))]
impl Demo {
    pub fn from_args() -> Option<Self> {
        let args: Vec<_> = std::env::args().collect();
        let enabled = args.iter().any(|a| a == "--scripted-test");
        let directory = args
            .iter()
            .position(|a| a == "--scripted-captures")
            .map(|i| {
                assert!(enabled, "--scripted-captures requires --scripted-test");
                let p = std::path::PathBuf::from(
                    args.get(i + 1).expect("missing NEW capture directory"),
                );
                std::fs::create_dir(&p).expect("create NEW capture directory");
                p
            });
        enabled.then(|| Self::new(directory))
    }
    pub fn new(directory: Option<std::path::PathBuf>) -> Self {
        Self {
            scenario: Scenario::default(),
            accumulator: 0.0,
            directory,
            last_capture: 0,
            started: false,
        }
    }
    pub fn tick(&self) -> u32 {
        self.scenario.tick()
    }
    pub fn finished(&self) -> bool {
        self.scenario.finished()
    }
    pub fn advance(&mut self, game: &mut Game, elapsed: f32) {
        if !self.started {
            println!("{}", self.scenario.report(game));
            self.started = true;
        }
        if !elapsed.is_finite() {
            return;
        }
        // Update and render callbacks may be separate. Hold a pending frame
        // without accumulating an unbounded backlog until the renderer takes it.
        if self.directory.is_some()
            && CAPTURE_TICKS.contains(&self.tick())
            && self.last_capture != self.tick()
        {
            return;
        }
        self.accumulator += elapsed.clamp(0.0, 0.1);
        while self.accumulator >= STEP && !self.finished() {
            self.scenario.advance(game);
            self.accumulator -= STEP;
            if self.tick().is_multiple_of(REPORT_EVERY) {
                println!("{}", self.scenario.report(game));
            }
            // Give the adapter a frame at every capture checkpoint, even on slow renders.
            if self.directory.is_some() && CAPTURE_TICKS.contains(&self.tick()) {
                break;
            }
        }
    }
    pub fn capture_path(&mut self) -> Option<std::path::PathBuf> {
        let tick = self.tick();
        if CAPTURE_TICKS.contains(&tick) && self.last_capture != tick {
            if let Some(dir) = &self.directory {
                self.last_capture = tick;
                return Some(dir.join(format!("tick-{tick:04}.png")));
            }
        }
        None
    }
    /// Encode the exact app-owned packed RGB buffer submitted by CPU renderers.
    pub fn save_rgb(path: &std::path::Path, pixels: &[u32], width: u32, height: u32) {
        assert_eq!(pixels.len(), width as usize * height as usize);
        let file = std::fs::File::create_new(path).expect("create capture file");
        let mut encoder = png::Encoder::new(std::io::BufWriter::new(file), width, height);
        encoder.set_color(png::ColorType::Rgb);
        encoder.set_depth(png::BitDepth::Eight);
        let mut writer = encoder.write_header().expect("write PNG header");
        let bytes: Vec<u8> = pixels
            .iter()
            .flat_map(|p| [(p >> 16) as u8, (p >> 8) as u8, *p as u8])
            .collect();
        writer.write_image_data(&bytes).expect("write PNG pixels");
    }
}
