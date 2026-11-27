use amml_api::{ApiError, Input, InputBlocks, InputLease, KeyCode, KeyHold, LeaseMode, LeaseRequest, LeaseState};

// the game keys the toggles hold: one input lease, taken at the first press and ended when the last hold goes, and
// one hold per pressed key. The lease blocks no real input, so the player keeps the keys, buttons and the look
pub struct KeyHolds {
    input: Input,
    // dropped before the lease, so each key goes up by its own hold
    holds: Vec<(u32, KeyHold)>,
    lease: Option<InputLease>,
}

impl KeyHolds {
    pub const REQUEST: LeaseRequest =
        LeaseRequest { mode: LeaseMode::Shared, blocks: InputBlocks::NONE, passthrough: &[], preempt: false };

    pub fn new(input: Input) -> Self {
        Self { input, holds: Vec::with_capacity(2), lease: None }
    }

    // presses `vk` in the game; takes the lease first when there is none (or the loader lost it). An error means
    // nothing is held for `vk`, and a lease with no hold left is ended
    pub fn press(&mut self, vk: u32) -> Result<(), ApiError> {
        let key = u16::try_from(vk).map_err(|_| ApiError::Refused)?;
        if self.lease.as_ref().is_some_and(|lease| lease.state() == LeaseState::Lost) {
            self.forget_all();
        }
        if self.lease.is_none() {
            self.lease = Some(self.input.acquire(Self::REQUEST)?);
        }
        let pressed = match &self.lease {
            Some(lease) => lease.key_down(KeyCode(key)),
            None => Err(ApiError::Refused),
        };
        match pressed {
            Ok(hold) => {
                self.holds.push((vk, hold));
                Ok(())
            }
            Err(error) => {
                self.end_when_empty();
                Err(error)
            }
        }
    }

    // lets `vk` go in the game; the lease ends with the last hold
    pub fn release(&mut self, vk: u32) {
        self.holds.retain(|(held, _)| *held != vk);
        self.end_when_empty();
    }

    // the loader released `vk` already: the record goes (a release of a released hold has no effect)
    pub fn forget(&mut self, vk: u32) {
        self.release(vk);
    }

    // the loader released every input of the lease, or lost it: the holds and the lease go
    pub fn forget_all(&mut self) {
        self.holds.clear();
        self.lease = None;
    }

    // whether `lease` is the id of the lease these holds use
    pub fn owns(&self, lease: u64) -> bool {
        self.lease.as_ref().is_some_and(|held| held.id() == lease)
    }

    pub fn has_lease(&self) -> bool {
        self.lease.is_some()
    }

    fn end_when_empty(&mut self) {
        if self.holds.is_empty() {
            self.lease = None;
        }
    }
}
