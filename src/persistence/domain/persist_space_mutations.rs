use board::domain::board::Board;
use board::domain::space::Space;
use persistence::domain::persist_support_mutation::persist_support_mutation;
use players::domain::space_mutation::SpaceMutation;
use players::domain::space_mutation_types::SpaceMutationTypes;

pub fn persist_space_mutations(
    space_mutations: &[SpaceMutation],
    board: &mut Board,
) -> Result<(), String> {
    for mutation in space_mutations.iter() {
        match mutation.get_space_mutation_type() {
            SpaceMutationTypes::Terror => board
                .get_space_mut(*mutation.get_space_identifier())?
                .set_terror()?,
            SpaceMutationTypes::Support => {
                persist_support_mutation(&mutation, board)?;
            }
            _ => panic!(
                "Persisting space mutation not implemented for space mutation type {:?}",
                mutation.get_space_mutation_type()
            ),
        }
    }

    Ok(())
}
