use game_definitions::factions::Factions;
use sequence_of_play::controllers::sequence_of_play_controller::SequenceOfPlayController;
use sequence_of_play::domain::movement_mutation::MovementMutation;
use sequence_of_play::domain::movements::Movements;
use sequence_of_play::domain::sequence_of_play_slots::SequenceOfPlaySlots;
use sequence_of_play::domain::was_executing_faction_one_of_main_eligible::was_executing_faction_one_of_main_eligible;

pub fn produce_sequence_of_play_movements_for_one_of_the_main_first_faction_actions(
    faction: &Factions,
    pick: &SequenceOfPlaySlots,
    sequence_of_play_controller: &SequenceOfPlayController,
) -> Result<Vec<MovementMutation>, String> {
    // Sanity check
    if !was_executing_faction_one_of_main_eligible(faction, sequence_of_play_controller)? {
        panic!("Was going to mark the faction {:?} as having chosen {:?}, but it wasn't either of the main elegibles! {:?} and {:?}", faction, pick, sequence_of_play_controller.get_first_eligible(), sequence_of_play_controller.get_second_eligible());
    }

    let mut movement_mutations: Vec<MovementMutation> = Vec::new();

    let movement_to_push = {
        match pick {
            SequenceOfPlaySlots::FirstFactionOperationOnly => Movements::FirstFactionOperationOnly,
            SequenceOfPlaySlots::FirstFactionOperationPlusSpecialActivity => {
                Movements::FirstFactionOperationPlusSpecialActivity
            }
            SequenceOfPlaySlots::FirstFactionEvent => Movements::FirstFactionEvent,
            _ => panic!("Case not handled for pick {:?}", pick),
        }
    };

    movement_mutations.push(MovementMutation::new(Some(*faction), movement_to_push));

    // We need to produce the movements to remove the executing faction from whatever eligible position it was in.
    if sequence_of_play_controller.is_there_a_first_eligible_faction()?
        && sequence_of_play_controller.get_first_eligible()? == *faction
    {
        movement_mutations.push(MovementMutation::new(None, Movements::FirstEligible));
    } else if sequence_of_play_controller.get_second_eligible()? == *faction {
        panic!("Was going to produce a mutation to register executing function as having to move to {:?}, but it was the second eligible faction! That ain't right.", movement_to_push);
    }

    Ok(movement_mutations)
}
