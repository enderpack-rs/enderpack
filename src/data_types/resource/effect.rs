#[derive(strum::Display, Clone)]
pub enum Effect {
    Speed,
    Slowness,
    Custom(String),
}
