// the descriptor the manager registers the mod from; it names the claim, the two keybinds and the settings schema
pub struct TogglesDescriptor;

impl TogglesDescriptor {
    pub const TEXT: &'static str = include_str!("../../descriptor.hcl");
}
