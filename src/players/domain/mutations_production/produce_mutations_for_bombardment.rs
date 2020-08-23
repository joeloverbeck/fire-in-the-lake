use board::domain::space::Space;
use board::domain::space::Spaces;
use game_definitions::forces::Forces;
use game_definitions::space_identifiers::SpaceIdentifiers;
use players::domain::forces_mutation::ForcesMutation;
use players::domain::mutation_types::MutationTypes;

pub fn produce_mutations_for_bombardment(
    target: &Spaces,
    forces_mutations: &mut Vec<ForcesMutation>,
) -> Result<(), String> {
    // Remove 1 US Troop or ARVN Troop cube from the location. If US, to the Casualties box.
    if target.get_forces(Forces::UsTroop)? > 0 {
        // Will remove 1 Us Troop to casualties.
        forces_mutations.push(ForcesMutation::new(
            Forces::UsTroop,
            MutationTypes::Move,
            1,
            Some(target.get_space_identifier()?),
            Some(SpaceIdentifiers::Casualties),
        ));
    } else if target.get_forces(Forces::ArvnTroop)? > 0 {
        forces_mutations.push(ForcesMutation::new(
            Forces::ArvnTroop,
            MutationTypes::Reduce,
            1,
            Some(target.get_space_identifier()?),
            Some(SpaceIdentifiers::Available),
        ));
    } else {
        panic!("There were no Us Troops nor Arvn Troops to eliminate from the target space! Space: {:?}", target);
    }

    Ok(())
}
