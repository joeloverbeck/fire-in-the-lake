use board::available_forces::AvailableForces;
use board::map::Map;
use board::track::Track;
use decision_making::choices::Choices;
use decision_making::commands_producer::CommandsProducer;
use decision_making::decision::Decision;
use decision_making::interpretation::interpret_commands::interpret_commands;
use decision_making::player::Player;
use decision_making::player::Players;
use factions::Factions;

pub struct DecisionMakingCenter {
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
                let interpreted_intentions =
                    interpret_commands(self.vc_player.provide_commands(active_card, map, track))?;

                // The player could have written 'event'. In that case we need to execute just the corresponding event,
                // which would be the shaded one in VC's case.
                if interpreted_intentions.does_it_want_to_activate_the_event() {
                    // VC player intended to execute the event. We will return the appropriate decision, along with
                    // with the written commands.
                    return Ok(Decision::new(
                        current_eligible,
                        Choices::ShadedEvent,
                        interpreted_intentions,
                    ));
                } else if interpreted_intentions.does_it_want_to_do_an_operation() {
                    return Ok(Decision::new(
                        current_eligible,
                        Choices::SecondOperationAndSpecialActivity,
                        interpreted_intentions,
                    ));
                }

                todo!()
            }
            Factions::NVA => {
                let interpreted_intentions =
                    interpret_commands(self.nva_player.provide_commands(active_card, map, track))?;

                // Could be that it has chosen to pass
                if interpreted_intentions.does_it_want_to_pass() {
                    return Ok(Decision::new(
                        current_eligible,
                        Choices::Pass,
                        interpreted_intentions,
                    ));
                }

                if interpreted_intentions.does_it_want_to_do_an_operation_only() {
                    // NVA wants to do an op only.
                    return Ok(Decision::new(
                        current_eligible,
                        Choices::OperationOnly,
                        interpreted_intentions,
                    ));
                }

                todo!()
            }
            Factions::ARVN => {
                let interpreted_intentions =
                    interpret_commands(self.arvn_player.provide_commands(active_card, map, track))?;

                if interpreted_intentions.does_it_want_to_do_an_operation() {
                    // Could play first the operation and then the special activity.
                    return Ok(Decision::new(
                        current_eligible,
                        Choices::SecondOperationAndSpecialActivity,
                        interpreted_intentions,
                    ));
                } else if interpreted_intentions.does_it_want_to_activate_the_event() {
                    return Ok(Decision::new(
                        current_eligible,
                        Choices::UnshadedEvent,
                        interpreted_intentions,
                    ));
                }

                todo!()
            }
            Factions::US => {
                let interpreted_intentions =
                    interpret_commands(self.us_player.provide_commands(active_card, map, track))?;

                if interpreted_intentions.does_it_want_to_sweep() {
                    // It's a special activity.
                    Ok(Decision::new(
                        current_eligible,
                        Choices::SecondLimitedOperation,
                        interpreted_intentions,
                    ))
                } else {
                    todo!()
                }
            }
            Factions::None => todo!(),
        }
    }
}
