use case::CaseExt;
use derive_new::new;

use crate::data_types::resource::effect::EffectResource;
use crate::data_types::selector::Selector;
use std::fmt::Display;

pub struct Effect;

impl Effect {
    pub fn give(self, selector: Selector, effect: EffectResource) -> EffectGive {
        EffectGive::new(selector, effect)
    }
}

#[derive(new)]
pub struct EffectGive {
    selector: Selector,
    effect: EffectResource,
}

impl Display for EffectGive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "effect give {} minecraft:{}",
            self.selector,
            match &self.effect {
                EffectResource::Custom(name) => name.to_owned(),
                _ => self.effect.to_string().to_snake(),
            }
        )
    }
}

pub fn effect() -> Effect {
    Effect {}
}
