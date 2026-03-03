use std::{error, fmt};

use crate::types::Team;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PlayerId(u8);

impl PlayerId {
    pub fn new(id: u8) -> Result<PlayerId, PlayerIdParseError> {
        if id > 18 || !id.is_multiple_of(2) {
            Err(PlayerIdParseError { bad_id: id })
        } else {
            Ok(PlayerId(id))
        }
    }

    pub fn team(&self) -> Team {
        if self.0 < 10 {
            Team::Radiant
        } else {
            Team::Dire
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct PlayerIdParseError {
    bad_id: u8,
}

impl fmt::Display for PlayerIdParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Can't initialize PlayerId from {}: id must be an even number from 0 to 18.",
            { self.bad_id }
        )
    }
}

impl error::Error for PlayerIdParseError {}
