use std::{cell::RefCell, rc::Rc};

use source2_demo::prelude::*;

use crate::{observers::GameTimeObserver, types::GamePhase};

const TIME_EPS: f32 = 0.001;

#[derive(Default)]
pub struct PeriodicObserver {
    interval_seconds: i32,
    game_time: Rc<RefCell<GameTimeObserver>>,

    iteration_number: i32,
}

#[observer]
#[uses_entities]
impl PeriodicObserver {
    pub fn init(&mut self, interval_seconds: i32, game_time: Rc<RefCell<GameTimeObserver>>) {
        self.interval_seconds = interval_seconds;
        self.game_time = game_time;
        self.iteration_number = 0;
    }

    #[on_tick_start]
    fn on_tick_start(&mut self, ctx: &Context) -> ObserverResult {
        let curent_time = self.game_time.borrow().calculate_game_time(ctx);

        let iteration_target_time = self.iteration_number * self.interval_seconds;
        if curent_time.game_phase != GamePhase::InGame
            || iteration_target_time as f32 - curent_time.time_seconds > TIME_EPS
        {
            return Ok(());
        }

        println!("observer triggered, curent_time is {curent_time}");
        self.iteration_number += 1;

        Ok(())
    }
}
