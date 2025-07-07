#[derive(strum::Display, Clone)]
#[strum(serialize_all = "lowercase")]
pub enum Criteria {
    Dummy,
    Trigger,
    DeathCount,
    PlayerKillCount,
    TotalKillCount,
    Health,
    Xp,
    Level,
    Food,
    Air,
    Armor,
    Stat,
}
