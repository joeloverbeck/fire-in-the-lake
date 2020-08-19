use board::domain::board::Board;
use cards::domain::card::Cards;
use game_definitions::event_type::EventType;
use game_definitions::factions::Factions;
use players::domain::player_type::PlayerType;

pub fn check_event_effectivity_for_card_1(
    _active_card: &Cards,
    _preview_card: &Cards,
    _player_type: &PlayerType,
    _faction: &Factions,
    _preferible_event_type: EventType,
    _board: &Board,
) -> Result<bool, String> {
    panic!("Not implemented for card 1");
}
