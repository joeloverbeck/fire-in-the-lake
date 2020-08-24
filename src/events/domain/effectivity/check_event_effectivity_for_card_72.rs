use board::domain::board::Board;
use cards::domain::card::Cards;
use game_definitions::event_types::EventTypes;
use game_definitions::factions::Factions;
use players::domain::player_type::PlayerType;
use std::collections::HashMap;

pub fn check_event_effectivity_for_card_72(
    _active_card: &Cards,
    _preview_card: &Cards,
    player_types: HashMap<Factions, PlayerType>,
    faction: &Factions,
    _preferible_event_type: EventTypes,
    _board: &Board,
) -> Result<bool, String> {
    // ARVN has special instructions, US has a grey rifle
    if faction == &Factions::US && player_types.get(&faction).unwrap() == &PlayerType::Ai {
        return Ok(false);
    }

    // Shaded: Place 1 VC Guerrilla in each Active Opposition space, 2 NVA Troops in each Laos/Cambodia space.
    // This is virtually always effective. Maybe edge cases will pop up.
    if faction == &Factions::NVA || faction == &Factions::VC {
        return Ok(true);
    }

    panic!("Only implemented for US AI, NVA and VC.");
}
