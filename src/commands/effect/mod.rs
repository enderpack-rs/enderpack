pub mod give;

use give::EffectGive;

use crate::data_types::resource::effect::EffectResource;
use crate::data_types::selector::Selector;

pub struct Effect;

impl Effect {
    pub fn give<T: Selector>(self, selector: T, effect: EffectResource) -> EffectGive<T> {
        EffectGive::new(selector, effect)
    }
}

pub fn effect() -> Effect {
    Effect {}
}
