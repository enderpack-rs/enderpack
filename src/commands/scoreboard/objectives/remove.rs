use crate::{arguments, data_types::resource};

arguments!(ScoreboardObjectivesRemove => "scoreboard objectives remove" {
    required {
        objective: resource::Objective
    };
});
