use board::domain::board::Board;
use game_definitions::factions::Factions;
use players::domain::decision::Decision;
use players::domain::player::Player;
use players::domain::sequence_of_play_mutation::SequenceOfPlayMutation;
use sequence_of_play::domain::sequence_of_play_slots::SequenceOfPlaySlots;
use sequence_of_play::domain::slot_occupancy::SlotOccupancy;
use user_interface::controllers::user_interface_controller::UserInterfaceController;

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
        _active_card: u8,
        _preview_card: u8,
        current_elegible_faction: Factions,
        _possible_actions: Vec<String>,
        _board: &Board,
        _user_interface_controller: &UserInterfaceController,
    ) -> Result<Decision, String> {
        // This dummy player just passes for every possible decision.

        let mut sequence_of_play_mutations: Vec<SequenceOfPlayMutation> = Vec::new();
        sequence_of_play_mutations.push(SequenceOfPlayMutation::new(
            SequenceOfPlaySlots::Pass,
            SlotOccupancy::Occupied,
            current_elegible_faction,
        ));

        Ok(Decision::new(
            sequence_of_play_mutations,
            Vec::new(),
            Vec::new(),
            Vec::new(),
        ))
    }
}

impl DummyPlayer {
    pub fn new() -> DummyPlayer {
        DummyPlayer {}
    }
}
