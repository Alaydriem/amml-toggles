use amml_api::Logger;
use toggles_core::structs::ToggleKind;

// the mod's log lines: the link, each toggle on and off with its cause, a refused key, the link errors and the first
// death check answer of a world
pub struct TogglesLog {
    log: Logger,
    linked: bool,
    alive_seen: bool,
}

impl TogglesLog {
    pub fn new(log: Logger) -> Self {
        Self { log, linked: false, alive_seen: false }
    }

    pub fn linked(&mut self) {
        if !self.linked {
            self.linked = true;
            self.log.info("toggles linked", &[]);
        }
    }

    pub fn on(&self, kind: ToggleKind, vk: u32) {
        self.log.info("toggle on", &[("kind", Self::kind(kind)), ("key", &format!("{vk:#04x}"))]);
    }

    pub fn off(&self, kind: ToggleKind, reason: &str) {
        self.log.info("toggle off", &[("kind", Self::kind(kind)), ("reason", reason)]);
    }

    pub fn refused(&self, kind: ToggleKind, error: &str) {
        self.log.warn("key refused", &[("kind", Self::kind(kind)), ("error", error)]);
    }

    // the toggles stay off for the rest of this enable
    pub fn stopped(&self, reason: &str) {
        self.log.warn("toggles off", &[("reason", reason)]);
    }

    // the first answer of the death check after a world join
    pub fn alive(&mut self, alive: bool) {
        if !self.alive_seen {
            self.alive_seen = true;
            self.log.info("death check ready", &[("alive", if alive { "true" } else { "false" })]);
        }
    }

    // a world was left: the next world logs its first death check answer again
    pub fn world_left(&mut self) {
        self.alive_seen = false;
    }

    fn kind(kind: ToggleKind) -> &'static str {
        match kind {
            ToggleKind::Sneak => "sneak",
            ToggleKind::Walk => "walk",
        }
    }
}
