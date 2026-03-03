use source2_demo::{Entity, property};

use crate::types::PlayerId;

pub trait WithPlayerId {
    fn player_id(&self) -> anyhow::Result<PlayerId>;
}

impl WithPlayerId for Entity {
    fn player_id(&self) -> anyhow::Result<PlayerId> {
        let player_id: u8 = property!(self, "m_nPlayerOwnerID");
        Ok(PlayerId::new(player_id)?)
    }
}
