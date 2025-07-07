use crate::{data_types::resource, prelude::Selector, subcommands};

pub mod add;
pub mod display;
pub mod enable;
pub mod get;
pub mod list;
pub mod operation;
pub mod random;
pub mod remove;
pub mod reset;
pub mod set;

use add::ScoreboardPlayersAdd;
use display::ScoreboardPlayersDisplay;
use enable::ScoreboardPlayersEnable;
use get::ScoreboardPlayersGet;
use list::ScoreboardPlayersList;
use operation::ScoreboardPlayersOperation;
use random::ScoreboardPlayersRandom;
use remove::ScoreboardPlayersRemove;
use reset::ScoreboardPlayersReset;
use set::ScoreboardPlayersSet;

pub struct ScoreboardPlayers;

subcommands!(ScoreboardPlayers {
    unit {
        display() => ScoreboardPlayersDisplay,
        list() => ScoreboardPlayersList
    };
    new with Selector {
        set(target: T, objective: resource::Objective, score: i32) => ScoreboardPlayersSet,
        operation(
            target: T,
            target_objective: resource::Objective,
            operation: str,
            source: T,
            source_objective: resource::Objective
        ) => ScoreboardPlayersOperation,
        get(target: T, objective: resource::Objective) => ScoreboardPlayersGet,
        add(target: T, objective: resource::Objective, score: i32) => ScoreboardPlayersAdd,
        remove(target: T, objective: resource::Objective, score: i32) => ScoreboardPlayersRemove,
        random(target: T, objective: resource::Objective, min: i32, max: i32) => ScoreboardPlayersRandom,
        reset(target: T, objective: resource::Objective) => ScoreboardPlayersReset,
        enable(target: T, objective: resource::Objective) => ScoreboardPlayersEnable
    };
});
