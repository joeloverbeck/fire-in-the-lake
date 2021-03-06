use board::domain::board::Board;
use board::domain::queries::board_level_queries::is_there_a_specific_force_anywhere::is_there_a_specific_force_anywhere;
use cards::domain::card::Cards;
use game_definitions::event_types::EventTypes;
use game_definitions::factions::Factions;
use game_definitions::forces::Forces;
use players::domain::player_type::PlayerType;
use std::collections::HashMap;

pub fn check_event_effectivity_for_card_9(
    _active_card: &Cards,
    _preview_card: &Cards,
    player_types: HashMap<Factions, PlayerType>,
    faction: &Factions,
    _preferible_event_type: EventTypes,
    board: &Board,
) -> Result<bool, String> {
    // ARVN has grey rifle. US has special instructions.
    if faction == &Factions::ARVN && player_types.get(&faction).unwrap() == &PlayerType::Ai {
        return Ok(false);
    }

    if faction == &Factions::NVA || faction == &Factions::VC {
        // Shaded: US takes 3 of its Troops from the map out of play.
        // Virtually always effective, but will check anyway.

        return Ok(is_there_a_specific_force_anywhere(Forces::UsTroop, board)?);
    }

    panic!("Not implemented for US");
}
