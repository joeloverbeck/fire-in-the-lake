extern crate fire_in_the_lake;
use fire_in_the_lake::board::controllers::setup_controller::SetupController;
use fire_in_the_lake::game_definitions::faction_stats::FactionStats;

#[test]
fn running_full_setup_should_end_up_with_a_board_in_the_correct_state() -> Result<(), String> {
    let sut = SetupController::new();

    let (created_board, _instructions) = sut.setup_full()?;

    let aid_result = created_board.get_faction_stat(FactionStats::Aid);

    if let Err(error) = aid_result {
        panic!("Failed because {:?}", error);
    } else if let Ok(aid) = aid_result {
        assert_eq!(aid, 15, "Hey!");
    }

    assert_eq!(created_board.get_faction_stat(FactionStats::TotalEcon)?, 15);
    assert_eq!(created_board.get_faction_stat(FactionStats::Patronage)?, 15);
    assert_eq!(
        created_board.get_faction_stat(FactionStats::VcResources)?,
        5
    );
    assert_eq!(
        created_board.get_faction_stat(FactionStats::NvaResources)?,
        10
    );
    assert_eq!(
        created_board.get_faction_stat(FactionStats::ArvnResources)?,
        30
    );
    assert_eq!(
        created_board.get_faction_stat(FactionStats::SupportPlusAvailable)?,
        38
    );
    assert_eq!(
        created_board.get_faction_stat(FactionStats::CoinPlusPatronage)?,
        35
    );
    assert_eq!(
        created_board.get_faction_stat(FactionStats::OppositionPlusBases)?,
        27
    );
    assert_eq!(
        created_board.get_faction_stat(FactionStats::NvaPlusBases)?,
        4
    );
    assert_eq!(created_board.get_faction_stat(FactionStats::TheTrail)?, 1);

    Ok(())
}
