use crate::{arguments, data_types::resource, prelude::Selector};

arguments!(ScoreboardPlayersSet with T: Selector => "scoreboard players set" {
    required {
        target: T,
        objective: resource::Objective,
        score: i32
    };
});
