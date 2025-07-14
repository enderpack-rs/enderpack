#[derive(Debug, Clone)]
pub enum Tag {
    Load,
    Tick,
    Custom(String),
    None,
}
