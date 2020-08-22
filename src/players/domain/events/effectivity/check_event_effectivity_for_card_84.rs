use board::controllers::queries_controller::QueriesController;
use board::domain::board::Board;
use cards::domain::card::Cards;
use game_definitions::event_types::EventTypes;
use game_definitions::factions::Factions;
use players::domain::player_type::PlayerType;
use std::collections::HashMap;

pub fn check_event_effectivity_for_card_84(
    _active_card: &Cards,
    _preview_card: &Cards,
    player_types: HashMap<Factions, PlayerType>,
    faction: &Factions,
    _preferible_event_type: EventTypes,
    board: &Board,
) -> Result<bool, String> {
    // ARVN, US and NVA have special instructions.

    if faction == &Factions::NVA && player_types.get(&faction).unwrap() == &PlayerType::Ai {
        // Shaded: ARVN remove 1 in 3 cubes (round down) each space. Place a VC Guerrilla in 3 spaces
        // where ARVN removed.
        // NVA: If VC a player, choose Op & Special Activity.

        if player_types.get(&Factions::VC).unwrap() == &PlayerType::Ai {
            return Ok(false);
        }

        // At this point this is only effective if there are ARVN cubes anywhere.
        if faction == &Factions::NVA && player_types.get(&faction).unwrap() == &PlayerType::Ai {
            let queries_controller = QueriesController::new();

            return Ok(queries_controller
                .are_there_any_cubes_of_a_faction_anywhere(Factions::ARVN, board)?);
        }
    }

    panic!("Only implemented for NVA AI");
}
