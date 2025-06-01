use std::fmt::Display;

use crate::prelude::Command;

pub struct ScoreboardObjectivesList;

impl Display for ScoreboardObjectivesList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "scoreboard objectives list")
    }
}

impl Command for ScoreboardObjectivesList {}
