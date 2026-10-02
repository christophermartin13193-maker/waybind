use super::Args;

pub enum Step {
    Init(Args),
    // Health
    Status,
    Reload,
    Get(String),
    List,
    // Bind
    Add,
    Mod,
    Del(String),
    // Else
    Exit,
}
