use board::map::Map;
use board::track::Track;
use decision_making::testing::playbook_first_turn_arvn::PlaybookFirstTurnArvn;
use decision_making::testing::playbook_first_turn_nva::PlaybookFirstTurnNva;
use decision_making::testing::playbook_first_turn_vc::PlaybookFirstTurnVc;
use decision_making::testing::playbook_second_turn_nva::PlaybookSecondTurnNva;
use decision_making::testing::playbook_second_turn_us::PlaybookSecondTurnUs;
use decision_making::testing::playbook_third_turn_arvn::PlaybookThirdTurnArvn;
use decision_making::testing::playbook_third_turn_vc::PlaybookThirdTurnVc;

extern crate enum_dispatch;
use self::enum_dispatch::enum_dispatch;

#[enum_dispatch]
pub trait Player {
    fn provide_commands(&self, active_card: u8, map: &Map, track: &Track) -> Vec<String>;
}

#[enum_dispatch(Player)]
#[derive(Debug)]
pub enum Players {
    PlaybookFirstTurnVc,
    PlaybookFirstTurnNva,
    PlaybookFirstTurnArvn,
    PlaybookSecondTurnNva,
    PlaybookSecondTurnUs,
    PlaybookThirdTurnArvn,
    PlaybookThirdTurnVc,
}
