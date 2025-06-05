use crate::{arguments, prelude::resource, subcommands};
use derive_new::new;

pub struct ScoreboardObjectives;

subcommands!(ScoreboardObjectives {
    unit {
        list() => ScoreboardObjectivesList
    };
    new {
        add(objective: &str, criteria: resource::Criteria) => ScoreboardObjectivesAdd
    };
});

arguments!(ScoreboardObjectivesList => "scoreboard objectives" {});

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
