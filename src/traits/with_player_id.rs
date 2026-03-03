use source2_demo::{
    Entity,
    error::{EntityError, FieldValueError},
    property,
};

use crate::types::{PlayerId, PlayerIdParseError};

pub trait WithPlayerId {
    fn player_id(&self) -> Result<PlayerId, PlayerIdError>;
}

impl WithPlayerId for Entity {
    fn player_id(&self) -> Result<PlayerId, PlayerIdError> {
        let player_id: u8 = property!(self, "m_nPlayerOwnerID");
        Ok(PlayerId::new(player_id)?)
    }
}

#[derive(thiserror::Error, Debug)]
pub enum PlayerIdError {
    #[error(transparent)]
    EntityError(#[from] EntityError),

    #[error(transparent)]
    PropertyValueError(#[from] FieldValueError),

    #[error(transparent)]
    InvalidValue(#[from] PlayerIdParseError),
}
