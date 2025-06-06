use crate::{arguments, data_types::resource, prelude::Selector};

arguments!(ScoreboardPlayersOperation with T: Selector => "scoreboard players operation" {
    required {
        target: T,
        target_objective: resource::Objective,
        #[new(into)]
        operation: String,
        source: T,
        source_objective: resource::Objective
    };
});
