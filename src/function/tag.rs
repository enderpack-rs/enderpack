#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Tag {
    Load,
    Tick,
    Custom(String),
    None,
}
