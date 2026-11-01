use super::{ResetCause, ToggleKind};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToggleInput {
    Chord(ToggleKind),
    KeyDown { vk: u32, repeat: bool },
    Reset(ResetCause),
    ManagerReleased,
    ConfigChanged,
}
