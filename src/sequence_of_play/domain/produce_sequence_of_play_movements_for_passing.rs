use sequence_of_play::domain::was_executing_faction_one_of_main_eligible::was_executing_faction_one_of_main_eligible;
use sequence_of_play::domain::produce_mutation_for_moving_next_in_faction_order_to_second_eligible::produce_mutation_for_moving_next_in_faction_order_to_second_eligible;
use sequence_of_play::domain::movement_mutation::MovementMutation;
use sequence_of_play::controllers::sequence_of_play_controller::SequenceOfPlayController;
use game_definitions::factions::Factions;
use sequence_of_play::domain::movements::Movements;

pub fn produce_sequence_of_play_movements_for_passing(
    faction: &Factions,
    faction_order: [Factions; 4],
    sequence_of_play_controller: &SequenceOfPlayController,
) -> Result<Vec<MovementMutation>, String> {
    // If a First or Second Eligible faction (only) opts to Pass, it remains eligible for the next card and receives +1 Resource if an Insurgent Faction and adds
    // +3 ARVN Resources if either COIN faction.
    let mut movement_mutations: Vec<MovementMutation> = Vec::new();

    // First we have to make sure that the faction we are going to put into Pass has been one of the main eligible.
    if !was_executing_faction_one_of_main_eligible(faction, sequence_of_play_controller)? {
        panic!("Was going to mark the faction {:?} as having passed, but it wasn't either of the main elegibles! {:?} and {:?}", faction, sequence_of_play_controller.get_first_eligible(), sequence_of_play_controller.get_second_eligible());
    }

    // The faction we received is indeed one that can pass.
    movement_mutations.push(MovementMutation::new(Some(*faction), Movements::Passed));

    // Now we have to remove it from whatever main eligible positions it was in, and move the remainder in the faction order to the left.
    if sequence_of_play_controller.is_there_a_first_eligible_faction()?
        && sequence_of_play_controller.get_first_eligible()? == *faction
    {
        // It was the first eligible. Now the second eligible becomes the first one, and the next in the faction order becomes the second.
        // However, we only do this if there IS a second eligible. If there isn't, then the turn has ended, but we need to register that the first eligible becomes empty.
        if !sequence_of_play_controller.is_there_a_second_eligible_faction()? {
            // Note that there should be no first eligible then, because the first one has passed and there was no second one.
            movement_mutations.push(MovementMutation::new(None, Movements::FirstEligible));

            return Ok(movement_mutations);
        }

        movement_mutations.push(MovementMutation::new(
            Some(sequence_of_play_controller.get_second_eligible()?),
            Movements::FirstEligible,
        ));

        // Now we have to look for the next one in the faction order *that remains eligible* and put it as the second eligible.
        movement_mutations.push(
            produce_mutation_for_moving_next_in_faction_order_to_second_eligible(
                faction_order,
                sequence_of_play_controller,
            )?,
        );
    } else if sequence_of_play_controller.get_second_eligible()? == *faction {
        // It was the second eligible. Now the next in the faction order becomes the second eligible.
        movement_mutations.push(
            produce_mutation_for_moving_next_in_faction_order_to_second_eligible(
                faction_order,
                sequence_of_play_controller,
            )?,
        );
    } else {
        todo!()
    }

    Ok(movement_mutations)
}
