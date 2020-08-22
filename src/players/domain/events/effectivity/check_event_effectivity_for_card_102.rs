use board::controllers::queries_controller::QueriesController;
use board::domain::board::Board;
use cards::domain::card::Cards;
use game_definitions::event_types::EventTypes;
use game_definitions::factions::Factions;
use game_definitions::forces::Forces;
use game_definitions::geographic_areas::GeographicAreas;
use players::domain::player_type::PlayerType;
use std::collections::HashMap;

pub fn check_event_effectivity_for_card_102(
    _active_card: &Cards,
    _preview_card: &Cards,
    player_types: HashMap<Factions, PlayerType>,
    faction: &Factions,
    _preferible_event_type: EventTypes,
    board: &Board,
) -> Result<bool, String> {
    // ARVN has grey rifle, all other factions have special instructions.
    if faction == &Factions::ARVN && player_types.get(&faction).unwrap() == &PlayerType::Ai {
        return Ok(false);
    }

    // Shaded: Place Tunnel markers on each Insurgent Base in 1 Province. Place 1 NVA and 1 VC Guerrilla there.
    // NVA & VC: Place maximum Tunnels of own faction in South Vietnam. If none, choose Op & Special Activity
    if (faction == &Factions::NVA || faction == &Factions::VC)
        && player_types.get(&faction).unwrap() == &PlayerType::Ai
    {
        // Only effective if there are bases of own faction in South Vietnam
        let queries_controller = QueriesController::new();

        if faction == &Factions::NVA {
            return Ok(queries_controller
                .is_there_any_number_of_a_particular_force_in_a_geographic_area(
                    Forces::NvaBase,
                    &GeographicAreas::SouthVietnam,
                    board,
                )?);
        }

        if faction == &Factions::VC {
            return Ok(queries_controller
                .is_there_any_number_of_a_particular_force_in_a_geographic_area(
                    Forces::VcBase,
                    &GeographicAreas::SouthVietnam,
                    board,
                )?);
        }
    }

    panic!("Not implemented for US");
}
