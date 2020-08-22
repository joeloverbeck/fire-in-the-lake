use board::domain::board::Board;
use cards::domain::card::Cards;
use flags::controllers::flags_controller::FlagsController;
use game_definitions::event_types::EventTypes;
use game_definitions::factions::Factions;
use players::domain::decision::Decision;
use players::domain::events::is_current_non_capability_event_effective::is_current_non_capability_event_effective;
use players::domain::player_type::PlayerType;
use sequence_of_play::controllers::sequence_of_play_controller::SequenceOfPlayController;
use std::collections::HashMap;

#[allow(clippy::too_many_arguments)]
pub fn whether_to_play_regular_event(
    faction: &Factions,
    preferred_event_type: EventTypes,
    active_card: &Cards,
    preview_card: &Cards,
    player_types: HashMap<Factions, PlayerType>,
    board: &Board,
    flags_controller: &FlagsController,
    sequence_of_play_controller: &SequenceOfPlayController,
) -> Result<Option<Decision>, String> {
    if is_current_non_capability_event_effective(
        &active_card,
        &preview_card,
        player_types,
        faction,
        preferred_event_type,
        board,
        flags_controller,
        sequence_of_play_controller,
    )? {
        panic!("Was going to play the card for the event.");
    }

    Ok(None)
}
