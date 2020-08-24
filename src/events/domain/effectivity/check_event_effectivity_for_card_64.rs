use board::domain::board::Board;
use cards::domain::card::Cards;
use game_definitions::event_types::EventTypes;
use game_definitions::factions::Factions;
use players::domain::player_type::PlayerType;
use std::collections::HashMap;

pub fn check_event_effectivity_for_card_64(
    _active_card: &Cards,
    _preview_card: &Cards,
    player_types: HashMap<Factions, PlayerType>,
    faction: &Factions,
    _preferible_event_type: EventTypes,
    _board: &Board,
) -> Result<bool, String> {
    // US and NVA have special instructions

    // There is no shaded or unshaded, just the following:
    // Aid +10 or -10. Patronage +3 or -5. If US or ARVN executing, that faction Pacifies
    // as if Support Phase. If Insurgent executing, that Faction remains Elegible.

    // NVA special instructions: if ARVN a non-player, choose Op & Special Activity.
    // Otherwise, decrease all.
    if faction == &Factions::NVA && player_types.get(&faction).unwrap() == &PlayerType::Ai {
        if player_types.get(&Factions::ARVN).unwrap() == &PlayerType::Ai {
            return Ok(false);
        }

        return Ok(true);
    }

    panic!("Only implemented for NVA.");
}
