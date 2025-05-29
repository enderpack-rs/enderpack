pub mod give;

use give::EffectGive;

use crate::data_types::resource::effect::EffectResource;
use crate::data_types::selector::Selector;

pub struct Effect;

impl Effect {
    pub fn give(self, selector: Selector, effect: EffectResource) -> EffectGive {
        EffectGive::new(selector, effect)
    }
}

pub fn effect() -> Effect {
    Effect {}
}
