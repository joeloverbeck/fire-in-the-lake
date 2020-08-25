use board::domain::board::Board;
use board::domain::queries::board_level_queries::is_there_a_specific_force_anywhere::is_there_a_specific_force_anywhere;
use cards::domain::card::Cards;
use game_definitions::event_types::EventTypes;
use game_definitions::factions::Factions;
use game_definitions::forces::Forces;
use players::domain::player_type::PlayerType;
use std::collections::HashMap;

pub fn check_event_effectivity_for_card_49(
    _active_card: &Cards,
    _preview_card: &Cards,
    _player_types: HashMap<Factions, PlayerType>,
    faction: &Factions,
    _preferible_event_type: EventTypes,
    board: &Board,
) -> Result<bool, String> {
    // All have special instructions but for NVA.

    if faction == &Factions::NVA {
        // Shaded: NVA in any 3 spaces places enough Troops to double their number. If then free Bombard.
        // Only effective if Nva Troops anywhere.
        return Ok(is_there_a_specific_force_anywhere(Forces::NvaTroop, board)?);
    }

    panic!("Only implemented for NVA.");
}
