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
    pre_init: Vec<Box<dyn Command>>,
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
            pre_init: vec![],
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
            pre_init: vec![],
            init,
        }
    }
}

impl VariableInit<Vec<Box<dyn Command>>> for Score {
    fn new(name: &str, path: &str, value: Vec<Box<dyn Command>>) -> Self {
        let fake_player_name = format!(".{name}");
        let declaration = scoreboard()
            .objectives()
            .add(path, resource::Criteria::Dummy);
        let init = Box::new(scoreboard().players().operation(
            PlayerSelector::new(&fake_player_name),
            path,
            "=",
            PlayerSelector::new(".eax"),
            path,
        ));
        Self {
            name: fake_player_name,
            path: path.to_owned(),
            declaration,
            pre_init: value,
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
    fn get_pre_init(&self) -> &Vec<Box<dyn Command>> {
        &self.pre_init
    }
}
