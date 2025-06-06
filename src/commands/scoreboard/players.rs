use crate::{arguments, data_types::resource, prelude::Selector, subcommands};

pub struct ScoreboardPlayers;

subcommands!(ScoreboardPlayers {
    unit {
        display() => ScoreboardPlayersDisplay,
        list() => ScoreboardPlayersList
    };
    new with Selector {
        set(target: T, objective: resource::Objective, score: i32) => ScoreboardPlayersSet,
        operation(
            target: T,
            target_objective: resource::Objective,
            operation: &str,
            source: T,
            source_objective: resource::Objective
        ) => ScoreboardPlayersOperation,
        get(target: T, objective: resource::Objective) => ScoreboardPlayersGet,
        add(target: T, objective: resource::Objective, score: i32) => ScoreboardPlayersAdd,
        remove(target: T, objective: resource::Objective, score: i32) => ScoreboardPlayersRemove,
        random(target: T, objective: resource::Objective, min: i32, max: i32) => ScoreboardPlayersRandom,
        reset(target: T, objective: resource::Objective) => ScoreboardPlayersReset,
        enable(target: T, objective: resource::Objective) => ScoreboardPlayersEnable
    };
});

arguments!(ScoreboardPlayersSet with T: Selector => "scoreboard players set" {
    required {
        target: T,
        objective: resource::Objective,
        score: i32
    };
});

arguments!(ScoreboardPlayersOperation with T: Selector => "scoreboard players operation" {
    required {
        target: T,
        target_objective: resource::Objective,
        #[new(into)]
        operation: String,
        source: T,
        source_objective: resource::Objective
    };
});

arguments!(ScoreboardPlayersList => "scoreboard players list" {});

subcommands!(ScoreboardPlayersList {
    new with Selector {
        target(target: T) => ScoreboardPlayersListTarget
    };
});

arguments!(ScoreboardPlayersListTarget with T: Selector => "scoreboard players list" {
    required {
        target: T
    };
});

arguments!(ScoreboardPlayersGet with T: Selector => "scoreboard players get" {
    required {
        target: T,
        objective: resource::Objective
    };
});

arguments!(ScoreboardPlayersAdd with T: Selector => "scoreboard players add" {
    required {
        target: T,
        objective: resource::Objective,
        score: i32
    };
});

arguments!(ScoreboardPlayersRemove with T: Selector => "scoreboard players remove" {
    required {
        target: T,
        objective: resource::Objective,
        score: i32
    };
});

arguments!(ScoreboardPlayersRandom with T: Selector => "scoreboard players random" {
    required {
        target: T,
        objective: resource::Objective,
        min: i32,
        max: i32
    };
});

arguments!(ScoreboardPlayersReset with T: Selector => "scoreboard players reset" {
    required {
        target: T,
        objective: resource::Objective
    };
});

arguments!(ScoreboardPlayersEnable with T: Selector => "scoreboard players enable" {
    required {
        target: T,
        objective: resource::Objective
    };
});

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
