use cards::domain::card::Card;
use cards::domain::card::Cards;
use game_definitions::factions::Factions;
use players::domain::does_card_have_a_factions_capability::does_card_have_a_factions_capability;

pub fn does_card_have_a_factions_capability_but_not_of_a_specific_one(
    card: &Cards,
    faction: &Factions,
) -> Result<bool, String> {
    Ok(
        does_card_have_a_factions_capability(card)?
            && !(card.get_faction_capability()? == *faction),
    )
}
