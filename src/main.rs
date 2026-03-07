mod traits;
mod types;

use std::{collections::HashMap, env, fs::File};

use anyhow::Context as AnyhowContext;
use source2_demo::{prelude::*, proto::CNetMsgTick};
use traits::{WithLocation, WithPlayerId};
use types::{GamePhase, GameTime, Location, Team};

struct WardEntry {
    time: GameTime,
    location: Location,
}

struct Wards {
    server_tick: u32,
    wards: HashMap<Team, Vec<WardEntry>>,
}

impl Default for Wards {
    fn default() -> Self {
        Self {
            server_tick: 0,
            wards: HashMap::from([(Team::Radiant, vec![]), (Team::Dire, vec![])]),
        }
    }
}

#[observer]
#[uses_entities]
impl Wards {
    #[on_entity]
    fn on_entity(&mut self, ctx: &Context, event: EntityEvents, entity: &Entity) -> ObserverResult {
        if event == EntityEvents::Created && entity.class().name() == "CDOTA_NPC_Observer_Ward" {
            let team = entity.player_id()?.team();
            let time = self.calculate_game_time(ctx);
            let location = entity.location()?;
            self.wards
                .entry(team)
                .and_modify(|entries| entries.push(WardEntry { time, location }));
        }

        Ok(())
    }

    #[on_message]
    fn on_message(&mut self, message: CNetMsgTick) -> ObserverResult {
        self.server_tick = message.tick();
        Ok(())
    }

    // TODO: learn how to insert Entities and incapsulate somewhere
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
                            return Some(GameTime {
                                game_phase: GamePhase::InGame,
                                time_seconds: time - start_time,
                            });
                        } else {
                            return Some(GameTime {
                                game_phase: GamePhase::PreGame,
                                time_seconds: time - transition_time,
                            });
                        }
                    }
                }

                Some(GameTime::default())
            })
            .unwrap_or_default()
    }

    fn time(&self) -> f32 {
        self.server_tick as f32 / 29.99999
    }
}

fn main() -> anyhow::Result<()> {
    let args = env::args().skip(1);

    for arg in args {
        let file =
            File::open(&arg).context(format!("can't open file passed as an argument: '{arg}'"))?;
        let mut parser = Parser::from_reader(file)
            .context(format!("can't create a parser for a file '{arg}'"))?;

        let match_id = parser
            .replay_info()
            .game_info
            .as_ref()
            .and_then(|info| info.dota.as_ref())
            .and_then(|dota| dota.match_id)
            .context(format!(
                "can't get match id from a parser for a file '{arg}'"
            ))?;

        let wards_observer = parser.register_observer::<Wards>();

        println!("Starting to parse match {}!", match_id);
        parser
            .run_to_end()
            .context(format!("error during parsing match {match_id}"))?;
        println!("Finished parsing {}:\n", match_id);
        for team in [Team::Radiant, Team::Dire] {
            println!("Observer wards placed by {team}:");
            for ward in wards_observer.borrow().wards.get(&team).unwrap_or(&vec![]) {
                println!("{} at {}", ward.time, ward.location);
            }
            println!();
        }
    }

    Ok(())
}
