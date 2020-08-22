use board::controllers::queries_controller::QueriesController;
use board::domain::board::Board;
use cards::domain::card::Cards;
use game_definitions::event_types::EventTypes;
use game_definitions::factions::Factions;
use game_definitions::forces::Forces;
use game_definitions::terrain_types::TerrainTypes;
use players::domain::player_type::PlayerType;
use std::collections::HashMap;

pub fn check_event_effectivity_for_card_71(
    _active_card: &Cards,
    _preview_card: &Cards,
    player_types: HashMap<Factions, PlayerType>,
    faction: &Factions,
    _preferible_event_type: EventTypes,
    board: &Board,
) -> Result<bool, String> {
    // US and VC have grey rifles. NVA has special instructions.

    if (faction == &Factions::US || faction == &Factions::VC)
        && player_types.get(&faction).unwrap() == &PlayerType::Ai
    {
        return Ok(false);
    }

    if faction == &Factions::NVA && player_types.get(&faction).unwrap() == &PlayerType::Ai {
        // Shaded: NVA free Marches Troops into a City and free Attacks there twice.
        // NVA: First March most Troops possible to a US Base. If Event would add 0 to total NVA Control, choose Op & Special Activity

        // Obviously, there needs to be a US Base in any city.
        let queries_controller = QueriesController::new();

        if !queries_controller.is_there_a_specific_force_in_a_terrain_type(
            Forces::UsBase,
            TerrainTypes::City,
            &board,
        )? {
            return Ok(false);
        }

        // We need to see if moving Nva forces (troops apparently) to a province with Us Base would change it to Nva Control.
        let space_identifiers_with_us_bases = queries_controller
            .get_space_identifiers_with_a_particular_force_and_terrain_type(
                Forces::UsBase,
                TerrainTypes::City,
                &board,
            )?;

        return Ok(queries_controller
            .would_marching_a_particular_force_into_space_identifiers_turn_any_into_nva_control(
                Forces::NvaTroop,
                space_identifiers_with_us_bases,
                &board,
            )?);
    }

    panic!("Not implemented for ARVN.");
}
