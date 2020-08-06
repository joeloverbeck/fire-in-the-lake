
use super::command::Command;
use board::space::Spaces;
use board::space::Space;

struct ShiftSupportOfSpace<'a> {
    space: &'a mut Spaces,
    levels_to_shift: i8
}

impl ShiftSupportOfSpace<'_> {
    fn new(space: &mut Spaces, levels_to_shift: i8) -> ShiftSupportOfSpace {
        ShiftSupportOfSpace {
            space: space,
            levels_to_shift: levels_to_shift
        }
    }
}

impl Command for ShiftSupportOfSpace<'_> {

    fn execute(&mut self) -> std::result::Result<(), std::string::String> { 
        // Shifting the support of a space has a particular logic that's encapsulated in the 
        // Support object contained within the appropriate space. It handles its own logic.

        if self.levels_to_shift == -1 {
            self.space.shift_support_level_down();
        }
        else {
            todo!();
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    use board::support::SupportLevels;
    use board::province::Province;
    use board::space_identifiers::SpaceIdentifiers;

    #[test]
    fn should_be_able_to_create_shift_support_of_space_command() {
        let mut space = Province::new(SpaceIdentifiers::Saigon).into();

        let _ = ShiftSupportOfSpace::new(&mut space, -1);
    }

    #[test]
    fn on_executing_the_shift_of_support_of_a_space_by_level_specified_the_level_should_have_dropped_in_the_space(){
        let mut space: Spaces = Province::new(SpaceIdentifiers::Saigon).into();

        space.set_support_level(SupportLevels::PassiveSupport);

        let mut command = ShiftSupportOfSpace::new(&mut space, -1);

        if let Err(error) = command.execute() {
            panic!(error);
        }        

        assert_eq!(space.get_current_support_level(), SupportLevels::Neutral, "The space's support after shifing down a level should have been neutral.");
    }

}