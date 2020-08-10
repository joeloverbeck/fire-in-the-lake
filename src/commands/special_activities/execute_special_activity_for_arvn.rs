use board::current_rvn_leader::CurrentRvnLeader;
use board::map::Map;
use board::space::Space;
use board::space_identifiers::translate_space_name_to_identifier;
use board::track::Track;
use commands::manipulate_aid::manipulate_aid;
use decision_making::input_commands::InputCommands;

pub fn execute_special_activity_for_arvn(
    commands: Vec<InputCommands>,
    map: &mut Map,
    track: &mut Track,
) -> Result<(), String> {
    if commands[0] == InputCommands::Govern {
        let mut index = 1;

        while commands[index] != InputCommands::Stop {
            // commands[index] should contain the name of the place where you want to Govern.
            // Govern increases Aid. +3 * Space's Pop.

            // retrieve the corresponding location
            let space_identifier = translate_space_name_to_identifier(commands[index]);

            let retrieved_space = map.get_space(space_identifier).unwrap();

            manipulate_aid(track, (retrieved_space.get_population_value() * 3) as i8)?;

            index += 1;
        }

        // Additionally, the current ARVN leader might have a bonus for Govern.
        if map.get_current_rvn_leader() == CurrentRvnLeader::Minh {
            // ARVN receives +5 Aid every time it trains
            manipulate_aid(track, 5)?;
        }
    } else {
        panic!(
            "While trying to execute a special activity for arvn, hadn't predicted this: {:?}",
            commands[0]
        );
    }

    Ok(())
}
