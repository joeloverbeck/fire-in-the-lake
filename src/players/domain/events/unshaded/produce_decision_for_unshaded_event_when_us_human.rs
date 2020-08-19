use board::domain::board::Board;
use cards::domain::card::Card;
use cards::domain::card::Cards;
use game_definitions::faction_stats::FactionStats;
use game_definitions::factions::Factions;
use game_definitions::flags::Flags;
use game_definitions::forces::Forces;
use game_definitions::space_identifiers::SpaceIdentifiers;
use players::domain::decision::Decision;
use players::domain::events::request_forces_movement_from_human::request_forces_movement_from_human;
use players::domain::faction_stats_mutation::FactionStatsMutation;
use players::domain::flags_mutation::FlagsMutation;
use players::domain::mutation_types::MutationTypes;
use players::domain::sequence_of_play_mutation::SequenceOfPlayMutation;
use sequence_of_play::domain::sequence_of_play_slots::SequenceOfPlaySlots;
use sequence_of_play::domain::slot_occupancy::SlotOccupancy;
use user_interface::controllers::display_controller::DisplayController;
use user_interface::controllers::keyboard_input_controller::KeyboardInputController;

pub fn produce_decision_for_unshaded_event_when_us_human(
    active_card: &Cards,
    board: &Board,
    keyboard_input_controller: &KeyboardInputController,
    display_controller: &DisplayController,
) -> Result<Decision, String> {
    let mut sequence_of_play_mutations: Vec<SequenceOfPlayMutation> = Vec::new();
    sequence_of_play_mutations.push(SequenceOfPlayMutation::new(
        SequenceOfPlaySlots::FirstFactionEvent,
        SlotOccupancy::Occupied,
        Factions::US,
    ));

    if active_card.get_number()? == 16 {
        // Aid +10. This Support phase, Pacify costs 1 Resource per step or Terror. MOMENTUM.
        let mut faction_stat_mutations: Vec<FactionStatsMutation> = Vec::new();
        faction_stat_mutations.push(FactionStatsMutation::new(
            FactionStats::Aid,
            MutationTypes::Increase,
            board.get_faction_stat(FactionStats::Aid)?,
            10,
        ));

        let mut flag_mutations: Vec<FlagsMutation> = Vec::new();
        flag_mutations.push(FlagsMutation::new(Flags::BlowtorchComer, true));

        Ok(Decision::new(
            sequence_of_play_mutations,
            faction_stat_mutations,
            Vec::new(),
            flag_mutations,
        ))
    } else if active_card.get_number()? == 22 {
        // US places up to 6 Troops in Da Nang, up to 3 from out of play.

        // Unfortunately in this case we require user input
        Ok(request_forces_movement_from_human(
            Forces::UsTroop,
            SpaceIdentifiers::DaNang,
            6,
            Some((SpaceIdentifiers::OutOfPlay, 3)),
            board,
            keyboard_input_controller,
            display_controller,
        )?)
    } else {
        panic!("Not contemplated for {:?}", active_card);
    }
}
