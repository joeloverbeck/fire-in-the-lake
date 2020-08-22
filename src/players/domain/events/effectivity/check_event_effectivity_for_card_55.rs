use board::controllers::queries_controller::QueriesController;
use board::domain::board::Board;
use cards::domain::card::Cards;
use game_definitions::event_types::EventTypes;
use game_definitions::faction_stats::FactionStats;
use game_definitions::factions::Factions;
use game_definitions::forces::Forces;
use game_definitions::geographic_areas::GeographicAreas;
use players::domain::player_type::PlayerType;
use std::collections::HashMap;

pub fn check_event_effectivity_for_card_55(
    _active_card: &Cards,
    _preview_card: &Cards,
    player_types: HashMap<Factions, PlayerType>,
    faction: &Factions,
    _preferible_event_type: EventTypes,
    board: &Board,
) -> Result<bool, String> {
    // US has a grey rifle
    if faction == &Factions::US && player_types.get(&faction).unwrap() == &PlayerType::Ai {
        return Ok(false);
    }

    // Shaded: Add twice Trail value to each NVA and VC resources. NVA moves its unTunneled Bases
    // anywhere within Laos/Cambodia.
    // Only doable if trail is > 0, and there are untunneled NVA bases in Laos or in Cambodia
    let queries_controller = QueriesController::new();

    if faction == &Factions::VC
        || faction == &Factions::NVA && board.get_faction_stat(FactionStats::TheTrail)? > 0
        || (queries_controller.is_there_any_number_of_a_particular_force_in_a_geographic_area(
            Forces::NvaBase,
            &GeographicAreas::Cambodia,
            &board,
        )? || queries_controller
            .is_there_any_number_of_a_particular_force_in_a_geographic_area(
                Forces::NvaBase,
                &GeographicAreas::Laos,
                &board,
            )?)
    {
        return Ok(true);
    }

    panic!("Only implemented for US, NVA and VC AIs");
}
