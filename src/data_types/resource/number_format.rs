#[derive(strum::Display, Clone)]
pub enum NumberFormat {
    #[strum(to_string = "")]
    Reset,
    #[strum(to_string = "blank")]
    Blank,
    #[strum(to_string = "fixed {0}")]
    Fixed(serde_json::Value),
    #[strum(to_string = "styled {0}")]
    Styled(serde_json::Value),
}
