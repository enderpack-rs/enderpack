use crate::{arguments, data_types::resource, prelude::Selector};

arguments!(ScoreboardPlayersGet with T: Selector => "scoreboard players get" {
    required {
        target: T,
        objective: resource::Objective
    };
});
