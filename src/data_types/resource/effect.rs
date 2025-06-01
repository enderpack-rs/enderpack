#[derive(strum::Display)]
pub enum Effect {
    Speed,
    Slowness,
    Custom(String),
}
