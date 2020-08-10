use board::available_forces::AvailableForces;
use board::map::Map;
use board::track::Track;
use decision_making::choices::Choices;
use decision_making::commands_producer::CommandsProducer;
use decision_making::decision::Decision;
use decision_making::input_commands::InputCommands;
use decision_making::input_interpreter::InputInterpreter;
use decision_making::player::Player;
use decision_making::player::Players;
use factions::Factions;

pub struct DecisionMakingCenter {
    input_interpreter: InputInterpreter,
    vc_player: Players,
    nva_player: Players,
    us_player: Players,
    arvn_player: Players,
}

impl DecisionMakingCenter {
    pub fn new(
        vc_player: Players,
        nva_player: Players,
        us_player: Players,
        arvn_player: Players,
    ) -> DecisionMakingCenter {
        DecisionMakingCenter {
            input_interpreter: InputInterpreter::new(),
            vc_player,
            nva_player,
            us_player,
            arvn_player,
        }
    }
}

impl CommandsProducer for DecisionMakingCenter {
    fn decide(
        &self,
        active_card: u8,
        current_eligible: Factions,
        map: &Map,
        track: &Track,
        _available_forces: &AvailableForces,
    ) -> Result<Decision, String> {
        // From the current elegible we know to which held player we need to derive the decision to.
        // The players will get all this stuff, and they will send back whether they want to play it for the
        // event, or do other stuff.

        // Note: the logic here is to focus on the players *just* sending back the equivalent of written commands,
        // whether they are AI or human. And here or in a delegate class we'll do all the automatic stuff
        // that happens and that the player wouldn't have to type.

        match current_eligible {
            Factions::VC => {
                let player_commands = self
                    .input_interpreter
                    .interpret(self.vc_player.provide_commands(active_card, map, track))?;

                // The player could have written 'event'. In that case we need to execute just the corresponding event,
                // which would be the shaded one in VC's case.
                if player_commands.len() == 1 && player_commands[0] == InputCommands::Event {
                    // VC player intended to execute the event. We will return the appropriate decision, along with
                    // with the written commands.
                    return Ok(Decision::new(
                        current_eligible,
                        Choices::ShadedEvent,
                        player_commands,
                    ));
                }

                todo!()
            }
            Factions::NVA => {
                let player_commands = self
                    .input_interpreter
                    .interpret(self.nva_player.provide_commands(active_card, map, track))?;

                // Could be that it has chosen to pass
                if player_commands.len() == 1 && player_commands[0] == InputCommands::Pass {
                    return Ok(Decision::new(
                        current_eligible,
                        Choices::Pass,
                        player_commands,
                    ));
                }

                // Could have chosen to do an op only.
                //let possible_op_only_command = extract_multiword_command(&player_commands[0]);

                if player_commands[0] == InputCommands::OperationOnly {
                    // NVA wants to do an op only.
                    return Ok(Decision::new(
                        current_eligible,
                        Choices::OperationOnly,
                        player_commands,
                    ));
                }

                todo!()
            }
            Factions::ARVN => {
                let player_commands = self
                    .input_interpreter
                    .interpret(self.arvn_player.provide_commands(active_card, map, track))?;

                if player_commands[0] == InputCommands::Operation {
                    // Could play first the operation and then the special activity.
                    return Ok(Decision::new(
                        current_eligible,
                        Choices::SecondOperationAndSpecialActivity,
                        player_commands,
                    ));
                } else if player_commands[0] == InputCommands::Event {
                    return Ok(Decision::new(
                        current_eligible,
                        Choices::UnshadedEvent,
                        player_commands,
                    ));
                }

                todo!()
            }
            Factions::US => {
                let player_commands = self
                    .input_interpreter
                    .interpret(self.us_player.provide_commands(active_card, map, track))?;

                if player_commands[0] == InputCommands::Sweep {
                    // It's a special activity.
                    Ok(Decision::new(
                        current_eligible,
                        Choices::SecondLimitedOperation,
                        player_commands,
                    ))
                } else {
                    todo!()
                }
            }
            Factions::None => todo!(),
        }
    }
}
