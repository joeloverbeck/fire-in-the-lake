use game_definitions::factions::Factions;
use sequence_of_play::controllers::sequence_of_play_controller::SequenceOfPlayController;

pub fn was_executing_faction_one_of_main_eligible(
    faction: &Factions,
    sequence_of_play_controller: &SequenceOfPlayController,
) -> Result<bool, String> {
    Ok(
        (sequence_of_play_controller.is_there_a_first_eligible_faction()?
            && sequence_of_play_controller.get_first_eligible()? == *faction)
            || (sequence_of_play_controller.is_there_a_second_eligible_faction()?
                && sequence_of_play_controller.get_second_eligible()? == *faction),
    )
}
