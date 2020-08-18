use game_definitions::factions::Factions;
use sequence_of_play::controllers::sequence_of_play_controller::SequenceOfPlayController;
use sequence_of_play::domain::movement_mutation::MovementMutation;
use sequence_of_play::domain::movements::Movements;

pub fn produce_mutation_for_moving_next_in_faction_order_to_second_eligible(
    faction_order: [Factions; 4],
    sequence_of_play_controller: &SequenceOfPlayController,
) -> Result<MovementMutation, String> {
    for index in 0..3 {
        if faction_order[index] == sequence_of_play_controller.get_second_eligible()? {
            // Next one should go to second eligible, *unless it was uneligible*
            let mut index_of_replacement = index + 1;

            while index_of_replacement <= 3 {
                if !sequence_of_play_controller
                    .is_faction_ineligible(&faction_order[index_of_replacement])?
                {
                    break;
                }

                index_of_replacement += 1;
            }

            return Ok(MovementMutation::new(
                Some(faction_order[index_of_replacement]),
                Movements::SecondEligible,
            ));
        }
    }

    Err("Failed to find a next eligible faction for the second eligible!".to_string())
}
