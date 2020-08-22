use board::domain::board::Board;
use cards::domain::card::Cards;
use game_definitions::event_types::EventTypes;
use game_definitions::factions::Factions;
use players::domain::player_type::PlayerType;
use std::collections::HashMap;

pub fn check_event_effectivity_for_card_60(
    _active_card: &Cards,
    _preview_card: &Cards,
    player_types: HashMap<Factions, PlayerType>,
    faction: &Factions,
    _preferible_event_type: EventTypes,
    _board: &Board,
) -> Result<bool, String> {
    // it has grey rifles for VC and ARVN AIs
    if (faction == &Factions::ARVN || faction == &Factions::VC)
        && player_types.get(&faction).unwrap() == &PlayerType::Ai
    {
        return Ok(false);
    }

    // Shaded: NVa place 6 Troops outside South Vietnam, add +6 resources, and, if executing, stay Eligible.
    // Adding Resources will virtually always be possible.
    if faction == &Factions::NVA && player_types.get(&faction).unwrap() == &PlayerType::Ai {
        return Ok(true);
    }

    panic!("Event 60 implemented for VC, ARVN and NVA Ais");
}
