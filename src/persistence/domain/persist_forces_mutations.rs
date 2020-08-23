use board::domain::board::Board;
use players::domain::forces_mutation::ForcesMutation;
use players::domain::mutation_types::MutationTypes;

pub fn persist_forces_mutations(
    forces_mutations: &[ForcesMutation],
    board: &mut Board,
) -> Result<(), String> {
    for mutation in forces_mutations {
        if mutation.get_mutation_type() == &MutationTypes::Move {
            board.reduce_forces_in_space(
                mutation.get_forces(),
                mutation.get_from().unwrap(),
                mutation.get_number(),
            )?;
            board.increase_forces_in_space(
                mutation.get_forces(),
                mutation.get_to().unwrap(),
                mutation.get_number(),
            )?;
        } else {
            panic!(
                "Case not handled for persist forces mutations type {:?}. Mutation: {:?}",
                mutation.get_mutation_type(),
                mutation
            );
        }
    }

    Ok(())
}
