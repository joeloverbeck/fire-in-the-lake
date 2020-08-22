use board::domain::board::Board;
use cards::domain::card::Cards;
use game_definitions::event_types::EventTypes;
use game_definitions::factions::Factions;
use players::domain::player_type::PlayerType;
use std::collections::HashMap;

pub fn check_event_effectivity_for_card_22(
    _active_card: &Cards,
    _preview_card: &Cards,
    player_types: HashMap<Factions, PlayerType>,
    faction: &Factions,
    _preferible_event_type: EventTypes,
    _board: &Board,
) -> Result<bool, String> {
    // ARVN has grey rifle, US special instructions.
    if faction == &Factions::ARVN && player_types.get(&faction).unwrap() == &PlayerType::Ai {
        return Ok(false);
    }

    // Shaded: Remove all Support within 1 Space of Da Nang. No Air Strike until Coup. MOMENTUM.
    // This is always effective for both VC and NVA.
    if faction == &Factions::NVA || faction == &Factions::VC {
        return Ok(true);
    }

    panic!("Not implemented for US.");
}
