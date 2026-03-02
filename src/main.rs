mod traits;
mod types;

use std::fs::File;

use source2_demo::{prelude::*, proto::CNetMsgTick};
use traits::Locationable;
use types::{GameTime, PlayerId};

#[derive(Default)]
struct Wards {
    server_tick: u32,
}

#[observer]
#[uses_entities]
impl Wards {
    #[on_entity]
    fn on_entity(&mut self, ctx: &Context, event: EntityEvents, entity: &Entity) -> ObserverResult {
        if event == EntityEvents::Created && entity.class().name() == "CDOTA_NPC_Observer_Ward" {
            let time = self.calculate_game_time(ctx);
            let owner_player_id = PlayerId::new(property!(entity, "m_nPlayerOwnerID"))?;
            let location = entity.location()?;
            println!(
                "Observer ward is placed by {}! Time: {time}, location: {location}",
                owner_player_id.team(),
            );
        }

        Ok(())
    }

    #[on_message]
    fn on_message(&mut self, message: CNetMsgTick) -> ObserverResult {
        self.server_tick = message.tick();
        Ok(())
    }

    fn calculate_game_time(&self, ctx: &Context) -> GameTime {
        ctx.entities()
            .get_by_class_name("CDOTAGamerulesProxy")
            .ok()
            .and_then(|game_rules| {
                let time_eps = 0.001;
                let time = self.time();

                if time > time_eps {
                    let pre_game_time: f32 =
                        try_property!(game_rules, "m_pGameRules.m_flPreGameStartTime")?;

                    if pre_game_time > time_eps {
                        let transition_time: f32 =
                            try_property!(game_rules, "m_pGameRules.m_flStateTransitionTime")?;
                        let start_time: f32 =
                            try_property!(game_rules, "m_pGameRules.m_flGameStartTime")?;

                        if start_time > time_eps {
                            return Some(GameTime::new(true, true, time - start_time));
                        } else {
                            return Some(GameTime::new(true, false, time - transition_time));
                        }
                    }
                }

                Some(GameTime::new(false, false, 0.0))
            })
            .unwrap_or_default()
    }

    fn time(&self) -> f32 {
        self.server_tick as f32 / 29.99999
    }
}

fn main() -> anyhow::Result<()> {
    const MATCH_ID: &str = "8710135523";
    let replay_file_path = format!("{MATCH_ID}.dem");

    let mut parser = Parser::from_reader(File::open(replay_file_path)?)?;
    parser.register_observer::<Wards>();

    parser.run_to_end()?;

    Ok(())
}
