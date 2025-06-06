use crate::{data_types::resource, subcommands};

pub mod add;
pub mod list;

use add::ScoreboardObjectivesAdd;
use list::ScoreboardObjectivesList;

pub struct ScoreboardObjectives;

subcommands!(ScoreboardObjectives {
    unit {
        list() => ScoreboardObjectivesList
    };
    new {
        add(objective: resource::Objective, criteria: resource::Criteria) => ScoreboardObjectivesAdd
    };
});
