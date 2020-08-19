use board::controllers::queries_controller::QueriesController;
use board::domain::board::Board;
use cards::domain::card::Cards;
use game_definitions::event_type::EventType;
use game_definitions::faction_groups::FactionGroups;
use game_definitions::factions::Factions;
use players::domain::player_type::PlayerType;
use std::collections::HashMap;

pub fn check_event_effectivity_for_card_48(
    _active_card: &Cards,
    _preview_card: &Cards,
    player_types: HashMap<Factions, PlayerType>,
    faction: &Factions,
    _preferible_event_type: EventType,
    board: &Board,
) -> Result<bool, String> {
    // Due to special instructions for NVA, if VC is a human player, this wouldn't be playable.
    if player_types.get(&faction).unwrap() == &PlayerType::Ai
        && player_types.get(&Factions::VC).unwrap() == &PlayerType::Human
    {
        panic!("Case not handled for VC being a human player while NVA being AI.");
    }

    // It's only effective in its unshaded part if there's a COIN base in a province with less or equal COIN "cubes" specifically (US to Casualties).
    if faction == &Factions::NVA || faction == &Factions::VC {
        let queries_controller = QueriesController::new();

        return Ok(queries_controller
            .is_there_a_faction_group_base_in_a_province_with_less_or_equal_cubes_of_that_group(
                &FactionGroups::Coin,
                2,
                board,
            )?);
    }

    panic!("Not implemented effectivity testing for card 48 if faction group is Coin.");
}
