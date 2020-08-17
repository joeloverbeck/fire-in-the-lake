use game_definitions::factions::Factions;
use sequence_of_play::controllers::sequence_of_play_controller::SequenceOfPlayController;
use sequence_of_play::domain::movement_mutation::MovementMutation;
use sequence_of_play::domain::movements::Movements;
use sequence_of_play::domain::sequence_of_play_slots::SequenceOfPlaySlots;

pub fn produce_sequence_of_play_movements(
    faction: &Factions,
    faction_order: [Factions; 4],
    slot: &SequenceOfPlaySlots,
    sequence_of_play_controller: &SequenceOfPlayController,
) -> Result<Vec<MovementMutation>, String> {
    let mut movement_mutations: Vec<MovementMutation> = Vec::new();

    if slot == &SequenceOfPlaySlots::FirstFactionEvent {
        // The faction was the first elegible, and played for the event.
        if sequence_of_play_controller.is_first_faction_event_taken()? {
            panic!("Had attempted to register {:?} as having chosen to play the event being the first elegible faction, but there was a faction already in that position!: {:?}", faction, sequence_of_play_controller.get_faction_in_first_faction_event()?);
        }

        movement_mutations.push(MovementMutation::new(
            *faction,
            Movements::FirstFactionEvent,
        ));
    } else if slot == &SequenceOfPlaySlots::Pass {
        // If a First or Second Eligible faction (only) opts to Pass, it remains eligible for the next card and receives +1 Resource if an Insurgent Faction and adds
        // +3 ARVN Resources if either COIN faction.

        // First we have to make sure that the faction we are going to put into Pass has been one of the main eligible.
        if sequence_of_play_controller.get_first_eligible()? == *faction
            || sequence_of_play_controller.get_second_eligible()? == *faction
        {
            // The faction we received is indeed one that can pass.
            movement_mutations.push(MovementMutation::new(*faction, Movements::Passed));

            // Now we have to remove it from whatever main eligible positions it was in, and move the remainder in the faction order to the left.
            if sequence_of_play_controller.get_first_eligible()? == *faction {
                // It was the first eligible. Now the second eligible becomes the first one, and the next in the faction order becomes the second.
                movement_mutations.push(MovementMutation::new(
                    sequence_of_play_controller.get_second_eligible()?,
                    Movements::FirstEligible,
                ));

                // Now we have to look for the next one in the faction order *that remains eligible* and put it as the second eligible.
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

                        movement_mutations.push(MovementMutation::new(
                            faction_order[index_of_replacement],
                            Movements::SecondEligible,
                        ));
                        break;
                    }
                }
            } else {
                todo!()
            }
        } else {
            panic!("Was going to mark the faction {:?} as having passed, but  it wasn't either of the main elegibles! {:?} and {:?}", faction, sequence_of_play_controller.get_first_eligible(), sequence_of_play_controller.get_second_eligible());
        }
    } else {
        panic!("Registering a pick in the sequence of play wasn't handled for the following: {:?} and {:?}", faction, slot);
    }

    Ok(movement_mutations)
}
