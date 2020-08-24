use board::controllers::queries_controller::QueriesController;
use board::domain::board::Board;
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

    let queries_controller = QueriesController::new();

    if faction == &Factions::NVA {
        // NVA has no special instructions for AI.
        // Unshaded: Place 1 NVA Base at NVA Control outside the South and flip any 3 NVA Guerrillas Underground.
        if (queries_controller.is_any_space_at_a_specific_control_type_in_a_geographic_area(
            &ControlTypes::Nva,
            &GeographicAreas::NorthVietnam,
            &board,
        )? || queries_controller.is_any_space_at_a_specific_control_type_in_a_geographic_area(
            &ControlTypes::Nva,
            &GeographicAreas::Cambodia,
            &board,
        )? || queries_controller.is_any_space_at_a_specific_control_type_in_a_geographic_area(
            &ControlTypes::Nva,
            &GeographicAreas::Laos,
            &board,
        )?) || queries_controller
            .is_there_a_specific_force_anywhere(Forces::ActiveNvaGuerrilla, &board)?
        {
            return Ok(true);
        }
    }

    Ok(false)
}
