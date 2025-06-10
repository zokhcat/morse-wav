use std::{f32::consts::PI, time::Duration};

use rodio::Source;

pub struct SineWave {
    freq: f32,
    sample_rate: u32,
    sample_clock: f32,
    duration: Duration,
}

impl Iterator for SineWave {
    type Item = f32;
    fn next(&mut self) -> Option<Self::Item> {
        if self.sample_clock / self.sample_rate as f32 >= self.duration.as_secs_f32() {
            return None;
        }
        let sample = (2.0 * PI * self.freq * self.sample_clock / self.sample_rate as f32).sin();
        self.sample_clock += 1.0;
        Some(sample * 0.2)
    }
}

impl Source for SineWave {
    fn current_frame_len(&self) -> Option<usize> { None }
    fn channels(&self) -> u16 { 1 }
    fn sample_rate(&self) -> u32 { self.sample_rate }
    fn total_duration(&self) -> Option<Duration> { Some(self.duration) }
}

pub fn morse_dot() -> SineWave {
    SineWave { freq: 600.0, sample_rate: 44100, sample_clock: 0.0, duration: Duration::from_millis(100) }
}

pub fn morse_dash() -> SineWave {
    SineWave { freq: 600.0, sample_rate: 44100, sample_clock: 0.0, duration: Duration::from_millis(300) }
}