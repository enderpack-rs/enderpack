use std::fmt::Display;

use derive_new::new;

use crate::prelude::{Command, Selector};

#[derive(new)]
pub struct ScoreboardPlayersSet<T: Selector> {
    selector: T,
    #[new(into)]
    objective: String,
    score: i32,
}

impl<T: Selector> Display for ScoreboardPlayersSet<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "scoreboard players set {} {} {}",
            self.selector, self.objective, self.score
        )
    }
}

impl<T: Selector> Command for ScoreboardPlayersSet<T> {}
