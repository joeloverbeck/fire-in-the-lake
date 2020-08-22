use board::controllers::queries_controller::QueriesController;
use board::domain::board::Board;
use cards::domain::card::Cards;
use game_definitions::event_types::EventTypes;
use game_definitions::factions::Factions;
use players::domain::player_type::PlayerType;
use sequence_of_play::controllers::sequence_of_play_controller::SequenceOfPlayController;
use std::collections::HashMap;

pub fn check_event_effectivity_for_card_54(
    _active_card: &Cards,
    _preview_card: &Cards,
    player_types: HashMap<Factions, PlayerType>,
    faction: &Factions,
    _preferible_event_type: EventTypes,
    board: &Board,
    sequence_of_play_controller: &SequenceOfPlayController,
) -> Result<bool, String> {
    // ARVN has grey rifle, NVA and VC have special instructions.

    if faction == &Factions::ARVN && player_types.get(&faction).unwrap() == &PlayerType::Ai {
        return Ok(false);
    }

    if (faction == &Factions::NVA || faction == &Factions::VC)
        && player_types.get(&faction).unwrap() == &PlayerType::Ai
    {
        // Shaded: Any 2 Casualties out of play. US Ineligible through next card.
        // NVA & VC: If US in Ineligible box, Choose Op & Special Activity instead.
        if sequence_of_play_controller.is_faction_ineligible(&Factions::US)? {
            return Ok(false);
        }

        // At this point only effective if any casualties.
        let queries_controller = QueriesController::new();

        return Ok(queries_controller.are_there_any_casualties(board)?);
    }

    panic!("Not implemented for US.");
}
