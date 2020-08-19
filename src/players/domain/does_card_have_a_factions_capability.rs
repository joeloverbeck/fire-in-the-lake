use cards::domain::card::Card;
use cards::domain::card::Cards;

pub fn does_card_have_a_factions_capability(card: &Cards) -> Result<bool, String> {
    Ok(card.has_any_faction_capability()?)
}
