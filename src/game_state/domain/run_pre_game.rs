use cards::controllers::cards_controller::CardsController;
use game_state::domain::reveal_and_get_preview_card::reveal_and_get_preview_card;
use user_interface::controllers::display_controller::DisplayController;
use user_interface::controllers::keyboard_input_controller::KeyboardInputController;

pub fn run_pre_game(
    cards_controller: &mut CardsController,
    display_controller: &DisplayController,
    keyboard_input_controller: &KeyboardInputController,
) -> Result<(), String> {
    display_controller.write_announcement("Game start")?;

    display_controller.write_instruction(
        "Reveal the top card of the draw deck and place it onto a played cards pile.",
    )?;

    let active_card_in_text = keyboard_input_controller
        .request_player_input("What is the number of the card on the played card stack?: ")?;

    cards_controller.set_active_card(active_card_in_text.parse::<u8>().unwrap())?;

    cards_controller.set_preview_card(reveal_and_get_preview_card(
        &display_controller,
        &keyboard_input_controller,
    )?)?;

    Ok(())
}
