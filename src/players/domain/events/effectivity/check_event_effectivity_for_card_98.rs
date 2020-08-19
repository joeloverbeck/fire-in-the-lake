use board::domain::board::Board;
use cards::domain::card::Cards;
use game_definitions::event_type::EventType;
use game_definitions::factions::Factions;
use players::domain::player_type::PlayerType;

pub fn check_event_effectivity_for_card_98(
    _active_card: &Cards,
    _preview_card: &Cards,
    player_type: &PlayerType,
    faction: &Factions,
    _preferible_event_type: EventType,
    _board: &Board,
) -> Result<bool, String> {
    if player_type == &PlayerType::Ai && (faction == &Factions::NVA || faction == &Factions::ARVN) {
        return Ok(false);
    }

    panic!("Card 98 only implemented for NVA and ARVN AIs.");
}
