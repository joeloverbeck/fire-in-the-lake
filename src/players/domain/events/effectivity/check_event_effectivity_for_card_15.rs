use board::domain::board::Board;
use cards::domain::card::Cards;
use game_definitions::event_type::EventType;
use game_definitions::factions::Factions;
use players::domain::player_type::PlayerType;

pub fn check_event_effectivity_for_card_15(
    _active_card: &Cards,
    _preview_card: &Cards,
    player_type: &PlayerType,
    faction: &Factions,
    _preferible_event_type: EventType,
    _board: &Board,
) -> Result<bool, String> {
    if player_type == &PlayerType::Ai
        && (faction == &Factions::NVA || faction == &Factions::ARVN || faction == &Factions::VC)
    {
        // Has grey rifle for all of them.
        return Ok(false);
    }

    panic!("Not implemented for card 15");
}
