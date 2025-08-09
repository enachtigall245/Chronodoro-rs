#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Phase { Focus, ShortBreak, LongBreak }

impl Phase {
    pub fn label(self) -> &'static str {
        match self {
            Phase::Focus => "Focus",
            Phase::ShortBreak => "Short Break",
            Phase::LongBreak => "Long Break",
        }
    }
}