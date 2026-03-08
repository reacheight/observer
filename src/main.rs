mod observers;
mod traits;
mod types;

use std::{env, fs::File};

use anyhow::Context as _;
use source2_demo::prelude::*;

use observers::{GameTimeObserver, WardsObserver};
use types::Team;

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

        let game_time_obs = parser.register_observer::<GameTimeObserver>();
        let wards_observer = parser.register_observer::<WardsObserver>();

        wards_observer
            .borrow_mut()
            .add_game_time_obs(game_time_obs.clone());

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
