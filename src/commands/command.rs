use commands::manipulate_aid::ManipulateAid;
use commands::shift_support_of_space::ShiftSupportOfSpace;

extern crate enum_dispatch;
use self::enum_dispatch::enum_dispatch;

#[enum_dispatch]
pub trait Command {
    fn execute(&mut self) -> Result<(), String>;
}

#[enum_dispatch(Command)]
#[derive(Debug)]
pub enum Commands<'a> {
    ManipulateAid(ManipulateAid<'a>),
    ShiftSupportOfSpace(ShiftSupportOfSpace<'a>),
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    use board::map_builder::MapBuilder;
    use board::space_identifiers::SpaceIdentifiers;
    use board::track::Track;

    use std::collections::VecDeque;

    #[test]
    fn test_should_be_able_to_create_a_vecdeque_with_different_struct_commands(
    ) -> Result<(), String> {
        let mut buf: VecDeque<Commands> = VecDeque::new();

        let mut track = Track::new();

        let map_builder = MapBuilder::new();
        let mut built_map = map_builder.build_initial_map().unwrap();

        let shift_support_of_space: Commands = ShiftSupportOfSpace::new(
            built_map.get_space_mut(SpaceIdentifiers::Saigon).unwrap(),
            -1,
        )
        .into();
        let manipulate_aid: Commands = ManipulateAid::new(&mut track, -1).into();

        buf.push_back(manipulate_aid);
        buf.push_back(shift_support_of_space);

        // Now execute all
        buf.iter_mut()
            .for_each(|command| command.execute().unwrap());

        Ok(())
    }
}
