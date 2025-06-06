use crate::{arguments, data_types::resource, prelude::Selector};

arguments!(ScoreboardPlayersRemove with T: Selector => "scoreboard players remove" {
    required {
        target: T,
        objective: resource::Objective,
        score: i32
    };
});
