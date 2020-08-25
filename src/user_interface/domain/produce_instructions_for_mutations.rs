use game_definitions::factions::Factions;
use game_definitions::flags::Flags;
use players::domain::decision::Decision;
use players::domain::mutation_types::MutationTypes;
use sequence_of_play::domain::sequence_of_play_slots::SequenceOfPlaySlots;
use sequence_of_play::domain::slot_occupancy::SlotOccupancy;

pub fn produce_instructions_for_mutations(
    decision: &Decision,
    faction: &Factions,
) -> Result<Vec<String>, String> {
    let mut instructions: Vec<String> = Vec::new();

    // First create the instructions for moving the pieces around in the Sequence of Play area.
    for mutation in decision
        .get_mutations()?
        .get_sequence_of_play_mutations()?
        .iter()
    {
        if mutation.get_sequence_of_play_slot() == &SequenceOfPlaySlots::FirstFactionEvent
            && mutation.get_slot_occupancy() == &SlotOccupancy::Occupied
        {
            // Player needs to move that faction's cylinder to First Faction Event
            instructions.push(format!(
                "Move {} cylinder from Elegible to First Faction Event.",
                faction
            ));
        } else if mutation.get_sequence_of_play_slot() == &SequenceOfPlaySlots::Pass
            && mutation.get_slot_occupancy() == &SlotOccupancy::Occupied
        {
            instructions.push(format!("Move {} cylinder from Elegible to Pass.", faction));
        } else if mutation.get_sequence_of_play_slot()
            == &SequenceOfPlaySlots::FirstFactionOperationOnly
        {
            instructions.push(format!(
                "Move {} cylinder from Elegible to First Faction Operation Only.",
                faction
            ));
        } else {
            panic!("The instruction for moving pieces in sequence of play area not implemented for {:?}", mutation.get_sequence_of_play_slot());
        }
    }

    // Second handle the mutations to the faction stats
    for mutation in decision
        .get_mutations()?
        .get_faction_stats_mutations()?
        .iter()
    {
        if mutation.get_mutation_type() == &MutationTypes::Reduce {
            // Wants to reduce a faction stat.
            instructions.push(format!(
                "Reduce {} from {} to {}",
                mutation.get_faction_stat(),
                mutation.get_previous_value(),
                mutation.get_previous_value() - mutation.get_value()
            ));
        } else if mutation.get_mutation_type() == &MutationTypes::Increase {
            // Wants to increase a faction stat.
            instructions.push(format!(
                "Increase {} from {} to {}",
                mutation.get_faction_stat(),
                mutation.get_previous_value(),
                mutation.get_previous_value() + mutation.get_value()
            ));
        } else {
            panic!(
                "Case not handled for faction stats mutation type {:?}",
                mutation.get_mutation_type()
            );
        }
    }

    // Handle the mutations to the forces.
    if decision.get_mutations()?.has_forces_mutations()? {
        for mutation in decision.get_mutations()?.get_forces_mutations()?.iter() {
            if mutation.get_mutation_type() == &MutationTypes::Move {
                // Sanity check
                if mutation.get_from().is_none() || mutation.get_to().is_none() {
                    return Err(format!("While attempting to produce the instruction for an order to move troops from a place to another, either the origin or the destination hadn't been set! Mutation: {:?}", mutation));
                }

                instructions.push(format!(
                    "Move {} {} from {} to {}.",
                    mutation.get_number(),
                    mutation.get_forces(),
                    mutation.get_from().unwrap(),
                    mutation.get_to().unwrap()
                ));
            } else {
                panic!(
                    "Case not implemented for mutation type {:?}",
                    mutation.get_mutation_type()
                );
            }
        }
    }

    // Handle space mutations
    if decision.get_mutations()?.has_space_mutations()? {
        for mutation in decision.get_mutations()?.get_space_mutations()?.iter() {
            panic!(
                "Writing instructions not implemented for space mutation {:?}",
                mutation.get_space_mutation_type()
            );
        }
    }

    // Handle the flag mutations
    if decision.get_mutations()?.has_flags_mutations()? {
        for mutation in decision.get_mutations()?.get_flags_mutations()?.iter() {
            if mutation.get_flag() == &Flags::BlowtorchComer
                && mutation.get_mutation_type() == &MutationTypes::Set
            {
                // 'Blowtorch Komer' will be active throughout the next Coup Round.
                instructions.push("'Blowtorch Komer' is now active through the next Coup Round: 'This Support Phase, Pacify costs 1 Resource per step or Terror.'".to_string());
            } else {
                panic!("Case not handled for mutation: {:?}", mutation);
            }
        }
    }

    Ok(instructions)
}
