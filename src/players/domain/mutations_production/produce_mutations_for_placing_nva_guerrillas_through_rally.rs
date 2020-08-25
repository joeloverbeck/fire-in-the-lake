use board::domain::board::Board;
use board::domain::space::Space;
use board::domain::space::Spaces;
use game_definitions::faction_stats::FactionStats;
use game_definitions::forces::Forces;
use game_definitions::space_identifiers::SpaceIdentifiers;
use players::domain::forces_mutation::ForcesMutation;
use players::domain::mutation_types::MutationTypes;

pub fn produce_mutations_for_placing_nva_guerrillas_through_rally(
    forces_mutations: &mut Vec<ForcesMutation>,
    available_nva_guerrillas: u8,
    target: &Spaces,
    board: &Board,
) -> Result<u8, String> {
    // Sanity check: there must be *any* nva guerrillas available to place.
    if available_nva_guerrillas == 0 {
        panic!("Was going to produce mutations for placing nva guerrillas through rally, but there were none available!");
    }

    // If space already has at least 1 of NVA's bases, the faction may:
    // -Place a number of its guerrillas up to the sum of Trail value plus the number of NVA Bases there.
    // Else it just places 1 UndergroundNvaGuerrilla.
    if target.get_forces(Forces::NvaBase)? >= 1 {
        let max_number_of_nva_guerrillas_to_place =
            board.get_faction_stat(FactionStats::TheTrail)? + target.get_forces(Forces::NvaBase)?;

        let number_of_guerrillas_to_rally = {
            if max_number_of_nva_guerrillas_to_place > available_nva_guerrillas {
                available_nva_guerrillas
            } else {
                max_number_of_nva_guerrillas_to_place
            }
        };

        forces_mutations.push(ForcesMutation::new(
            Forces::UndergroundNvaGuerrilla,
            MutationTypes::Move,
            number_of_guerrillas_to_rally,
            Some(SpaceIdentifiers::Available),
            Some(target.get_space_identifier()?),
        ));

        Ok(number_of_guerrillas_to_rally)
    } else {
        forces_mutations.push(ForcesMutation::new(
            Forces::UndergroundNvaGuerrilla,
            MutationTypes::Move,
            1,
            Some(SpaceIdentifiers::Available),
            Some(target.get_space_identifier()?),
        ));

        Ok(1)
    }
}
