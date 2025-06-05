use operation::ScoreboardPlayersOperation;
use set::ScoreboardPlayersSet;

use crate::{prelude::Selector, subcommand_setup};

pub mod operation;
pub mod set;

pub struct ScoreboardPlayers;

subcommand_setup!(ScoreboardPlayers {
    new with Selector {
        set(selector: T, objective: &str, score: i32) => ScoreboardPlayersSet,
        operation(
            target: T,
            target_objective: &str,
            operation: &str,
            source: T,
            source_objective: &str
        ) => ScoreboardPlayersOperation
    };
});
