use board::domain::queries::board_level_queries::is_any_space_at_a_specific_control_type_in_a_geographic_area::is_any_space_at_a_specific_control_type_in_a_geographic_area;
use board::domain::board::Board;
use board::domain::queries::board_level_queries::is_there_a_specific_force_anywhere::is_there_a_specific_force_anywhere;
use cards::domain::card::Cards;
use game_definitions::control_types::ControlTypes;
use game_definitions::event_types::EventTypes;
use game_definitions::factions::Factions;
use game_definitions::forces::Forces;
use game_definitions::geographic_areas::GeographicAreas;
use players::domain::player_type::PlayerType;
use std::collections::HashMap;

pub fn check_event_effectivity_for_card_12(
    _active_card: &Cards,
    _preview_card: &Cards,
    player_types: HashMap<Factions, PlayerType>,
    faction: &Factions,
    _preferible_event_type: EventTypes,
    board: &Board,
) -> Result<bool, String> {
    // For Insurgents: VC has the grey rifle, NVA has no special instructions.
    if player_types.get(&faction).unwrap() == &PlayerType::Ai && faction == &Factions::VC {
        return Ok(false);
    }

    if faction == &Factions::NVA {
        // NVA has no special instructions for AI.
        // Unshaded: Place 1 NVA Base at NVA Control outside the South and flip any 3 NVA Guerrillas Underground.
        if (is_any_space_at_a_specific_control_type_in_a_geographic_area(
            &ControlTypes::Nva,
            &GeographicAreas::NorthVietnam,
            &board,
        )? || is_any_space_at_a_specific_control_type_in_a_geographic_area(
            &ControlTypes::Nva,
            &GeographicAreas::Cambodia,
            &board,
        )? || is_any_space_at_a_specific_control_type_in_a_geographic_area(
            &ControlTypes::Nva,
            &GeographicAreas::Laos,
            &board,
        )?) || is_there_a_specific_force_anywhere(Forces::ActiveNvaGuerrilla, &board)?
        {
            return Ok(true);
        }
    }

    Ok(false)
}
