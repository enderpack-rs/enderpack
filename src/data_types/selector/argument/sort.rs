#[derive(strum::Display, Clone)]
#[strum(serialize_all = "snake_case")]
pub enum Sort {
    Nearest,
    Furthest,
    Random,
    Arbitrary,
}
