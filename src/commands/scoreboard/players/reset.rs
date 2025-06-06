use crate::{arguments, data_types::resource, prelude::Selector};

arguments!(ScoreboardPlayersReset with T: Selector => "scoreboard players reset" {
    required {
        target: T,
        objective: resource::Objective
    };
});
