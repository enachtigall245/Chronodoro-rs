use anyhow::Result;
use std::time::Duration;

pub fn play_chime() -> Result<()> {
    use rodio::{OutputStream, Sink, Source};
    let (_stream, handle) = OutputStream::try_default()?;
    let sink = Sink::try_new(&handle)?;
    let beep = SineWave::new(880.0).amplify(0.15).take_duration(Duration::from_millis(300));
    sink.append(beep);
    sink.detach();
    Ok(())
}

struct SineWave { freq_hz: f32, sample_rate: u32, t: f32 }
impl SineWave { fn new(freq_hz: f32) -> Self { Self { freq_hz, sample_rate: 44100, t: 0.0 } } }
impl Iterator for SineWave {
    type Item = f32;
    fn next(&mut self) -> Option<Self::Item> {
        let v = (2.0 * std::f32::consts::PI * self.freq_hz * self.t).sin();
        self.t += 1.0 / self.sample_rate as f32;
        Some(v)
    }
}
impl rodio::Source for SineWave {
    fn current_frame_len(&self) -> Option<usize> { None }
    fn channels(&self) -> u16 { 1 }
    fn sample_rate(&self) -> u32 { self.sample_rate }
    fn total_duration(&self) -> Option<std::time::Duration> { None }
}