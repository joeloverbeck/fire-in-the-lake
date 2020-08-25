use board::domain::board::Board;
use board::domain::queries::board_level_queries::is_there_a_specific_force_anywhere::is_there_a_specific_force_anywhere;
use cards::domain::card::Cards;
use game_definitions::event_types::EventTypes;
use game_definitions::factions::Factions;
use game_definitions::forces::Forces;
use players::domain::player_type::PlayerType;
use std::collections::HashMap;

pub fn check_event_effectivity_for_card_23(
    _active_card: &Cards,
    _preview_card: &Cards,
    player_types: HashMap<Factions, PlayerType>,
    faction: &Factions,
    _preferible_event_type: EventTypes,
    board: &Board,
) -> Result<bool, String> {
    // It has the grey rifle, so AI ARVN will never play this event. US has special instructions.
    if player_types.get(faction).unwrap() == &PlayerType::Ai && faction == &Factions::ARVN {
        return Ok(false);
    }

    // Shaded: Select a Tunnel space--remove a die roll of US Troops within 1 space of it to Casualties.
    // For now only effective if there is a tunnel space. Maybe in the future will need to check if there are
    // US troops within 1 space.
    if faction == &Factions::NVA || faction == &Factions::VC {
        return Ok(
            is_there_a_specific_force_anywhere(Forces::TunneledVcBase, &board)?
                || is_there_a_specific_force_anywhere(Forces::TunneledNvaBase, &board)?,
        );
    }

    panic!("Not implemented for US.");
}
