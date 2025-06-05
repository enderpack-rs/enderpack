use std::ops::Deref;

use crate::prelude::{
    Command, PlayerSelector, objectives::add::ScoreboardObjectivesAdd,
    players::set::ScoreboardPlayersSet, resource, scoreboard,
};

use super::{Variable, VariableInit};

pub struct Score {
    pub name: String,
    path: String,
    /// Full path, formatted like module_path.function
    declaration: ScoreboardObjectivesAdd,
    init: Box<dyn Command>, // Can be a ScoreboardPlayersSet or ScoreboardPlayersOperation
}

impl VariableInit<i32> for Score {
    fn new(name: &str, path: &str, value: i32) -> Self {
        let fake_player_name = format!(".{name}");
        let declaration = scoreboard()
            .objectives()
            .add(path, resource::Criteria::Dummy);
        let init = Box::new(scoreboard().players().set(
            PlayerSelector::new(&fake_player_name),
            path,
            value,
        ));
        Self {
            name: fake_player_name,
            path: path.to_owned(),
            declaration,
            init,
        }
    }
}

impl VariableInit<Score> for Score {
    fn new(name: &str, path: &str, value: Score) -> Self {
        let fake_player_name = format!(".{name}");
        let declaration = scoreboard()
            .objectives()
            .add(path, resource::Criteria::Dummy);
        let init = Box::new(scoreboard().players().operation(
            PlayerSelector::new(&fake_player_name),
            path,
            "=",
            PlayerSelector::new(&value.name),
            path,
        ));
        Self {
            name: fake_player_name,
            path: path.to_owned(),
            declaration,
            init,
        }
    }
}

impl Variable for Score {
    fn get_declaration(&self) -> &impl Command {
        &self.declaration
    }
    fn get_init(&self) -> &(impl Command + ?Sized) {
        self.init.deref()
    }
}
