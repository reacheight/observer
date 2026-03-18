use std::{cell::RefCell, rc::Rc};

use source2_demo::prelude::*;

use crate::{observers::GameTimeObserver, types::GamePhase};

#[derive(Default)]
pub struct PeriodicObserver {
    interval_seconds: f32,
    iteration_number: i32,
    game_time: Rc<RefCell<GameTimeObserver>>,
}

#[observer]
#[uses_entities]
impl PeriodicObserver {
    pub fn init(&mut self, interval_seconds: f32, game_time: Rc<RefCell<GameTimeObserver>>) {
        self.interval_seconds = interval_seconds;
        self.game_time = game_time;
        self.iteration_number = 0;
    }

    #[on_tick_start]
    fn on_tick_start(&mut self, ctx: &Context) -> ObserverResult {
        let eps: f32 = 0.001;

        let curent_time = self.game_time.borrow().calculate_game_time(ctx);

        if curent_time.game_phase != GamePhase::InGame
            || self.iteration_number as f32 * self.interval_seconds - curent_time.time_seconds > eps
        {
            return Ok(());
        }

        println!("observer triggered, curent_time is {curent_time}");
        self.iteration_number += 1;

        Ok(())
    }
}
