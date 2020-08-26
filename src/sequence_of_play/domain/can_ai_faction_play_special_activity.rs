use sequence_of_play::domain::sequence_of_play_slots::SequenceOfPlaySlots;

pub fn can_ai_faction_play_special_activity(
    possible_actions: &[SequenceOfPlaySlots],
) -> Result<bool, String> {
    Ok(possible_actions.iter().any(|possible_action| {
        possible_action == &SequenceOfPlaySlots::FirstFactionOperationPlusSpecialActivity
    }) || possible_actions.iter().any(|possible_action| {
        possible_action == &SequenceOfPlaySlots::SecondFactionLimitedOperation
    }) || possible_actions.iter().any(|possible_action| {
        possible_action == &SequenceOfPlaySlots::SecondFactionLimitedOperationOrEvent
    }) || possible_actions.iter().any(|possible_action| {
        possible_action == &SequenceOfPlaySlots::SecondFactionOperationPlusSpecialActivity
    }))
}
