use crate::{arguments, data_types::resource};

arguments!(ScoreboardObjectivesAdd => "scoreboard objectives add" {
    required {
        #[new(into)]
        objective: resource::Objective,
        criteria: resource::Criteria
    };
    optional {
        display_name: String
    };
});
