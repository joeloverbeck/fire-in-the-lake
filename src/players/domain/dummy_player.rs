use board::domain::board::Board;
use cards::domain::card::Cards;
use game_definitions::factions::Factions;
use players::domain::decision::Decision;
use players::domain::faction_stats_mutation::FactionStatsMutation;
use players::domain::passing::produce_faction_stats_mutations_for_passing::produce_faction_stats_mutations_for_passing;
use players::domain::player::Player;
use players::domain::player_type::PlayerType;
use players::domain::sequence_of_play_mutation::SequenceOfPlayMutation;
use randomization::controllers::randomization_controller::RandomizationController;
use sequence_of_play::domain::sequence_of_play_slots::SequenceOfPlaySlots;
use sequence_of_play::domain::slot_occupancy::SlotOccupancy;
use user_interface::controllers::display_controller::DisplayController;
use user_interface::controllers::keyboard_input_controller::KeyboardInputController;

#[derive(Debug)]
pub struct DummyPlayer {}

impl Default for DummyPlayer {
    fn default() -> Self {
        Self::new()
    }
}

impl Player for DummyPlayer {
    fn decide(
        &self,
        _active_card: &Cards,
        _preview_card: &Cards,
        current_elegible_faction: Factions,
        _possible_actions: Vec<String>,
        board: &Board,
        _randomization_controller: &RandomizationController,
        _keyboard_input_controller: &KeyboardInputController,
        _display_controller: &DisplayController,
    ) -> Result<Decision, String> {
        // This dummy player just passes for every possible decision.

        let mut sequence_of_play_mutations: Vec<SequenceOfPlayMutation> = Vec::new();
        sequence_of_play_mutations.push(SequenceOfPlayMutation::new(
            SequenceOfPlaySlots::Pass,
            SlotOccupancy::Occupied,
            current_elegible_faction,
        ));

        let mut faction_stats_mutations: Vec<FactionStatsMutation> = Vec::new();

        faction_stats_mutations.append(&mut produce_faction_stats_mutations_for_passing(
            &current_elegible_faction,
            &board,
        )?);

        Ok(Decision::new(
            sequence_of_play_mutations,
            faction_stats_mutations,
            Vec::new(),
            Vec::new(),
        ))
    }

    fn get_player_type(&self) -> Result<PlayerType, String> {
        Ok(PlayerType::Ai)
    }
}

impl DummyPlayer {
    pub fn new() -> DummyPlayer {
        DummyPlayer {}
    }
}
