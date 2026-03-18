use crate::types::{GamePhase, GameTime};
use source2_demo::{prelude::*, proto::CNetMsgTick};

#[derive(Default)]
pub struct GameTimeObserver {
    server_tick: u32,
}

#[observer]
impl GameTimeObserver {
    #[on_message]
    fn on_message(&mut self, message: CNetMsgTick) -> ObserverResult {
        self.server_tick = message.tick();
        Ok(())
    }

    fn time(&self) -> f32 {
        self.server_tick as f32 / 29.99999
    }

    pub fn calculate_game_time(&self, ctx: &Context) -> GameTime {
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
                            let end_time: f32 =
                                try_property!(game_rules, "m_pGameRules.m_flGameEndTime")?;

                            return Some(GameTime {
                                game_phase: if end_time > time_eps {
                                    GamePhase::Ended
                                } else {
                                    GamePhase::InGame
                                },
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
}
