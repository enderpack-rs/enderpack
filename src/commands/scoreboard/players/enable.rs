use crate::{arguments, data_types::resource, prelude::Selector};

arguments!(ScoreboardPlayersEnable with T: Selector => "scoreboard players enable" {
    required {
        target: T,
        objective: resource::Objective
    };
});
