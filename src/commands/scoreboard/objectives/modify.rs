use crate::{arguments, data_types::resource};

#[derive(strum::Display, Clone)]
pub enum Mode {
    #[strum(to_string = "displayautoupdate {0}")]
    DisplayAutoupdate(bool),
    #[strum(to_string = "displayname {0}")]
    DisplayName(serde_json::Value),
    #[strum(to_string = "numberformat {0}")]
    NumberFormat(resource::NumberFormat),
    #[strum(to_string = "rendertype {0}")]
    RenderType(RenderTypeEnum),
}

#[derive(strum::Display, Clone)]
#[strum(serialize_all = "lowercase")]
pub enum RenderTypeEnum {
    Hearts,
    Integer,
}

arguments!(ScoreboardObjectivesModify => "scoreboard objectives modify" {
    required {
        objective: resource::Objective,
        mode: Mode
    };
});
