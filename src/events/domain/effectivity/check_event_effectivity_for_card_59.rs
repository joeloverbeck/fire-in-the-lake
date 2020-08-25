use board::domain::queries::board_level_queries::would_marching_a_particular_force_into_space_identifiers_turn_any_into_nva_control::would_marching_a_particular_force_into_space_identifiers_turn_any_into_nva_control;
use board::domain::board::Board;
use board::domain::queries::board_level_queries::get_space_identifiers_with_a_particular_force::get_space_identifiers_with_a_particular_force;
use board::domain::queries::board_level_queries::is_there_a_specific_force_anywhere::is_there_a_specific_force_anywhere;
use cards::domain::card::Cards;
use game_definitions::event_types::EventTypes;
use game_definitions::factions::Factions;
use game_definitions::forces::Forces;
use players::domain::player_type::PlayerType;
use std::collections::HashMap;

pub fn check_event_effectivity_for_card_59(
    _active_card: &Cards,
    _preview_card: &Cards,
    player_types: HashMap<Factions, PlayerType>,
    faction: &Factions,
    _preferible_event_type: EventTypes,
    board: &Board,
) -> Result<bool, String> {
    // All four factions have special instructions for this one.
    if player_types.get(faction).unwrap() == &PlayerType::Ai && faction == &Factions::NVA {
        // Special NVA instructions for this event:
        // First March most Troops possible to a US Base. If Event would add 0 total NVA Control, choose
        // Op & Special Activity instead.
        // Rules: The NVA alone Control a Province or City if NVA pieces exceed all other pieces (including VC).

        // Obviously, there needs to be a US Base in the map.
        if !is_there_a_specific_force_anywhere(Forces::UsBase, &board)? {
            return Ok(false);
        }

        // We need to see if moving Nva forces (troops apparently) to a province with Us Base would change it to Nva Control.
        let space_identifiers_with_us_bases =
            get_space_identifiers_with_a_particular_force(Forces::UsBase, &board)?;

        return Ok(
            would_marching_a_particular_force_into_space_identifiers_turn_any_into_nva_control(
                Forces::NvaTroop,
                space_identifiers_with_us_bases,
                &board,
            )?,
        );
    }

    panic!("Card 59 only implemented for NVA AI.");
}
