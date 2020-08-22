use board::controllers::queries_controller::QueriesController;
use board::domain::board::Board;
use cards::domain::card::Cards;
use game_definitions::event_types::EventTypes;
use game_definitions::factions::Factions;
use game_definitions::forces::Forces;
use players::domain::player_type::PlayerType;
use std::collections::HashMap;

pub fn check_event_effectivity_for_card_105(
    _active_card: &Cards,
    _preview_card: &Cards,
    player_types: HashMap<Factions, PlayerType>,
    faction: &Factions,
    _preferible_event_type: EventTypes,
    board: &Board,
) -> Result<bool, String> {
    // ARVN and NVA have special instructions.

    if faction == &Factions::NVA && player_types.get(&faction).unwrap() == &PlayerType::Ai {
        // Shaded: Shift 3 Provinces with Police each by 1 level toward Active Opposition. Patronage +6 or -6.
        // NVA: If VC a player, choose Op & Special Activity.

        if player_types.get(&Factions::VC).unwrap() == &PlayerType::Human {
            return Ok(false);
        }

        // Test at least if there are any provinces with Police.
        let queries_controller = QueriesController::new();

        return Ok(queries_controller
            .is_there_a_specific_force_in_any_province(Forces::ArvnPolice, board)?);
    }

    panic!("Only implemented for NVA AI.");
}
