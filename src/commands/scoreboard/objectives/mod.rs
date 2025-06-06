use crate::{data_types::resource, subcommands};

pub mod add;
pub mod list;
pub mod modify;
pub mod remove;
pub mod set_display;

use add::ScoreboardObjectivesAdd;
use list::ScoreboardObjectivesList;
use modify::ScoreboardObjectivesModify;
use remove::ScoreboardObjectivesRemove;
use set_display::ScoreboardObjectivesSetdisplay;

pub struct ScoreboardObjectives;

subcommands!(ScoreboardObjectives {
    unit {
        list() => ScoreboardObjectivesList
    };
    new {
        add(objective: resource::Objective, criteria: resource::Criteria) => ScoreboardObjectivesAdd,
        modify(objective: resource::Objective, mode: modify::Mode) => ScoreboardObjectivesModify,
        remove(objective: resource::Objective) => ScoreboardObjectivesRemove,
        set_display(slot: resource::ScoreboardSlot) => ScoreboardObjectivesSetdisplay
    };
});
