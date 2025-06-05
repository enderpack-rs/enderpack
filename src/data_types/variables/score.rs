use crate::prelude::{
    Command, PlayerSelector, objectives::add::ScoreboardObjectivesAdd, resource, scoreboard,
};

use super::{Variable, VariableInit};

pub struct Score {
    pub name: String,
    path: String,
    /// Full path, formatted like module_path.function
    declaration: ScoreboardObjectivesAdd,
    stack: Vec<Box<dyn Command>>,
}

impl VariableInit<i32> for Score {
    fn new(name: &str, path: &str, value: i32) -> Self {
        let fake_player_name = format!(".{name}");
        let declaration = scoreboard()
            .objectives()
            .add(path, resource::Criteria::Dummy);
        let stack: Vec<Box<dyn Command>> = vec![Box::new(scoreboard().players().set(
            PlayerSelector::new(&fake_player_name),
            path,
            value,
        ))];
        Self {
            name: fake_player_name,
            path: path.to_owned(),
            declaration,
            stack,
        }
    }
}

impl VariableInit<Score> for Score {
    fn new(name: &str, path: &str, value: Score) -> Self {
        let fake_player_name = format!(".{name}");
        let declaration = scoreboard()
            .objectives()
            .add(path, resource::Criteria::Dummy);
        let mut stack: Vec<Box<dyn Command>> = value.stack;
        stack.push(Box::new(scoreboard().players().operation(
            PlayerSelector::new(&fake_player_name),
            path,
            "=",
            if stack.is_empty() {
                PlayerSelector::new(&value.name)
            } else {
                PlayerSelector::new(".eax")
            },
            path,
        )));
        Self {
            name: fake_player_name,
            path: path.to_owned(),
            declaration,
            stack,
        }
    }
}

impl Variable for Score {
    fn get_declaration(&self) -> &impl Command {
        &self.declaration
    }
    fn get_stack(&self) -> &Vec<Box<dyn Command>> {
        &self.stack
    }
}
