use cards::card::Card;
use factions::Factions;

pub fn get_position_in_faction_order_of_faction(card: &Card, faction: Factions) -> Option<usize> {
    card.get_faction_order()
        .iter()
        .enumerate()
        .filter_map(|faction_in_order| {
            if faction_in_order.1 == &faction {
                Some(faction_in_order.0)
            } else {
                None
            }
        })
        .next()
}
