use board::domain::board::Board;
use cards::domain::card::Cards;
use game_definitions::event_types::EventTypes;
use game_definitions::factions::Factions;
use players::domain::player_type::PlayerType;
use std::collections::HashMap;

pub fn check_event_effectivity_for_card_51(
    _active_card: &Cards,
    _preview_card: &Cards,
    player_types: HashMap<Factions, PlayerType>,
    faction: &Factions,
    _preferible_event_type: EventTypes,
    _board: &Board,
) -> Result<bool, String> {
    // It has grey rifles for VC, US and ARVN
    if player_types.get(faction).unwrap() == &PlayerType::Ai
        && (faction == &Factions::US || faction == &Factions::VC || faction == &Factions::ARVN)
    {
        return Ok(false);
    }

    // For the remaining, NVA: Improve Trail by 2 boxes and add a die roll of NVA Resources.
    // This is always playable.
    Ok(true)
}
