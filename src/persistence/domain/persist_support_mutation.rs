use board::domain::board::Board;
use board::domain::space::Space;
use game_definitions::support_levels::SupportLevels;
use players::domain::mutation_types::MutationTypes;
use players::domain::space_mutation::SpaceMutation;

pub fn persist_support_mutation(mutation: &SpaceMutation, board: &mut Board) -> Result<(), String> {
    match mutation.get_mutation_type() {
        MutationTypes::ShiftDown => {
            match board.get_support_level_of_space(*mutation.get_space_identifier())?{
                SupportLevels::ActiveSupport => board.get_space_mut(*mutation.get_space_identifier())?.set_support_level(SupportLevels::PassiveSupport)?,
                SupportLevels::PassiveSupport => board.get_space_mut(*mutation.get_space_identifier())?.set_support_level(SupportLevels::Neutral)?,
                SupportLevels::Neutral => board.get_space_mut(*mutation.get_space_identifier())?.set_support_level(SupportLevels::PassiveOpposition)?,
                SupportLevels::PassiveOpposition => board.get_space_mut(*mutation.get_space_identifier())?.set_support_level(SupportLevels::ActiveOpposition)?,
                SupportLevels::ActiveOpposition => panic!("Attempted to persist shifting down the support level of a space that was already at ActiveOpposition. Something is wrong with the calling code."),
            }
        }
        _ => panic!(
            "Attempting to perform a space mutation type 'Support', but {:?} wasn't implemented.",
            mutation.get_mutation_type()
        ),
    }

    Ok(())
}
