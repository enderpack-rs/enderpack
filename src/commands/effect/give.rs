use std::fmt::Display;

use case::CaseExt;
use derive_new::new;

use crate::prelude::{Selector, resource::EffectResource};

#[derive(new)]
pub struct EffectGive<T: Selector> {
    selector: T,
    effect: EffectResource,
    #[new(default)]
    duration: Option<i32>,
}

impl<T: Selector> EffectGive<T> {
    pub fn duration(mut self, duration: i32) -> Self {
        self.duration = Some(duration);
        self
    }
}

impl<T: Selector> Display for EffectGive<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "effect give {} minecraft:{}",
            self.selector,
            match &self.effect {
                EffectResource::Custom(name) => name.to_owned(),
                _ => self.effect.to_string().to_snake(),
            }
        )?;
        for arg in [&self.duration].into_iter().flatten() {
            write!(f, " {}", arg)?;
        }
        Ok(())
    }
}
