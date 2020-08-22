use board::domain::board::Board;
use cards::domain::card::Cards;
use game_definitions::event_types::EventTypes;
use game_definitions::factions::Factions;
use players::domain::player_type::PlayerType;
use std::collections::HashMap;

pub fn check_event_effectivity_for_card_77(
    _active_card: &Cards,
    _preview_card: &Cards,
    player_types: HashMap<Factions, PlayerType>,
    faction: &Factions,
    _preferible_event_type: EventTypes,
    _board: &Board,
) -> Result<bool, String> {
    // ARVN has a grey rifle, NVA and US have special instructions
    if player_types.get(faction).unwrap() == &PlayerType::Ai && faction == &Factions::ARVN {
        return Ok(false);
    }

    if faction == &Factions::NVA && player_types.get(faction).unwrap() == &PlayerType::Ai {
        // Shaded: NVA add +9 Resources or free Infiltrate. Then VC free Rally in up to 6 spaces.
        // Special instructions: Choose resources if NVA Resources < 20, otherwise Infiltrate.
        // It's always effective. Maybe edge cases.
        return Ok(true);
    }

    panic!("Card 77 not implemented for US and VC.");
}
