use board::domain::board::Board;
use cards::domain::card::Cards;
use game_definitions::event_types::EventTypes;
use game_definitions::factions::Factions;
use game_definitions::forces::Forces;
use players::domain::player_type::PlayerType;
use std::collections::HashMap;

pub fn check_event_effectivity_for_card_39(
    _active_card: &Cards,
    _preview_card: &Cards,
    player_types: HashMap<Factions, PlayerType>,
    faction: &Factions,
    _preferible_event_type: EventTypes,
    board: &Board,
) -> Result<bool, String> {
    // It has the grey rifle, so AI VC will never play this event.
    if player_types.get(faction).unwrap() == &PlayerType::Ai && faction == &Factions::VC {
        return Ok(false);
    }

    if faction == &Factions::NVA {
        // Shaded: 1 Available US Troop to out of play.
        // So it will be playable if there are available Us Troops.
        // Shouldn't need a query for this one.
        return Ok(board.get_forces_available(Forces::UsTroop)? > 0);
    }

    panic!("Card 39 only implemented for NVA and VC AIs.");
}
