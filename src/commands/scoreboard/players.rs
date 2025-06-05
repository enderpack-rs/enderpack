use crate::{arguments, prelude::Selector, subcommands};

pub struct ScoreboardPlayers;

subcommands!(ScoreboardPlayers {
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

arguments!(ScoreboardPlayersSet with T: Selector => "scoreboard players set" {
    required {
        selector: T,
        #[new(into)]
        objective: String,
        score: i32
    };
});

arguments!(ScoreboardPlayersOperation with T: Selector => "scoreboard players operation" {
    required {
        target: T,
        #[new(into)]
        target_objective: String,
        #[new(into)]
        operation: String,
        source: T,
        #[new(into)]
        source_objective: String
    };
});
