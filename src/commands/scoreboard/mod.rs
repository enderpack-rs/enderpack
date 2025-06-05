use objectives::ScoreboardObjectives;
use players::ScoreboardPlayers;

use crate::{factory, subcommands};

pub mod objectives;
pub mod players;

factory!(Scoreboard => scoreboard);
subcommands!(Scoreboard {
    unit {
        objectives() => ScoreboardObjectives,
        players() => ScoreboardPlayers
    };
});
