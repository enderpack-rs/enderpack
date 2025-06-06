use std::fmt::Display;

pub enum NumberFormat {
    Reset,
    Blank,
    Fixed(serde_json::Value),
    Styled(serde_json::Value),
}

impl Display for NumberFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Reset => Ok(()),
            Self::Blank => write!(f, "blank"),
            Self::Fixed(contents) => write!(f, "fixed {}", contents),
            Self::Styled(style) => write!(f, "styled {}", style),
        }
    }
}
