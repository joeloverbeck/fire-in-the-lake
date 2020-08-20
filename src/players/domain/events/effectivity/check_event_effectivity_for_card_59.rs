use board::domain::board::Board;
use cards::domain::card::Cards;
use game_definitions::event_type::EventType;
use game_definitions::factions::Factions;
use players::domain::player_type::PlayerType;
use std::collections::HashMap;

pub fn check_event_effectivity_for_card_59(
    _active_card: &Cards,
    _preview_card: &Cards,
    player_types: HashMap<Factions, PlayerType>,
    faction: &Factions,
    _preferible_event_type: EventType,
    _board: &Board,
) -> Result<bool, String> {
    // All four factions have special instructions for this one.
    if player_types.get(faction).unwrap() == &PlayerType::Ai && faction == &Factions::NVA {
        // Special NVA instructions for this event:
        // First March most Troops possible to a US Base. If Event would add 0 total NVA Control, choose
        // Op & Special Activity instead.
        // Rules: The NVA alone Control a Province or City if NVA pieces exceed all other pieces (including VC).
        todo!()
    }

    panic!("Card 59 only implemented for NVA AI.");
}
