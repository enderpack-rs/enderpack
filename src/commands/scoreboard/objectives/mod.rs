use add::ScoreboardObjectivesAdd;
use list::ScoreboardObjectivesList;

use crate::{prelude::resource, subcommand_setup};

pub mod add;
pub mod list;
pub mod modify;
pub mod remove;
pub mod setdisplay;

pub struct ScoreboardObjectives;

subcommand_setup!(ScoreboardObjectives {
    unit {
        list() => ScoreboardObjectivesList
    };
    new {
        add(objective: &str, criteria: resource::Criteria) => ScoreboardObjectivesAdd
    };
});
