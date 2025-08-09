use super::{config::Config, phase::Phase};
use std::time::Instant;

#[derive(Debug)]
pub struct Timer {
    pub phase: Phase,
    pub seconds_left: i64,
    pub running: bool,
    pub completed_focus_sessions: u32,
    pub last_tick: std::time::Instant,
    tick_accum: f64,
}

impl Timer {
    pub fn new(cfg: &Config) -> Self {
        Self {
            phase: Phase::Focus,
            seconds_left: (cfg.focus_minutes as i64) * 60,
            running: false,
            completed_focus_sessions: 0,
            tick_accum: 0.0,
            last_tick: Instant::now(),
        }
    }

    pub fn reset_phase(&mut self, cfg: &Config, phase: Phase) {
        self.phase = phase;
        self.seconds_left = match phase {
            Phase::Focus => (cfg.focus_minutes as i64) * 60,
            Phase::ShortBreak => (cfg.short_break_minutes as i64) * 60,
            Phase::LongBreak => (cfg.long_break_minutes as i64) * 60,
        };
        self.running = false;
        self.tick_accum = 0.0;
        self.last_tick = std::time::Instant::now();
    }

    pub fn tick(&mut self) -> bool {
        if !self.running {
            self.last_tick = std::time::Instant::now();
            return false;
        }
    
        let now = std::time::Instant::now();
        let dt = now.duration_since(self.last_tick);
        self.last_tick = now;
    
        self.tick_accum += dt.as_secs_f64();
        let mut finished = false;
    
        while self.tick_accum >= 1.0 {
            self.seconds_left -= 1;
            self.tick_accum -= 1.0;
    
            if self.seconds_left <= 0 {
                self.seconds_left = 0;
                self.running = false;
                self.tick_accum = 0.0;
                finished = true;
                break;
            }
        }
    
        finished
    }

    pub fn format_mmss(secs: i64) -> String {
        let m = secs.max(0) / 60;
        let s = secs.max(0) % 60;
        format!("{:02}:{:02}", m, s)
    }

    pub fn advance_phase(&mut self, cfg: &Config) {
        match self.phase {
            Phase::Focus => {
                self.completed_focus_sessions += 1;
                if self.completed_focus_sessions % cfg.sessions_until_long_break == 0 {
                    self.reset_phase(cfg, Phase::LongBreak);
                } else {
                    self.reset_phase(cfg, Phase::ShortBreak);
                }
            }
            Phase::ShortBreak | Phase::LongBreak => {
                self.reset_phase(cfg, Phase::Focus);
            }
        }
        if cfg.auto_start_next { self.running = true; }
    }
}