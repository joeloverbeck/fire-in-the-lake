use game_definitions::factions::Factions;
use players::domain::sequence_of_play_mutation::SequenceOfPlayMutation;
use sequence_of_play::controllers::sequence_of_play_controller::SequenceOfPlayController;

pub fn persist_sequence_of_play_mutations(
    sequence_of_play_mutations: &[SequenceOfPlayMutation],
    faction_order: [Factions; 4],
    sequence_of_play_controller: &mut SequenceOfPlayController,
) -> Result<(), String> {
    for mutation in sequence_of_play_mutations {
        sequence_of_play_controller.register_pick(
            mutation.get_faction(),
            faction_order,
            mutation.get_sequence_of_play_slot(),
        )?;
    }

    Ok(())
}
