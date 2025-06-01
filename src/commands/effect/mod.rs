pub mod give;

use give::EffectGive;

use crate::{
    command_setup,
    prelude::{Selector, resource},
    subcommand_setup,
};

pub struct Effect;

command_setup!(Effect => effect);
subcommand_setup!(Effect {
    new with Selector {
        give(selector: T, effect: resource::Effect) => EffectGive
    };
});
