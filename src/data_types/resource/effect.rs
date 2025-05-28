#[derive(strum::Display)]
pub enum EffectResource {
    Speed,
    Slowness,
    Custom(String),
}
