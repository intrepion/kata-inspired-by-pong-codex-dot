//! Shared wall-clock frame-callback sampling. Not GPU time or displayed FPS.
use std::time::{Duration, Instant};
pub struct Frames {
    start: Instant,
    previous: Option<Instant>,
    intervals: Vec<f64>,
    width: u32,
    height: u32,
    resized: bool,
}
impl Frames {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            start: Instant::now(),
            previous: None,
            intervals: Vec::new(),
            width,
            height,
            resized: false,
        }
    }
    /// Invoke once at a consistent boundary of each actual rendering callback.
    /// Five seconds warmup; ten seconds measured; never measures simulation steps.
    pub fn frame(&mut self, width: u32, height: u32) -> bool {
        self.frame_at(Instant::now(), width, height)
    }
    fn frame_at(&mut self, now: Instant, width: u32, height: u32) -> bool {
        if now.duration_since(self.start) < Duration::from_secs(5) {
            // A tiling compositor can settle the requested size during startup.
            self.width = width;
            self.height = height;
            return false;
        }
        self.resized |= (width, height) != (self.width, self.height);
        if let Some(previous) = self.previous {
            self.intervals
                .push(now.duration_since(previous).as_secs_f64() * 1000.0);
        }
        self.previous = Some(now);
        now.duration_since(self.start) >= Duration::from_secs(15)
    }
    pub fn report(&self) -> String {
        let mut sorted = self.intervals.clone();
        sorted.sort_by(f64::total_cmp);
        let sum: f64 = sorted.iter().sum();
        format!("{{\"schema\":1,\"metric\":\"frame_callback_interval_ms\",\"warmup_seconds\":5,\"nominal_measure_seconds\":10,\"samples\":{},\"sampled_seconds\":{},\"mean_ms\":{},\"p50_ms\":{},\"p95_ms\":{},\"p99_ms\":{},\"max_ms\":{},\"width\":{},\"height\":{},\"resized\":{},\"intervals_ms\":{:?}}}", sorted.len(), sum / 1000., if sorted.is_empty() {0.} else {sum / sorted.len() as f64}, percentile(&sorted, 0.5), percentile(&sorted, 0.95), percentile(&sorted, 0.99), sorted.last().copied().unwrap_or(0.), self.width, self.height, self.resized, self.intervals)
    }
}
fn percentile(sorted: &[f64], p: f64) -> f64 {
    if sorted.is_empty() {
        return 0.;
    }
    sorted[((sorted.len() as f64 * p).ceil() as usize)
        .saturating_sub(1)
        .min(sorted.len() - 1)]
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn clock_warmup_measurement_and_resize() {
        let mut frames = Frames::new(1000, 680);
        let start = frames.start;
        assert!(!frames.frame_at(start + Duration::from_secs(4), 1000, 680));
        assert!(frames.previous.is_none());
        assert!(!frames.frame_at(start + Duration::from_secs(5), 1000, 680));
        assert!(!frames.frame_at(start + Duration::from_millis(5010), 1000, 680));
        assert_eq!(frames.intervals, vec![10.]);
        assert!(frames.frame_at(start + Duration::from_secs(15), 900, 680));
        assert!(frames.resized);
    }
    #[test]
    fn nearest_rank() {
        assert_eq!(percentile(&[], 0.5), 0.);
        assert_eq!(percentile(&[1., 2., 3., 4.], 0.5), 2.);
        assert_eq!(percentile(&[1., 2., 3., 4.], 0.95), 4.);
    }
    #[test]
    fn empty_report_is_finite() {
        let r = Frames::new(1000, 680).report();
        assert!(!r.contains("NaN"));
        assert!(r.contains("\"samples\":0"));
    }
}
