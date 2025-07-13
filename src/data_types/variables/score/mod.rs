pub mod add;

use crate::prelude::{
    Command, PlayerSelector,
    objectives::add::ScoreboardObjectivesAdd,
    resource::{self, Objective},
    scoreboard,
};

use super::{Variable, VariableInit};

pub struct Score {
    name: String,
    path: String,
    /// Full path, formatted like module_path.function
    declaration: ScoreboardObjectivesAdd,
    init: Vec<Box<dyn Command>>, // Can be a ScoreboardPlayersSet or ScoreboardPlayersOperation
}

const INTERMEDIATE: &str = ".eax";

impl VariableInit<i32> for Score {
    fn new(name: &str, path: &str, value: i32) -> Self {
        let fake_player_name = format!(".{name}");
        let declaration = scoreboard()
            .objectives()
            .add(Objective::new(path).unwrap(), &resource::Criteria::Dummy);
        let init: Vec<Box<dyn Command>> = vec![Box::new(scoreboard().players().set(
            PlayerSelector::new(&fake_player_name),
            Objective::new(path).unwrap(),
            value,
        ))];
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
            .add(Objective::new(path).unwrap(), resource::Criteria::Dummy);

        let copy_from_value = Box::new(scoreboard().players().operation(
            PlayerSelector::new(&fake_player_name),
            Objective::new(path).unwrap(),
            "=",
            PlayerSelector::new(&value.name),
            Objective::new(path).unwrap(),
        ));

        let init: Vec<Box<dyn Command>> = if dbg!(&value.name) == INTERMEDIATE {
            // Operation result
            let mut previous_operations = value.init;
            previous_operations.push(copy_from_value);
            previous_operations
        } else {
            // Copy variable
            vec![copy_from_value]
        };
        Self {
            name: fake_player_name,
            path: path.to_owned(),
            declaration,
            init,
        }
    }
}

impl VariableInit<Vec<Box<dyn Command>>> for Score {
    fn new(name: &str, path: &str, value: Vec<Box<dyn Command>>) -> Self {
        let fake_player_name = format!(".{name}");
        let declaration = scoreboard()
            .objectives()
            .add(Objective::new(path).unwrap(), resource::Criteria::Dummy);
        dbg!(name);
        let mut init = value;
        if fake_player_name != INTERMEDIATE {
            init.push(Box::new(scoreboard().players().operation(
                PlayerSelector::new(&fake_player_name),
                Objective::new(path).unwrap(),
                "=",
                PlayerSelector::new(INTERMEDIATE),
                Objective::new(path).unwrap(),
            )));
        }
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
    fn get_init(&self) -> &Vec<Box<dyn Command>> {
        &self.init
    }
}
