use board::domain::board::Board;
use cards::domain::card::Cards;
use game_definitions::event_types::EventTypes;
use game_definitions::factions::Factions;
use game_definitions::forces::Forces;
use players::domain::player_type::PlayerType;
use std::collections::HashMap;

pub fn check_event_effectivity_for_card_108(
    _active_card: &Cards,
    _preview_card: &Cards,
    player_types: HashMap<Factions, PlayerType>,
    faction: &Factions,
    _preferible_event_type: EventTypes,
    board: &Board,
) -> Result<bool, String> {
    // Both VC and ARVN have grey rifles
    if player_types.get(faction).unwrap() == &PlayerType::Ai
        && (faction == &Factions::VC || faction == &Factions::ARVN)
    {
        return Ok(false);
    }

    if faction == &Factions::NVA {
        // Shaded: Move 1 Us Troop per Casualty piece, to a maximum of 3, from Available to out of play.
        // This will only be effective if there are casualties.
        return Ok(board.get_forces_in_casualties(Forces::UsTroop)? > 0);
    }

    panic!("Only implemented for VC, ARVN and NVA.");
}
