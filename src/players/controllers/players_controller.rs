use board::domain::board::Board;
use game_definitions::factions::Factions;
use players::domain::decision::Decision;
use players::domain::dummy_player::DummyPlayer;
use players::domain::human_us_player::HumanUsPlayer;
use players::domain::player::Player;
use players::domain::player::Players;
use user_interface::controllers::user_interface_controller::UserInterfaceController;

pub struct PlayersController {
    us_player: Players,
    arvn_player: Players,
    nva_player: Players,
    vc_player: Players,
}

impl Default for PlayersController {
    fn default() -> Self {
        Self::new()
    }
}

impl PlayersController {
    pub fn new() -> PlayersController {
        PlayersController {
            us_player: HumanUsPlayer::new().into(),
            arvn_player: DummyPlayer::new().into(),
            nva_player: DummyPlayer::new().into(),
            vc_player: DummyPlayer::new().into(),
        }
    }

    pub fn decide(
        &mut self,
        faction: &Factions,
        active_card: u8,
        preview_card: u8,
        possible_actions: Vec<String>,
        board: &Board,
        user_interface_controller: &UserInterfaceController,
    ) -> Result<Decision, String> {
        // Depending on the faction that has to decide and is passed as an argument,
        // this delegates asking the appropriate stored player.
        if faction == &Factions::US {
            let decide_result = self.us_player.decide(
                active_card,
                preview_card,
                *faction,
                possible_actions,
                board,
                user_interface_controller,
            );

            return Ok(decide_result?);
        } else if faction == &Factions::ARVN {
            let decide_result = self.arvn_player.decide(
                active_card,
                preview_card,
                *faction,
                possible_actions,
                board,
                user_interface_controller,
            );

            return Ok(decide_result?);
        } else if faction == &Factions::NVA {
            let decide_result = self.nva_player.decide(
                active_card,
                preview_card,
                *faction,
                possible_actions,
                board,
                user_interface_controller,
            );

            return Ok(decide_result?);
        } else if faction == &Factions::VC {
            let decide_result = self.vc_player.decide(
                active_card,
                preview_card,
                *faction,
                possible_actions,
                board,
                user_interface_controller,
            );

            return Ok(decide_result?);
        }

        panic!("Shouldn't have gotten here!");
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
}
