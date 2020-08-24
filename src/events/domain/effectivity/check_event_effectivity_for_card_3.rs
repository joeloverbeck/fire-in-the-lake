use board::domain::board::Board;
use cards::domain::card::Cards;
use game_definitions::event_types::EventTypes;
use game_definitions::factions::Factions;
use players::domain::player_type::PlayerType;
use std::collections::HashMap;

pub fn check_event_effectivity_for_card_3(
    _active_card: &Cards,
    _preview_card: &Cards,
    player_types: HashMap<Factions, PlayerType>,
    faction: &Factions,
    _preferible_event_type: EventTypes,
    _board: &Board,
) -> Result<bool, String> {
    // Both ARVN and VC have grey rifles

    if (faction == &Factions::ARVN || faction == &Factions::VC)
        && player_types.get(&faction).unwrap() == &PlayerType::Ai
    {
        return Ok(false);
    }

    // Shaded: NVA Resources +9. If Trail 0-2, Improve to 3.
    // NVA could always play this.
    if faction == &Factions::NVA {
        return Ok(true);
    }

    panic!("Not implemented for US");
}
