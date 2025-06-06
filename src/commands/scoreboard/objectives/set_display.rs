use crate::{arguments, data_types::resource};

arguments!(ScoreboardObjectivesSetdisplay => "scoreboard objectives setdisplay" {
    required {
        slot: resource::ScoreboardSlot
    };
    optional {
        objective: resource::Objective
    };
});
