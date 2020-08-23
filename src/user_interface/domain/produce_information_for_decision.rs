use game_definitions::factions::Factions;
use players::domain::decision::Decision;
use sequence_of_play::domain::sequence_of_play_slots::SequenceOfPlaySlots;

pub fn produce_information_for_decision(
    decision: &Decision,
    faction: &Factions,
) -> Result<Vec<String>, String> {
    // Depending on the slot it has chosen to occupy, we can determine what it
    // Intended to do.
    if decision
        .get_mutations()?
        .get_sequence_of_play_mutations()?
        .len()
        > 1
    {
        panic!("Was going to produce information for a decision, but couldn't handle the case that the sequence of play mutations would be more than one.");
    }

    let sequence_of_play_mutation = &decision.get_mutations()?.get_sequence_of_play_mutations()?[0];

    let mut information: Vec<String> = Vec::new();

    let sequence_of_play_slot = sequence_of_play_mutation.get_sequence_of_play_slot();

    match sequence_of_play_slot {
        SequenceOfPlaySlots::FirstFactionEvent => {
            information.push(format!("{} chose to play the card for the event", faction));
        }
        SequenceOfPlaySlots::Pass => {
            information.push(format!("{} chose to pass", faction));
        }
        _ => panic!("Not implemented for {:?}", sequence_of_play_slot),
    }

    Ok(information)
}
