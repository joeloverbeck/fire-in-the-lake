use board::domain::board::Board;
use game_definitions::forces::Forces;
use players::domain::forces_mutation::ForcesMutation;
use players::domain::mutation_types::MutationTypes;

pub fn persist_forces_mutations(
    forces_mutations: &[ForcesMutation],
    board: &mut Board,
) -> Result<(), String> {
    for mutation in forces_mutations {
        match mutation.get_mutation_type() {
            MutationTypes::Reduce | MutationTypes::Increase => panic!("Attempted to push through a mutation with either a reduce or increase forces. You should use Move, because they always get moved from a place to another (even if that other place is the casualties, etc.)"),
            MutationTypes::Move => {
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
            }
            MutationTypes::Flip => {
                if mutation.get_number() > 1 {
                    panic!("Case not handled for flipping more than one piece!");
                }

                board.reduce_forces_in_space(
                    mutation.get_forces(),
                    mutation.get_from().unwrap(),
                    1,
                )?;

                // There is no direct translation from the unflipped piece to the flipped one.
                match mutation.get_forces() {
                    Forces::UndergroundNvaGuerrilla => board.increase_forces_in_space(
                        &Forces::ActiveNvaGuerrilla,
                        mutation.get_to().unwrap(),
                        1,
                    )?,
                    _ => panic!("Case not handled for flipping {:?}", mutation.get_forces()),
                }
            }
            _ => panic!(
                "Case not handled for persist forces mutations type {:?}. Mutation: {:?}",
                mutation.get_mutation_type(),
                mutation
            ),
        }
    }

    Ok(())
}
