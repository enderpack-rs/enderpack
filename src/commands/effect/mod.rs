pub mod give;

use give::EffectGive;

use crate::{
    factory,
    prelude::{Selector, resource},
    subcommands,
};

factory!(Effect => effect);
subcommands!(Effect {
    new with Selector {
        give(selector: T, effect: resource::Effect) => EffectGive
    };
});
