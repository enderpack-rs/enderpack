use crate::{arguments, data_types::resource, prelude::Selector};

arguments!(ScoreboardPlayersAdd with T: Selector => "scoreboard players add" {
    required {
        target: T,
        objective: resource::Objective,
        score: i32
    };
});
