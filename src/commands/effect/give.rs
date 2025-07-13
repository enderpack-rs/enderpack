use std::fmt::Display;

use case::CaseExt;
use derive_new::new;

use crate::prelude::{Command, Selector, resource};

#[derive(new, Clone)]
pub struct EffectGive<T: Selector> {
    selector: T,
    effect: resource::Effect,
    #[new(default)]
    duration: Option<i32>,
}

impl<T: Selector> EffectGive<T> {
    pub fn duration(mut self, duration: i32) -> Self {
        self.duration = Some(duration);
        self
    }
}

impl<T: Selector> Command for EffectGive<T> {}

impl<T: Selector> Display for EffectGive<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "effect give {} {}",
            self.selector,
            match &self.effect {
                resource::Effect::Custom(name) => name.to_owned(),
                _ => format!("minecraft:{}", self.effect.to_string().to_snake()),
            }
        )?;
        for arg in [&self.duration].into_iter().flatten() {
            write!(f, " {arg}")?;
        }
        Ok(())
    }
}
