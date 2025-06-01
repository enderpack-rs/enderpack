use objectives::ScoreboardObjectives;
use players::ScoreboardPlayers;

use crate::{command_setup, subcommand_setup};

pub mod objectives;
pub mod players;

pub struct Scoreboard;

command_setup!(Scoreboard => scoreboard);
subcommand_setup!(Scoreboard {
    unit {
        objectives() => ScoreboardObjectives,
        players() => ScoreboardPlayers
    };
});
