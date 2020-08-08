pub struct AvailableForces {
    _arvn_available: u8,
}

impl AvailableForces {
    pub fn new() -> AvailableForces {
        AvailableForces { _arvn_available: 0 }
    }
}

impl Default for AvailableForces {
    fn default() -> Self {
        Self::new()
    }
}
