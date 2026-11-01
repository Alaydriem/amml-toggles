#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResetCause {
    Menu,
    Death,
    FocusLost,
    WorldExit,
    Disabled,
}
