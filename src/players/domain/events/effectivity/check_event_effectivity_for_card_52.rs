use flags::controllers::flags_controller::FlagsController;

use board::domain::board::Board;
use cards::domain::card::Cards;
use game_definitions::event_types::EventTypes;
use game_definitions::factions::Factions;
use players::domain::player_type::PlayerType;
use std::collections::HashMap;

pub fn check_event_effectivity_for_card_52(
    _active_card: &Cards,
    _preview_card: &Cards,
    player_types: HashMap<Factions, PlayerType>,
    faction: &Factions,
    _preferible_event_type: EventTypes,
    _board: &Board,
    flags_controller: &FlagsController,
) -> Result<bool, String> {
    // Grey rifle for ARVN
    if faction == &Factions::ARVN && player_types.get(&faction).unwrap() == &PlayerType::Ai {
        return Ok(false);
    }

    // Shaded: Flip 1 unshaded US Capability to shaded.
    // Obviously there need to be US Capabilities active for this to be playable.
    if faction == &Factions::NVA || faction == &Factions::VC {
        return Ok(flags_controller.has_unshaded_us_capabilities()?);
    }

    panic!("Not handled for US");
}
