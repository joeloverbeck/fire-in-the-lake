use board::domain::board::Board;
use cards::domain::card::Cards;
use game_definitions::event_types::EventTypes;
use game_definitions::factions::Factions;
use players::domain::player_type::PlayerType;
use std::collections::HashMap;

pub fn check_event_effectivity_for_card_74(
    _active_card: &Cards,
    _preview_card: &Cards,
    player_types: HashMap<Factions, PlayerType>,
    faction: &Factions,
    _preferible_event_type: EventTypes,
    _board: &Board,
) -> Result<bool, String> {
    // VC has grey rifle. US has special instructions.
    if player_types.get(faction).unwrap() == &PlayerType::Ai && faction == &Factions::VC {
        return Ok(false);
    }

    if faction == &Factions::NVA {
        // Shaded: NVA Resources +6 and +1 more for each ARVN piece in Laos.
        // Always effective.
        return Ok(true);
    }

    panic!("Card 70 only implemented for VC and NVA AIs.");
}
