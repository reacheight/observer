use std::{cell::RefCell, rc::Rc};

use source2_demo::prelude::*;

use crate::{
    observers::GameTimeObserver,
    types::{GamePhase, GameTime},
};

const TIME_EPS: f32 = 0.001;

#[derive(Default)]
pub struct PeriodicObserver {
    interval_seconds: u32,
    game_time: Rc<RefCell<GameTimeObserver>>,
    routines: Vec<Rc<RefCell<dyn PeriodicObserverRoutine>>>,

    iteration_number: u32,
}

#[observer]
#[uses_entities]
impl PeriodicObserver {
    pub fn init(
        &mut self,
        interval_seconds: u32,
        game_time: Rc<RefCell<GameTimeObserver>>,
        routines: Vec<Rc<RefCell<dyn PeriodicObserverRoutine>>>,
    ) {
        self.interval_seconds = interval_seconds;
        self.game_time = game_time;
        self.routines = routines;
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

        for routine in &self.routines {
            routine.borrow_mut().on_iteration(ctx, curent_time);
        }

        self.iteration_number += 1;
        Ok(())
    }
}

pub trait PeriodicObserverRoutine {
    fn on_iteration(&mut self, ctx: &Context, current_time: GameTime);
}
