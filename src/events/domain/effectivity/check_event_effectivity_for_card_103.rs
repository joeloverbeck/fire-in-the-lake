use board::domain::board::Board;
use cards::domain::card::Cards;
use game_definitions::event_types::EventTypes;
use game_definitions::faction_stats::FactionStats;
use game_definitions::factions::Factions;
use players::domain::player_type::PlayerType;
use sequence_of_play::controllers::sequence_of_play_controller::SequenceOfPlayController;
use std::collections::HashMap;

pub fn check_event_effectivity_for_card_103(
    _active_card: &Cards,
    _preview_card: &Cards,
    player_types: HashMap<Factions, PlayerType>,
    faction: &Factions,
    _preferible_event_type: EventTypes,
    board: &Board,
    sequence_of_play_controller: &SequenceOfPlayController,
) -> Result<bool, String> {
    // ARVN has grey rifle, VC and NVA have special instructions

    if faction == &Factions::ARVN && player_types.get(&faction).unwrap() == &PlayerType::Ai {
        return Ok(false);
    }

    if (faction == &Factions::NVA || faction == &Factions::VC)
        && player_types.get(&faction).unwrap() == &PlayerType::Ai
    {
        // Shaded: Up to 3 US Troop Casualties out of play. Aid -6. US Ineligible through next card.
        // NVA & VC: If US in Ineligible, choose Op & Special Activity
        if sequence_of_play_controller.is_faction_ineligible(&Factions::US)? {
            return Ok(false);
        }

        // This would only be effective if there are Us Troops as casualties, or Aid > 0.
        // Do minimum check for now.
        return Ok(board.get_faction_stat(FactionStats::Aid)? > 0);
    }

    todo!()
}
