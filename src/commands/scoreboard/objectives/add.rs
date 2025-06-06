use crate::{arguments, data_types::resource};

arguments!(ScoreboardObjectivesAdd => "scoreboard objectives add" {
    required {
        #[new(into)]
        objective: String,
        criteria: resource::Criteria
    };
    optional {
        display_name: String
    };
});
