use board::domain::board::Board;
use cards::domain::card::Cards;
use flags::controllers::flags_controller::FlagsController;
use game_definitions::factions::Factions;
use players::domain::ai_nva_player::AiNvaPlayer;
use players::domain::decision::Decision;
use players::domain::dummy_player::DummyPlayer;
use players::domain::human_us_player::HumanUsPlayer;
use players::domain::player::Player;
use players::domain::player::Players;
use players::domain::player_type::PlayerType;
use randomization::controllers::randomization_controller::RandomizationController;
use sequence_of_play::controllers::sequence_of_play_controller::SequenceOfPlayController;
use std::collections::HashMap;
use user_interface::controllers::display_controller::DisplayController;
use user_interface::controllers::keyboard_input_controller::KeyboardInputController;

pub struct PlayersController {
    us_player: Players,
    arvn_player: Players,
    nva_player: Players,
    vc_player: Players,
    randomization_controller: RandomizationController,
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
            nva_player: AiNvaPlayer::new().into(),
            vc_player: DummyPlayer::new().into(),
            randomization_controller: RandomizationController::new(),
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn decide(
        &mut self,
        faction: &Factions,
        active_card: &Cards,
        preview_card: &Cards,
        possible_actions: Vec<String>,
        board: &Board,
        flags_controller: &FlagsController,
        sequence_of_play_controller: &SequenceOfPlayController,
        keyboard_input_controller: &KeyboardInputController,
        display_controller: &DisplayController,
    ) -> Result<Decision, String> {
        // Depending on the faction that has to decide and is passed as an argument,
        // this delegates asking the appropriate stored player.
        let player_types: HashMap<Factions, PlayerType> = [
            (Factions::ARVN, self.arvn_player.get_player_type()?),
            (Factions::NVA, self.nva_player.get_player_type()?),
            (Factions::US, self.us_player.get_player_type()?),
            (Factions::VC, self.vc_player.get_player_type()?),
        ]
        .iter()
        .cloned()
        .collect();

        if faction == &Factions::US {
            let decide_result = self.us_player.decide(
                active_card,
                preview_card,
                *faction,
                player_types,
                possible_actions,
                board,
                flags_controller,
                sequence_of_play_controller,
                &self.randomization_controller,
                keyboard_input_controller,
                display_controller,
            );

            return Ok(decide_result?);
        } else if faction == &Factions::ARVN {
            let decide_result = self.arvn_player.decide(
                active_card,
                preview_card,
                *faction,
                player_types,
                possible_actions,
                board,
                flags_controller,
                sequence_of_play_controller,
                &self.randomization_controller,
                keyboard_input_controller,
                display_controller,
            );

            return Ok(decide_result?);
        } else if faction == &Factions::NVA {
            let decide_result = self.nva_player.decide(
                active_card,
                preview_card,
                *faction,
                player_types,
                possible_actions,
                board,
                flags_controller,
                sequence_of_play_controller,
                &self.randomization_controller,
                keyboard_input_controller,
                display_controller,
            );

            return Ok(decide_result?);
        } else if faction == &Factions::VC {
            let decide_result = self.vc_player.decide(
                active_card,
                preview_card,
                *faction,
                player_types,
                possible_actions,
                board,
                flags_controller,
                sequence_of_play_controller,
                &self.randomization_controller,
                keyboard_input_controller,
                display_controller,
            );

            return Ok(decide_result?);
        }

        panic!("Shouldn't have gotten here!");
    }
}
