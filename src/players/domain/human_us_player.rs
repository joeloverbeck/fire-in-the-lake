use board::domain::board::Board;
use game_definitions::faction_stats::FactionStats;
use game_definitions::factions::Factions;
use players::domain::decision::Decision;
use players::domain::events::unshaded::produce_decision_for_unshaded_event_when_us_human::produce_decision_for_unshaded_event_when_us_human;
use players::domain::faction_stats_mutation::FactionStatsMutation;
use players::domain::mutation_types::MutationTypes;
use players::domain::player::Player;
use players::domain::sequence_of_play_mutation::SequenceOfPlayMutation;
use sequence_of_play::domain::sequence_of_play_slots::SequenceOfPlaySlots;
use sequence_of_play::domain::slot_occupancy::SlotOccupancy;
use user_interface::controllers::display_controller::DisplayController;
use user_interface::controllers::keyboard_input_controller::KeyboardInputController;

#[derive(Debug)]
pub struct HumanUsPlayer {}

impl Default for HumanUsPlayer {
    fn default() -> Self {
        Self::new()
    }
}

impl Player for HumanUsPlayer {
    fn decide(
        &self,
        active_card: u8,
        _preview_card: u8,
        _current_elegible_faction: Factions,
        possible_actions: Vec<String>,
        board: &Board,
        keyboard_input_controller: &KeyboardInputController,
        display_controller: &DisplayController,
    ) -> Result<Decision, String> {
        let mut possible_actions_text = "[".to_string();

        for entry in possible_actions {
            possible_actions_text += &entry;
        }

        possible_actions_text += &"]".to_string();

        let input = keyboard_input_controller.request_player_input(
            format!(
                "What action do you want to take? {}: ",
                possible_actions_text
            )
            .as_str(),
        )?;

        if input == "event" {
            // Many, many things can vary here. You first neet to take into account which card we are dealing with.
            // However, the cards controller cannot help us here, as it would be unreasonable
            // to codify any card event mechanics there. So there should be some general functions that deal with what
            // must be applied, or could even be chosen, regarding every event.
            Ok(produce_decision_for_unshaded_event_when_us_human(
                active_card,
                board,
                keyboard_input_controller,
                display_controller,
            )?)
        } else if input == "pass" {
            // ARVN gains 3 resources.
            let mut sequence_of_play_mutations: Vec<SequenceOfPlayMutation> = Vec::new();
            sequence_of_play_mutations.push(SequenceOfPlayMutation::new(
                SequenceOfPlaySlots::Pass,
                SlotOccupancy::Occupied,
                Factions::US,
            ));

            let mut faction_stats_mutations: Vec<FactionStatsMutation> = Vec::new();
            faction_stats_mutations.push(FactionStatsMutation::new(
                FactionStats::ArvnResources,
                MutationTypes::Increase,
                board.get_faction_stat(FactionStats::ArvnResources)?,
                3,
            ));

            Ok(Decision::new(
                sequence_of_play_mutations,
                faction_stats_mutations,
                Vec::new(),
                Vec::new(),
            ))
        } else {
            todo!()
        }
    }
}

impl HumanUsPlayer {
    pub fn new() -> HumanUsPlayer {
        HumanUsPlayer {}
    }
}
