use board::domain::board::Board;
use board::domain::translate_typed_input_to_space_identifier::translate_typed_input_to_space_identifier;
use game_definitions::forces::Forces;
use game_definitions::space_identifiers::SpaceIdentifiers;
use players::domain::decision::Decision;
use players::domain::forces_mutation::ForcesMutation;
use players::domain::mutation_types::MutationTypes;
use user_interface::controllers::display_controller::DisplayController;
use user_interface::controllers::keyboard_input_controller::KeyboardInputController;
use user_interface::domain::input_violation_types::InputViolationTypes;

pub fn request_forces_movement_from_human(
    forces: Forces,
    destination: SpaceIdentifiers,
    mut limit_of_forces_to_move: u8,
    limit_for_particular_space: Option<(SpaceIdentifiers, u8)>,
    board: &Board,
    keyboard_input_controller: &KeyboardInputController,
    display_controller: &DisplayController,
) -> Result<Decision, String> {
    let mut forces_mutations: Vec<ForcesMutation> = Vec::new();

    loop {
        if limit_of_forces_to_move == 0 {
            break;
        }

        let (user_typed_space, wants_out) = keyboard_input_controller
            .request_player_input_configurable(
                "Where do you want to move your troops from? (or else write 'done'): ",
                Some("done"),
            )?;

        if wants_out {
            break;
        }

        let space_identifier =
            translate_typed_input_to_space_identifier(user_typed_space.as_str())?;

        // To help the player (and the system itself), get the number of those forces in the chosen location.
        let number_of_those_forces_in_location =
            board.get_forces_in_space(forces, space_identifier)?;

        let mut input_violations: Vec<(u8, InputViolationTypes, String)> = Vec::new();

        if let Some((special_space, special_limit)) = limit_for_particular_space {
            if space_identifier == special_space {
                input_violations.push((
                    special_limit,
                    InputViolationTypes::Bigger,
                    format!(
                        "Can only move {} from {} for this case",
                        special_limit, space_identifier
                    ),
                ));
            }
        }

        input_violations.push((
            limit_of_forces_to_move,
            InputViolationTypes::Bigger,
            format!(
                "Can only move {} more forces of that type!",
                limit_of_forces_to_move
            ),
        ));

        input_violations.push((
            number_of_those_forces_in_location,
            InputViolationTypes::Bigger,
            "Can't get so many forces from the space!".to_string(),
        ));

        let numeric_request_result = keyboard_input_controller
            .request_numeric_player_input_configurable(
                format!(
                    "How many {} do you want to move from {}? (has {}): ",
                    forces, space_identifier, number_of_those_forces_in_location
                )
                .as_str(),
                None,
                Some(input_violations),
                &display_controller,
            );

        if let Ok((number, _)) = numeric_request_result {
            limit_of_forces_to_move -= number;

            push_mutation_for_interpreted_case(
                &forces,
                number,
                &space_identifier,
                &destination,
                &mut forces_mutations,
            )?;
        } else if numeric_request_result.is_err() {
            continue;
        }
    }

    Ok(Decision::new(
        Vec::new(),
        Vec::new(),
        forces_mutations,
        Vec::new(),
    ))
}

fn push_mutation_for_interpreted_case(
    forces: &Forces,
    number_to_move: u8,
    from: &SpaceIdentifiers,
    to: &SpaceIdentifiers,
    forces_mutations: &mut Vec<ForcesMutation>,
) -> Result<(), String> {
    forces_mutations.push(ForcesMutation::new(
        *forces,
        MutationTypes::Move,
        number_to_move,
        Some(*from),
        Some(*to),
    ));

    Ok(())
}
