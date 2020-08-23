use board::domain::board::Board;
use players::domain::faction_stats_mutation::FactionStatsMutation;
use players::domain::mutation_types::MutationTypes;

pub fn persist_faction_stats_mutations(
    faction_stats_mutations: &[FactionStatsMutation],
    board: &mut Board,
) -> Result<(), String> {
    for mutation in faction_stats_mutations {
        if mutation.get_mutation_type() == &MutationTypes::Increase {
            board.increase_faction_stat(mutation.get_faction_stat(), mutation.get_value())?;
        } else if mutation.get_mutation_type() == &MutationTypes::Reduce {
            board.reduce_faction_stat(mutation.get_faction_stat(), mutation.get_value())?;
        } else {
            panic!("Case not handled for faction stats mutation {:?}", mutation);
        }
    }

    Ok(())
}
