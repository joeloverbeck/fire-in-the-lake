use game_definitions::factions::Factions;
use sequence_of_play::controllers::sequence_of_play_controller::SequenceOfPlayController;
use sequence_of_play::domain::movement_mutation::MovementMutation;
use sequence_of_play::domain::movements::Movements;
use sequence_of_play::domain::produce_sequence_of_play_movements_for_passing::produce_sequence_of_play_movements_for_passing;
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
        movement_mutations.append(&mut produce_sequence_of_play_movements_for_passing(
            faction,
            faction_order,
            sequence_of_play_controller,
        )?);
    } else {
        panic!("Registering a pick in the sequence of play wasn't handled for the following: {:?} and {:?}", faction, slot);
    }

    Ok(movement_mutations)
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_when_the_first_faction_passes_the_next_first_eligible_should_be_the_previous_first_one(
    ) -> Result<(), String> {
        let mut sequence_of_play_controller = SequenceOfPlayController::new();

        let faction_order = [Factions::NVA, Factions::ARVN, Factions::VC, Factions::US];

        sequence_of_play_controller.register_faction_order(faction_order)?;

        let mutations = produce_sequence_of_play_movements(
            &Factions::NVA,
            faction_order,
            &SequenceOfPlaySlots::Pass,
            &sequence_of_play_controller,
        )?;

        assert_eq!(*mutations[0].get_faction(), Factions::NVA);
        assert_eq!(*mutations[0].get_movement(), Movements::Passed);

        assert_eq!(*mutations[1].get_faction(), Factions::ARVN);
        assert_eq!(*mutations[1].get_movement(), Movements::FirstEligible);

        assert_eq!(*mutations[2].get_faction(), Factions::VC);
        assert_eq!(*mutations[2].get_movement(), Movements::SecondEligible);

        Ok(())
    }

    #[test]
    fn test_when_the_second_faction_passes_the_next_first_eligible_should_be_the_previous_first_one(
    ) -> Result<(), String> {
        let mut sequence_of_play_controller = SequenceOfPlayController::new();

        let faction_order = [Factions::NVA, Factions::ARVN, Factions::VC, Factions::US];

        sequence_of_play_controller.register_faction_order(faction_order)?;

        let mutations = produce_sequence_of_play_movements(
            &Factions::ARVN,
            faction_order,
            &SequenceOfPlaySlots::Pass,
            &sequence_of_play_controller,
        )?;

        println!("{:?}", mutations);

        assert_eq!(
            *mutations[0].get_faction(),
            Factions::ARVN,
            "There should be a mutation for ARVN regarding having been marked as Passed."
        );
        assert_eq!(
            *mutations[0].get_movement(),
            Movements::Passed,
            "ARVN should have been marked as Passed."
        );

        assert_eq!(
            *mutations[1].get_faction(),
            Factions::VC,
            "VC should have a mutation regarding having been marked as Second Eligible."
        );
        assert_eq!(
            *mutations[1].get_movement(),
            Movements::SecondEligible,
            "VC should have been marked as Second Eligible."
        );

        Ok(())
    }
}
