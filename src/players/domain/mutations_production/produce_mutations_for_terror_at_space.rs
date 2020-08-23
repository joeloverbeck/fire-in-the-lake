use board::domain::space::Space;
use board::domain::space::Spaces;
use game_definitions::factions::Factions;
use game_definitions::forces::Forces;
use game_definitions::support_levels::SupportLevels;
use players::domain::forces_mutation::ForcesMutation;
use players::domain::mutation_types::MutationTypes;
use players::domain::space_mutation::SpaceMutation;
use players::domain::space_mutation_types::SpaceMutationTypes;

pub fn produce_mutations_for_terror_at_space(
    space: &Spaces,
    faction: Factions,
    forces_mutations: &mut Vec<ForcesMutation>,
    space_mutations: &mut Vec<SpaceMutation>,
) -> Result<(), String> {
    // Terror Operations in Cities or Provinces affect Support and Opposition (1.6) and
    // place Terror markers that hinder future efforts to
    // influence it. On LoCs, they place Sabotage markers that block ARVN
    // Resource earnings (6.2.1). Select any spaces where the executing
    // Faction has at least 1 Underground Guerrilla or, for NVA Terror, NVA
    // Troop cube; pay 1 Resource per Province or City (0 for LoCs).

    // PROCEDURE: Activate 1 of the executing Faction’s Underground
    // Guerrillas in each selected space (if any there).
    // • If the space is a Province or City without a Terror marker, place
    // a Terror marker. If VC, shift 1 level toward Active Opposition
    // (1.6). If NVA, shift any Support 1 level toward Neutral.
    // • If the space is a LoC without a Sabotage marker, place a Sabotage
    // marker.
    // • Do not place a Terror/Sabotage marker if all are already on the
    // map. (There are 15.)

    // Additional instructions: choose spaces to reduce Total Support as much as able.
    // Tie breaker: where no Underground NVA Guerrilla.

    match space.is_habitable()? {
        true => {
            match faction {
                Factions::NVA => {
                    // If there's a NVA Troop at the space, just use that. If they don't, they must have an Underground NVA Guerrilla.
                    if space.get_forces(Forces::NvaTroop)? == 0
                        && space.get_forces(Forces::UndergroundNvaGuerrilla)? == 0
                    {
                        panic!("Attempted to produce the mutation for terror at target for NVA, but there were neither NVA Troops nor Underground NVA Guerrillas there! Space: {:?}", space);
                    }

                    if space.get_forces(Forces::NvaTroop)? == 0 {
                        // Must flip the underground guerrilla.
                        forces_mutations.push(ForcesMutation::new(
                            Forces::UndergroundNvaGuerrilla,
                            MutationTypes::Flip,
                            1,
                            Some(space.get_space_identifier()?),
                            Some(space.get_space_identifier()?),
                        ));
                    }

                    if !space.has_terror()? {
                        // Must set terror in space.
                        space_mutations.push(SpaceMutation::new(
                            space.get_space_identifier()?,
                            SpaceMutationTypes::Terror,
                            MutationTypes::Set,
                            None,
                        ));
                    }

                    // Needs to shift down its support.
                    // Given that the rules for NVA say the following: "If NVA, shift any Support 1 level toward Neutral.", will only create
                    // a shift down mutation for Support as long as the original support level is higher than neutral.
                    if space.get_support_level()? == &SupportLevels::ActiveSupport
                        || space.get_support_level()? == &SupportLevels::PassiveSupport
                    {
                        space_mutations.push(SpaceMutation::new(
                            space.get_space_identifier()?,
                            SpaceMutationTypes::Support,
                            MutationTypes::ShiftDown,
                            None,
                        ));
                    }
                }
                Factions::VC => {
                    panic!("Produce mutations for terror at space not implemented for VC.")
                }
                _ => panic!(
                    "Produce mutations for terror at space not implemented for faction {:?}",
                    faction
                ),
            }
        }
        false => panic!("Produce mutations for terror at space not implemented for LoCs"),
    }

    Ok(())
}
