use board::domain::board::Board;
use cards::domain::card::Cards;
use game_definitions::event_types::EventTypes;
use game_definitions::factions::Factions;
use players::domain::player_type::PlayerType;
use std::collections::HashMap;

pub fn check_event_effectivity_for_card_47(
    _active_card: &Cards,
    _preview_card: &Cards,
    player_types: HashMap<Factions, PlayerType>,
    faction: &Factions,
    _preferible_event_type: EventTypes,
    _board: &Board,
) -> Result<bool, String> {
    // US and VC have grey rifles
    if (faction == &Factions::VC || faction == &Factions::US)
        && player_types.get(&faction).unwrap() == &PlayerType::Ai
    {
        return Ok(false);
    }

    if faction == &Factions::NVA {
        // Shaded: Place up to 10 NVA Troops anywhere within 1 space of North Vietnam.
        // Always effective.
        return Ok(true);
    }

    panic!("Not implemented for ARVN.");
}
