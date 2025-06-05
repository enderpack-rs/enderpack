use std::fmt::Display;

use derive_new::new;

use crate::prelude::{Command, Selector};

#[derive(new)]
pub struct ScoreboardPlayersOperation<T: Selector> {
    target: T,
    #[new(into)]
    target_objective: String,
    #[new(into)]
    operation: String,
    source: T,
    #[new(into)]
    source_objective: String,
}

impl<T: Selector> Display for ScoreboardPlayersOperation<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "scoreboard players operation {} {} {} {} {}",
            self.target, self.target_objective, self.operation, self.source, self.source_objective
        )
    }
}

impl<T: Selector> Command for ScoreboardPlayersOperation<T> {}
