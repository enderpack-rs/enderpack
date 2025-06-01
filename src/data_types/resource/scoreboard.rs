pub use Criteria::*;

#[derive(strum::Display)]
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
