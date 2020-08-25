use board::domain::queries::board_level_queries::is_there_a_specific_force_anywhere::is_there_a_specific_force_anywhere;

use board::domain::board::Board;
use cards::domain::card::Cards;
use flags::controllers::flags_controller::FlagsController;
use game_definitions::event_types::EventTypes;
use game_definitions::factions::Factions;
use game_definitions::forces::Forces;
use players::domain::player_type::PlayerType;
use std::collections::HashMap;

pub fn check_event_effectivity_for_card_110(
    _active_card: &Cards,
    _preview_card: &Cards,
    player_types: HashMap<Factions, PlayerType>,
    faction: &Factions,
    _preferible_event_type: EventTypes,
    board: &Board,
    flags_controller: &FlagsController,
) -> Result<bool, String> {
    // All factions have special intructions except for the US.

    if (faction == &Factions::NVA || faction == &Factions::VC)
        && player_types.get(&faction).unwrap() == &PlayerType::Ai
    {
        // Shaded: Flip all VC and NVA Guerrillas Underground.
        // NVA & VC: If Monsoon or no Guerrillas of own Faction are Active, choose Op & Special Activity
        if flags_controller.is_monsoon()? {
            return Ok(false);
        }

        if faction == &Factions::NVA {
            // There must be active nva guerrillas anywhere.
            return Ok(is_there_a_specific_force_anywhere(
                Forces::ActiveNvaGuerrilla,
                &board,
            )?);
        }

        if faction == &Factions::VC {
            return Ok(is_there_a_specific_force_anywhere(
                Forces::ActiveVcGuerrilla,
                &board,
            )?);
        }
    }

    todo!()
}
