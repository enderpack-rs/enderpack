#[derive(strum::Display, Clone)]
#[strum(serialize_all = "snake_case")]
pub enum ScoreboardSlot {
    List,
    Sidebar,
    #[strum(to_string = "sidebar.team.{0}")]
    SidebarTeam(super::color::Color),
    BelowName,
}
