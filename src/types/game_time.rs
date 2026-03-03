use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GamePhase {
    NotStarted,
    PreGame,
    InGame,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GameTime {
    pub game_phase: GamePhase,
    pub time_seconds: f32,
}

impl fmt::Display for GameTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.game_phase == GamePhase::NotStarted {
            return write!(f, "game not started");
        }

        let seconds_abs = self.time_seconds.abs() as i32;
        let minutes = seconds_abs / 60;
        let seconds = seconds_abs % 60;

        let sign = if self.game_phase == GamePhase::InGame {
            ""
        } else {
            "-"
        };

        write!(f, "{sign}{minutes}:{seconds:02}")
    }
}

impl Default for GameTime {
    fn default() -> Self {
        Self {
            game_phase: GamePhase::NotStarted,
            time_seconds: 0.0,
        }
    }
}
