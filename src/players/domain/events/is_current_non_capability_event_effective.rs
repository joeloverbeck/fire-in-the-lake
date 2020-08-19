use cards::domain::card::Card;
use cards::domain::card::Cards;
use game_definitions::event_type::EventType;
use game_definitions::factions::Factions;
use players::domain::events::effectivity::check_event_effectivity_for_card_46::check_event_effectivity_for_card_46;
use players::domain::player_type::PlayerType;

pub fn is_current_non_capability_event_effective(
    active_card: &Cards,
    preview_card: &Cards,
    player_type: &PlayerType,
    faction: &Factions,
    preferible_event_type: EventType,
) -> Result<bool, String> {
    // Sanity check: we specify that the event should be non-capability. That means that the caller should have checked first whether it was.
    if active_card.has_any_faction_capability()? {
        // Don't hard crash, though.
        return Ok(false);
    }

    if active_card.get_number()? == 46 {
        Ok(check_event_effectivity_for_card_46(
            active_card,
            preview_card,
            player_type,
            faction,
            preferible_event_type,
        )?)
    } else {
        panic!(
            "Checking event effectivity not implemented for event {:?}",
            active_card.get_number()?
        );
    }
}
