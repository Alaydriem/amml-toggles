#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct KeyCommand {
    pub vk: u32,
    pub down: bool,
}
