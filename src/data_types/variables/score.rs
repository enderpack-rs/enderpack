use std::ops::{Add, Deref};

use crate::prelude::{
    Command, PlayerSelector, TargetSelector, objectives::ScoreboardObjectivesAdd, resource,
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

impl VariableInit<i32> for Score {
    fn new(name: &str, path: &str, value: i32) -> Self {
        let fake_player_name = format!(".{name}");
        let declaration = scoreboard()
            .objectives()
            .add(path, resource::Criteria::Dummy);
        let init: Vec<Box<dyn Command>> = vec![Box::new(scoreboard().players().set(
            PlayerSelector::new(&fake_player_name),
            path,
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
            .add(path, resource::Criteria::Dummy);
        let init: Vec<Box<dyn Command>> = vec![Box::new(scoreboard().players().operation(
            PlayerSelector::new(&fake_player_name),
            path,
            "=",
            PlayerSelector::new(&value.name),
            path,
        ))];
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
            .add(path, resource::Criteria::Dummy);
        let mut init = value;
        init.push(Box::new(scoreboard().players().operation(
            PlayerSelector::new(&fake_player_name),
            path,
            "=",
            PlayerSelector::new(".eax"),
            path,
        )));
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

// impl Add<i32> for Score {
//     type Output = Vec<Box<dyn Command>>;
//     fn add(self, rhs: i32) -> Self::Output {
//         let eax = PlayerSelector::new(".eax");
//         vec![
//             Box::new(scoreboard().players().operation(
//                 eax,
//                 &self.path,
//                 "=",
//                 PlayerSelector::new(&self.name),
//                 &self.path,
//             )),
//             Box::new(scoreboard().players().operation(
//                 eax,
//                 &self.path,
//                 "+=",
//                 ,
//                 source_objective,
//             )),
//         ]
//     }
// }
