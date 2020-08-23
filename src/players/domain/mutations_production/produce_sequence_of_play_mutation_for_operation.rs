use game_definitions::factions::Factions;
use players::domain::sequence_of_play_mutation::SequenceOfPlayMutation;
use sequence_of_play::domain::sequence_of_play_slots::SequenceOfPlaySlots;
use sequence_of_play::domain::slot_occupancy::SlotOccupancy;

pub fn produce_sequence_of_play_mutation_for_operation(
    faction: Factions,
    possible_actions: &[SequenceOfPlaySlots],
    sequence_of_play_mutations: &mut Vec<SequenceOfPlayMutation>,
) -> Result<(), String> {
    let chosen_sequence_of_play_slot: SequenceOfPlaySlots = {
        // First check the obvious for the second eligible.
        if possible_actions.contains(&SequenceOfPlaySlots::SecondFactionLimitedOperation) {
            SequenceOfPlaySlots::SecondFactionLimitedOperation
        } else if possible_actions
            .contains(&SequenceOfPlaySlots::SecondFactionLimitedOperationOrEvent)
        {
            SequenceOfPlaySlots::SecondFactionLimitedOperationOrEvent
        } else if possible_actions
            .contains(&SequenceOfPlaySlots::SecondFactionOperationPlusSpecialActivity)
        {
            SequenceOfPlaySlots::SecondFactionOperationPlusSpecialActivity
        } else if possible_actions.contains(&SequenceOfPlaySlots::FirstFactionOperationOnly) {
            SequenceOfPlaySlots::FirstFactionOperationOnly
        } else if possible_actions
            .contains(&SequenceOfPlaySlots::FirstFactionOperationPlusSpecialActivity)
        {
            SequenceOfPlaySlots::FirstFactionOperationPlusSpecialActivity
        } else {
            panic!("Attempted to produce a sequence of play mutation for an operation (for the AI, probably), but couldn't deal with the following: {:?}", possible_actions);
        }
    };

    sequence_of_play_mutations.push(SequenceOfPlayMutation::new(
        chosen_sequence_of_play_slot,
        SlotOccupancy::Occupied,
        faction,
    ));

    Ok(())
}
