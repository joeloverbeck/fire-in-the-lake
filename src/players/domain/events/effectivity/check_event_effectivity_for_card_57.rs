use board::domain::board::Board;
use cards::domain::card::Cards;
use game_definitions::event_types::EventTypes;
use game_definitions::factions::Factions;
use game_definitions::forces::Forces;
use players::domain::player_type::PlayerType;
use std::collections::HashMap;

pub fn check_event_effectivity_for_card_57(
    _active_card: &Cards,
    _preview_card: &Cards,
    player_types: HashMap<Factions, PlayerType>,
    faction: &Factions,
    _preferible_event_type: EventTypes,
    board: &Board,
) -> Result<bool, String> {
    // ARVN has grey rifle. VC has special instructions.
    if player_types.get(faction).unwrap() == &PlayerType::Ai && faction == &Factions::ARVN {
        return Ok(false);
    }

    if faction == &Factions::NVA {
        // Shaded: 2 Available Us Troops out of play. NVA add a die roll of Resources.
        // Effective if there are available troops.
        return Ok(board.get_forces_available(Forces::UsTroop)? > 0);
    }

    panic!("Only implemented for ARVN and NVA AIs.");
}
