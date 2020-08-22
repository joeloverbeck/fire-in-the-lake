use players::domain::faction_stats_mutations_production::produce_faction_stats_mutations_for_passing::produce_faction_stats_mutations_for_passing;
use board::domain::board::Board;
use game_definitions::factions::Factions;
use players::domain::decision::Decision;
use players::domain::faction_stats_mutation::FactionStatsMutation;
use players::domain::sequence_of_play_mutation::SequenceOfPlayMutation;
use sequence_of_play::domain::sequence_of_play_slots::SequenceOfPlaySlots;
use sequence_of_play::domain::slot_occupancy::SlotOccupancy;

pub fn produce_decision_to_pass(faction: Factions, board: &Board) -> Result<Decision, String> {
    let mut sequence_of_play_mutations: Vec<SequenceOfPlayMutation> = Vec::new();
    sequence_of_play_mutations.push(SequenceOfPlayMutation::new(
        SequenceOfPlaySlots::Pass,
        SlotOccupancy::Occupied,
        faction,
    ));

    let mut faction_stats_mutations: Vec<FactionStatsMutation> = Vec::new();

    faction_stats_mutations.append(&mut produce_faction_stats_mutations_for_passing(
        &faction, &board,
    )?);

    Ok(Decision::new(
        sequence_of_play_mutations,
        faction_stats_mutations,
        Vec::new(),
        Vec::new(),
    ))
}
