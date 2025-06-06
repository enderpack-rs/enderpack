use crate::{arguments, data_types::resource, prelude::Selector};

arguments!(ScoreboardPlayersRandom with T: Selector => "scoreboard players random" {
    required {
        target: T,
        objective: resource::Objective,
        min: i32,
        max: i32
    };
});
