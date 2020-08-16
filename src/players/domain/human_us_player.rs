use board::domain::board::Board;
use game_definitions::faction_stats::FactionStats;
use players::domain::decision::Decision;
use players::domain::faction_stats_mutation::FactionStatsMutation;
use players::domain::mutation_types::MutationTypes;
use players::domain::sequence_of_play_mutation::SequenceOfPlayMutation;
use sequence_of_play::domain::sequence_of_play_slots::SequenceOfPlaySlots;
use sequence_of_play::domain::slot_occupancy::SlotOccupancy;
use user_interface::controllers::user_interface_controller::UserInterfaceController;

pub struct HumanUsPlayer {}

impl Default for HumanUsPlayer {
    fn default() -> Self {
        Self::new()
    }
}

impl HumanUsPlayer {
    pub fn new() -> HumanUsPlayer {
        HumanUsPlayer {}
    }

    pub fn decide(
        &self,
        active_card: u8,
        _preview_card: u8,
        possible_actions: Vec<String>,
        _board: &Board,
        user_interface_controller: &UserInterfaceController,
    ) -> Result<Decision, String> {
        let mut possible_actions_text = "[".to_string();

        for entry in possible_actions {
            possible_actions_text += &entry;
        }

        possible_actions_text += &"]".to_string();

        let input = user_interface_controller.request_player_input(
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
            if active_card == 16 {
                // Aid +10. This Support phase, Pacify costs 1 Resource per step or Terror. MOMENTUM.
                let mut faction_stat_mutations: Vec<FactionStatsMutation> = Vec::new();
                faction_stat_mutations.push(FactionStatsMutation::new(
                    FactionStats::Aid,
                    MutationTypes::Reduce,
                    10,
                ));

                let mut sequence_of_play_mutations: Vec<SequenceOfPlayMutation> = Vec::new();
                sequence_of_play_mutations.push(SequenceOfPlayMutation::new(
                    SequenceOfPlaySlots::FirstFactionEvent,
                    SlotOccupancy::Occupied,
                ));

                Ok(Decision::new(
                    sequence_of_play_mutations,
                    faction_stat_mutations,
                ))
            } else {
                panic!("Not contemplated for {:?}", active_card);
            }
        } else {
            todo!()
        }
    }
}
