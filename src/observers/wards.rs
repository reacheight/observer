use std::{cell::RefCell, collections::HashMap, rc::Rc};

use source2_demo::prelude::*;

use crate::{
    observers::GameTimeObserver,
    traits::{WithLocation, WithPlayerId},
    types::{GameTime, Location, Team},
};

pub struct WardEntry {
    pub time: GameTime,
    pub location: Location,
}

pub struct WardsObserver {
    pub wards: HashMap<Team, Vec<WardEntry>>,
    game_time_obs: Rc<RefCell<GameTimeObserver>>,
}

impl Default for WardsObserver {
    fn default() -> Self {
        Self {
            wards: HashMap::from([(Team::Radiant, vec![]), (Team::Dire, vec![])]),
            game_time_obs: Rc::new(RefCell::new(GameTimeObserver::default())),
        }
    }
}

#[observer]
#[uses_entities]
impl WardsObserver {
    #[on_entity]
    fn on_entity(&mut self, ctx: &Context, event: EntityEvents, entity: &Entity) -> ObserverResult {
        if event == EntityEvents::Created && entity.class().name() == "CDOTA_NPC_Observer_Ward" {
            let team = entity.player_id()?.team();
            let time = self.game_time_obs.borrow().calculate_game_time(ctx);
            let location = entity.location()?;
            self.wards
                .entry(team)
                .and_modify(|entries| entries.push(WardEntry { time, location }));
        }

        Ok(())
    }

    pub fn add_game_time_obs(&mut self, game_time_obs: Rc<RefCell<GameTimeObserver>>) {
        self.game_time_obs = game_time_obs;
    }
}
