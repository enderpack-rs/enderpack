use crate::{arguments, factory, prelude::Selector};

factory!(Tellraw => tellraw<T: Selector>(selector: T, message: serde_json::Value));

arguments!(Tellraw with T: Selector => "tellraw" {
    required {
        selector: T,
        message: serde_json::Value
    };
});
