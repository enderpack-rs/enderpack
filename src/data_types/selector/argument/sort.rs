#[derive(strum::Display, Clone)]
pub enum Sort {
    Nearest,
    Furthest,
    Random,
    Arbitrary,
}
