use board::domain::space::Space;
use board::domain::space::Spaces;
use game_definitions::forces::Forces;
use game_definitions::space_identifiers::SpaceIdentifiers;
use players::domain::forces_mutation::ForcesMutation;
use players::domain::mutation_types::MutationTypes;

pub fn produce_mutations_for_placing_nva_base_through_rally(
    forces_mutations: &mut Vec<ForcesMutation>,
    target: &Spaces,
) -> Result<(), String> {
    // Place a NVA Base (replacing 2 NVA Guerrillas, removing Active Guerrillas first), wherever there are at last 4 NVA or VC Guerrillas or Troops (including 2 NVA Guerrillas and room for the base).

    // It should remove Active Guerrillas first.
    if target.get_forces(Forces::ActiveNvaGuerrilla)? >= 2 {
        // Should remove two ActiveNvaGuerrillas.
        forces_mutations.push(ForcesMutation::new(
            Forces::ActiveNvaGuerrilla,
            MutationTypes::Move,
            2,
            Some(target.get_space_identifier()?),
            Some(SpaceIdentifiers::Available),
        ));
    } else if target.get_forces(Forces::ActiveNvaGuerrilla)? == 1 {
        // Should remove just one ActiveNvaGuerrillas and 1 UndergroundNvaGuerrilla.
        forces_mutations.push(ForcesMutation::new(
            Forces::ActiveNvaGuerrilla,
            MutationTypes::Move,
            1,
            Some(target.get_space_identifier()?),
            Some(SpaceIdentifiers::Available),
        ));
        forces_mutations.push(ForcesMutation::new(
            Forces::UndergroundNvaGuerrilla,
            MutationTypes::Move,
            1,
            Some(target.get_space_identifier()?),
            Some(SpaceIdentifiers::Available),
        ));
    } else {
        forces_mutations.push(ForcesMutation::new(
            Forces::UndergroundNvaGuerrilla,
            MutationTypes::Move,
            2,
            Some(target.get_space_identifier()?),
            Some(SpaceIdentifiers::Available),
        ));
    }

    // Place a regular Nva Base on the target space.
    forces_mutations.push(ForcesMutation::new(
        Forces::NvaBase,
        MutationTypes::Move,
        1,
        Some(SpaceIdentifiers::Available),
        Some(target.get_space_identifier()?),
    ));

    Ok(())
}
