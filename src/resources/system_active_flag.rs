pub struct SystemActive {
    pub is_active: bool,
}

impl SystemActive {
    pub fn new(is_active: bool) -> Self {
        Self { is_active }
    }
}

impl Default for SystemActive {
    fn default() -> Self {
        Self::new(false)
    }
}
