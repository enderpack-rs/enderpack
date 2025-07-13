use std::ops::Add;

use crate::prelude::{Command, PlayerSelector, VariableInit, resource::Objective, scoreboard};

use super::{INTERMEDIATE, Score};

fn inner_add(score: &Score, rhs: i32) -> Score {
    let intermediate = PlayerSelector::new(INTERMEDIATE);
    let owning_scoreboard = Objective::new(&score.path).unwrap();

    let mut previous_operations: Vec<Box<dyn Command>> = score.init.to_vec();
    let copy_to_intermediate = Box::new(scoreboard().players().operation(
        &intermediate,
        &owning_scoreboard,
        "=",
        PlayerSelector::new(&score.name),
        &owning_scoreboard,
    )) as Box<dyn Command>;
    let addition = Box::new(scoreboard().players().add::<PlayerSelector>(
        &intermediate,
        &owning_scoreboard,
        rhs,
    )) as Box<dyn Command>;

    let init = if score.name == INTERMEDIATE {
        previous_operations.push(addition);
        previous_operations
    } else {
        vec![copy_to_intermediate, addition]
    };
    Score::new(INTERMEDIATE.trim_start_matches("."), &score.path, init)
}

impl Add<i32> for Score {
    type Output = Score;
    fn add(self, rhs: i32) -> Self::Output {
        inner_add(&self, rhs)
    }
}

impl Add<Score> for i32 {
    type Output = Score;
    fn add(self, rhs: Score) -> Self::Output {
        inner_add(&rhs, self)
    }
}

impl Add<i32> for &Score {
    type Output = Score;
    fn add(self, rhs: i32) -> Self::Output {
        inner_add(self, rhs)
    }
}

impl Add<&Score> for i32 {
    type Output = Score;
    fn add(self, rhs: &Score) -> Self::Output {
        inner_add(rhs, self)
    }
}
