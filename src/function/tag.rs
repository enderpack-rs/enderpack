#[derive(Debug)]
pub enum Tag {
    Load,
    Tick,
    Custom(String),
    None,
}
