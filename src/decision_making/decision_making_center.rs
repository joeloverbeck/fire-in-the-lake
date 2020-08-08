use board::map::Map;
use board::track::Track;
use decision_making::choices::Choices;
use decision_making::commands_producer::CommandsProducer;
use decision_making::decision::Decision;
use decision_making::player::Player;
use decision_making::player::Players;
use factions::Factions;

pub struct DecisionMakingCenter {
    vc_player: Players,
    _nva_player: Players,
    _us_player: Players,
    _arvn_player: Players,
}

impl DecisionMakingCenter {
    pub fn new(
        vc_player: Players,
        _nva_player: Players,
        _us_player: Players,
        _arvn_player: Players,
    ) -> DecisionMakingCenter {
        DecisionMakingCenter {
            vc_player,
            _nva_player,
            _us_player,
            _arvn_player,
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
    ) -> Decision {
        // From the current elegible we know to which held player we need to derive the decision to.
        // The players will get all this stuff, and they will send back whether they want to play it for the
        // event, or do other stuff.

        // Note: the logic here is to focus on the players *just* sending back the equivalent of written commands,
        // whether they are AI or human. And here or in a delegate class we'll do all the automatic stuff
        // that happens and that the player wouldn't have to type.

        match current_eligible {
            Factions::VC => {
                let player_commands = self.vc_player.provide_command(active_card, map, track);

                // The player could have written 'event'. In that case we need to execute just the corresponding event,
                // which would be the shaded one in VC's case.
                if player_commands.len() == 1 && player_commands[0] == "event" {
                    // VC player intended to execute the event. We will return the appropriate decision, along with
                    // with the written commands.
                    return Decision::new(current_eligible, Choices::ShadedEvent, player_commands);
                }

                todo!()
            }
            _ => todo!(),
        }
    }
}
