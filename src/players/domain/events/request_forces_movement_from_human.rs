use board::domain::board::Board;
use board::domain::translate_typed_input_to_space_identifier::translate_typed_input_to_space_identifier;
use game_definitions::forces::Forces;
use game_definitions::space_identifiers::SpaceIdentifiers;
use players::domain::decision::Decision;
use players::domain::forces_mutation::ForcesMutation;
use players::domain::mutation_types::MutationTypes;
use user_interface::controllers::user_interface_controller::UserInterfaceController;

pub fn request_forces_movement_from_human(
    forces: Forces,
    destination: SpaceIdentifiers,
    mut limit_of_forces_to_move: u8,
    limit_for_particular_space: Option<(SpaceIdentifiers, u8)>,
    board: &Board,
    user_interface_controller: &UserInterfaceController,
) -> Result<Decision, String> {
    let mut forces_mutations: Vec<ForcesMutation> = Vec::new();

    loop {
        if limit_of_forces_to_move == 0 {
            break;
        }

        let user_typed_space = user_interface_controller.request_player_input(
            "Where do you want to move your troops from? (or else write 'done'): ",
        )?;

        if user_typed_space == "done" {
            break;
        }

        let space_identifier =
            translate_typed_input_to_space_identifier(user_typed_space.as_str())?;

        // To help the player (and the system itself), get the number of those forces in the chosen location.
        let number_of_those_forces_in_location =
            board.get_forces_in_space(forces, space_identifier)?;

        let number = user_interface_controller.request_player_input(
            format!(
                "How many {} do you want to move from {}? (has {}): ",
                forces, space_identifier, number_of_those_forces_in_location
            )
            .as_str(),
        )?;

        let parsed_number = number.parse::<u8>().unwrap();

        if let Some((special_space, special_limit)) = limit_for_particular_space {
            if space_identifier == special_space && parsed_number > special_limit {
                println!(
                    " Can only move {:?} from {:?} for this case.",
                    special_limit, space_identifier
                );
                continue;
            }
        }
        if parsed_number > limit_of_forces_to_move {
            println!(
                " Can only move {} more forces of that type!",
                limit_of_forces_to_move
            );
            continue;
        }
        if parsed_number > number_of_those_forces_in_location {
            println!(" Can't get so many forces from the space!");
            continue;
        }

        limit_of_forces_to_move -= parsed_number;

        push_mutation_for_interpreted_case(
            &forces,
            parsed_number,
            &space_identifier,
            &destination,
            &mut forces_mutations,
        )?;
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
