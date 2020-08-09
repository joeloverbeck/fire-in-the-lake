use board::space::Space;
use board::space::Spaces;

#[derive(Debug)]
pub struct ShiftSupportOfSpace {}

impl Default for ShiftSupportOfSpace {
    fn default() -> Self {
        Self::new()
    }
}

impl ShiftSupportOfSpace {
    pub fn new() -> ShiftSupportOfSpace {
        ShiftSupportOfSpace {}
    }

    pub fn execute(
        &self,
        space: &mut Spaces,
        levels_to_shift: i8,
    ) -> std::result::Result<(), std::string::String> {
        // Shifting the support of a space has a particular logic that's encapsulated in the
        // Support object contained within the appropriate space. It handles its own logic.

        if levels_to_shift == -1 {
            space.shift_support_level_down();
        } else {
            space.shift_support_level_up();
        }

        Ok(())
    }
}
