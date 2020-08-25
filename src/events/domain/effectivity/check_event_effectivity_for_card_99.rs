use board::domain::board::Board;
use board::domain::queries::board_level_queries::is_there_a_specific_force_anywhere::is_there_a_specific_force_anywhere;
use cards::domain::card::Cards;
use game_definitions::event_types::EventTypes;
use game_definitions::factions::Factions;
use game_definitions::forces::Forces;
use players::domain::player_type::PlayerType;
use std::collections::HashMap;

pub fn check_event_effectivity_for_card_99(
    _active_card: &Cards,
    _preview_card: &Cards,
    player_types: HashMap<Factions, PlayerType>,
    faction: &Factions,
    _preferible_event_type: EventTypes,
    board: &Board,
) -> Result<bool, String> {
    if player_types.get(faction).unwrap() == &PlayerType::Ai && faction == &Factions::NVA {
        // Shaded: VC or NVA free March Guerrillas to any 3 spaces then free Ambush in each (even if Active).
        // To be effective there must be any guerrillas in the map.

        return Ok(
            is_there_a_specific_force_anywhere(Forces::UndergroundNvaGuerrilla, &board)?
                || is_there_a_specific_force_anywhere(Forces::ActiveNvaGuerrilla, &board)?,
        );
    }

    panic!("Card 99 only implemented for NVA AI.");
}
