use rodio::Source;
use std::f32::consts::PI;
use std::iter::Iterator;
use std::time::Duration;

pub struct SineWave {
    sample_rate: u32,
    frequency: f32,
    amplitude: f32,
    t: f32,
}

impl SineWave {
    pub fn new(frequency: f32, amplitude: f32) -> Self {
        SineWave {
            sample_rate: 44100,
            frequency,
            amplitude,
            t: 0.0,
        }
    }
}

impl Iterator for SineWave {
    type Item = f32;

    fn next(&mut self) -> Option<f32> {
        let sample = (2.0 * PI * self.frequency * self.t).sin() * self.amplitude;
        self.t += 1.0 / self.sample_rate as f32;
        Some(sample)
    }
}

impl Source for SineWave {
    fn current_frame_len(&self) -> Option<usize> {
        None
    }

    fn channels(&self) -> u16 {
        1
    }

    fn sample_rate(&self) -> u32 {
        self.sample_rate
    }

    fn total_duration(&self) -> Option<Duration> {
        None
    }
}
