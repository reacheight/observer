mod observers;
mod traits;
mod types;

use std::{cell::RefCell, env, fs::File, rc::Rc, time::Instant};

use anyhow::{Context as _, Ok};
use source2_demo::{FieldValue, prelude::*};

use observers::{GameTimeObserver, PeriodicObserver, PeriodicObserverRoutine, WardsObserver};
use types::{GameTime, Team};

#[derive(Default)]
struct NetworthLeaderRoutine {
    leaders: Vec<Team>,
}

impl NetworthLeaderRoutine {
    fn calculate_team_top_networth(data: &Entity) -> anyhow::Result<i32> {
        let networth = (0..=4)
            .map(|i| data.get_property_by_name(&format!("m_vecDataTeam.{i:04}.m_iNetWorth")))
            .collect::<Result<Vec<&FieldValue>, _>>()
            .context("Couldn't get networth values from entity")?
            .into_iter()
            .map(|v| v.try_into())
            .collect::<Result<Vec<i32>, _>>()
            .context("Couldn't convert networth values")?;

        networth
            .into_iter()
            .max()
            .context("Couldn't get networth from entity")
    }
}

impl PeriodicObserverRoutine for NetworthLeaderRoutine {
    fn on_iteration(&mut self, ctx: &Context, _current_time: GameTime) -> ObserverResult {
        let radiant_data = ctx.entities().get_by_class_name("CDOTA_DataRadiant")?;
        let dire_data = ctx.entities().get_by_class_name("CDOTA_DataDire")?;

        let radiant_top_nw = NetworthLeaderRoutine::calculate_team_top_networth(radiant_data)?;
        let dire_top_nw = NetworthLeaderRoutine::calculate_team_top_networth(dire_data)?;

        let leader = if radiant_top_nw > dire_top_nw {
            Team::Radiant
        } else {
            Team::Dire
        };

        self.leaders.push(leader);

        Ok(())
    }
}

fn main() -> anyhow::Result<()> {
    let args = env::args().skip(1);

    let start = Instant::now();

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

        let game_time_obs = parser.register_observer::<GameTimeObserver>();
        let wards_observer = parser.register_observer::<WardsObserver>();

        wards_observer
            .borrow_mut()
            .add_game_time_obs(game_time_obs.clone());

        let networth_leader_observer = parser.register_observer::<PeriodicObserver>();
        let networth_leader_routine = Rc::new(RefCell::new(NetworthLeaderRoutine::default()));

        networth_leader_observer.borrow_mut().init(
            60,
            game_time_obs.clone(),
            networth_leader_routine.clone(),
        );

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

        println!(
            "Networth leader every 30 seconds: {:?}",
            networth_leader_routine.borrow().leaders
        );
    }

    println!("Parsing finished in {:?}", start.elapsed());

    Ok(())
}
