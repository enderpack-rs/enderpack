use crate::{arguments, data_types::resource, prelude::Selector, subcommands};

pub struct ScoreboardPlayersDisplay;

subcommands!(ScoreboardPlayersDisplay {
    new with Selector {
        name(target: T, objective: resource::Objective) => ScoreboardPlayersDisplayName,
        number_format(target: T, objective: resource::Objective, format: resource::NumberFormat) => ScoreboardPlayersDisplayNumberformat
    };
});

arguments!(ScoreboardPlayersDisplayName with T: Selector => "scoreboard players display name"{
    required {
        target: T,
        objective: resource::Objective
    };
});

arguments!(ScoreboardPlayersDisplayNumberformat with T: Selector => "scoreboard players display numberfomat" {
    required {
        target: T,
        objective: resource::Objective,
        format: resource::NumberFormat
    };
});
