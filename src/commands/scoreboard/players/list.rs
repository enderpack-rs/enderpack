use crate::{arguments, prelude::Selector, subcommands};

arguments!(ScoreboardPlayersList => "scoreboard players list" {});

subcommands!(ScoreboardPlayersList {
    new with Selector {
        target(target: T) => ScoreboardPlayersListTarget
    };
});

arguments!(ScoreboardPlayersListTarget with T: Selector => "scoreboard players list" {
    required {
        target: T
    };
});
