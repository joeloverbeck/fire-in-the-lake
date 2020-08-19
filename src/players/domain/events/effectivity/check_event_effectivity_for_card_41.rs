use board::domain::board::Board;
use cards::domain::card::Cards;
use game_definitions::event_type::EventType;
use game_definitions::factions::Factions;
use players::domain::player_type::PlayerType;

pub fn check_event_effectivity_for_card_41(
    _active_card: &Cards,
    _preview_card: &Cards,
    player_type: &PlayerType,
    _faction: &Factions,
    _preferible_event_type: EventType,
    _board: &Board,
) -> Result<bool, String> {
    if player_type == &PlayerType::Ai {
        // It has grey rifles for all the factions. None of them will play it if they are AI.
        return Ok(false);
    }

    panic!("Human event effectivity not implemented for card 41");
}
