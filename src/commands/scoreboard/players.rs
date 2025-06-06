use crate::{arguments, data_types::resource, prelude::Selector, subcommands};

pub struct ScoreboardPlayers;

subcommands!(ScoreboardPlayers {
    new with Selector {
        set(target: T, objective: resource::Objective, score: i32) => ScoreboardPlayersSet,
        operation(
            target: T,
            target_objective: resource::Objective,
            operation: &str,
            source: T,
            source_objective: resource::Objective
        ) => ScoreboardPlayersOperation,
        list() => ScoreboardPlayersList,
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

arguments!(ScoreboardPlayersList with T: Selector => "scoreboard players list" {
    optional {
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
