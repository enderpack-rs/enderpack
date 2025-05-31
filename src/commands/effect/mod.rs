pub mod give;

use give::EffectGive;

use crate::prelude::{Selector, resource::EffectResource};

pub struct Effect;

impl Effect {
    pub fn give<T: Selector>(self, selector: T, effect: EffectResource) -> EffectGive<T> {
        EffectGive::new(selector, effect)
    }
}

pub fn effect() -> Effect {
    Effect {}
}
