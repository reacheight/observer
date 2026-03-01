use std::fmt;

#[derive(Debug)]
pub struct GameTime {
    pub pre_game_started: bool,
    pub game_started: bool,
    pub time_seconds: f32,
}

impl GameTime {
    pub fn new(pre_game_started: bool, game_started: bool, time_seconds: f32) -> Self {
        GameTime {
            pre_game_started,
            game_started,
            time_seconds,
        }
    }
}

impl fmt::Display for GameTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if !self.pre_game_started {
            return write!(f, "game not started");
        }

        let seconds_abs = self.time_seconds.abs() as i32;
        let minutes = seconds_abs / 60;
        let seconds = seconds_abs % 60;

        let seconds_str = if seconds < 10 {
            format!("0{seconds}")
        } else {
            format!("{seconds}")
        };

        let sign = if self.game_started { "" } else { "-" };

        write!(f, "{sign}{minutes}:{seconds_str}")
    }
}

impl Default for GameTime {
    fn default() -> Self {
        Self::new(false, false, 0.0)
    }
}
