use std::fmt::Display;

use derive_new::new;

use crate::prelude::{Command, resource};

#[derive(new)]
pub struct ScoreboardObjectivesAdd {
    #[new(into)]
    pub objective: String,
    pub criteria: resource::Criteria,
    #[new(default)]
    display_name: Option<String>,
}

impl ScoreboardObjectivesAdd {
    pub fn display_name(mut self, display_name: &str) -> Self {
        self.display_name = Some(display_name.to_string());
        self
    }
}

impl Display for ScoreboardObjectivesAdd {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "scoreboard objectives add {} {}",
            self.objective,
            self.criteria.to_string().to_lowercase()
        )?;
        if let Some(display_name) = &self.display_name {
            write!(f, "{}", display_name)?;
        }
        Ok(())
    }
}

impl Command for ScoreboardObjectivesAdd {}
