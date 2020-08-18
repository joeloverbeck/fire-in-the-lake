use user_interface::controllers::display_controller::DisplayController;
use user_interface::controllers::keyboard_input_controller::KeyboardInputController;

pub fn reveal_and_get_preview_card(
    display_controller: &DisplayController,
    keyboard_input_controller: &KeyboardInputController,
) -> Result<u8, String> {
    display_controller.write_instruction("Reveal the next card on top of the draw deck.")?;

    let preview_card_in_text = keyboard_input_controller
        .request_player_input("What is the number of the card on top of the draw deck?: ")?;

    Ok(preview_card_in_text.parse::<u8>().unwrap())
}
