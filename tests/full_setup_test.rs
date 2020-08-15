extern crate fire_in_the_lake;
use fire_in_the_lake::board::controllers::setup_controller::SetupController;
use fire_in_the_lake::game_definitions::faction_stats::FactionStats;
use fire_in_the_lake::game_definitions::forces::Forces;
use fire_in_the_lake::game_definitions::space_identifiers::SpaceIdentifiers;

#[test]
fn running_full_setup_should_end_up_with_a_board_in_the_correct_state() -> Result<(), String> {
    let sut = SetupController::new();

    let (created_board, _instructions) = sut.setup_full()?;

    assert_eq!(created_board.get_faction_stat(FactionStats::Aid)?, 15);
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

    // Check forces in out of play and spaces
    assert_eq!(created_board.get_forces_in_out_of_play(Forces::UsBase)?, 2);
    assert_eq!(
        created_board.get_forces_in_out_of_play(Forces::UsTroop)?,
        10
    );
    assert_eq!(
        created_board.get_forces_in_out_of_play(Forces::ArvnBase)?,
        2
    );
    assert_eq!(
        created_board.get_forces_in_out_of_play(Forces::ArvnTroop)?,
        10
    );
    assert_eq!(
        created_board.get_forces_in_out_of_play(Forces::UndergroundArvnRanger)?,
        3
    );

    assert_eq!(
        created_board.get_forces_in_space(Forces::UsBase, SpaceIdentifiers::Saigon)?,
        1
    );
    assert_eq!(
        created_board.get_forces_in_space(Forces::UsTroop, SpaceIdentifiers::Saigon)?,
        2
    );
    assert_eq!(
        created_board.get_forces_in_space(Forces::ArvnTroop, SpaceIdentifiers::Saigon)?,
        2
    );
    assert_eq!(
        created_board.get_forces_in_space(Forces::ArvnPolice, SpaceIdentifiers::Saigon)?,
        3
    );
    assert_eq!(
        created_board.get_forces_in_space(Forces::ArvnTroop, SpaceIdentifiers::Hue)?,
        2
    );
    assert_eq!(
        created_board.get_forces_in_space(Forces::ArvnPolice, SpaceIdentifiers::Hue)?,
        2
    );

    assert_eq!(
        created_board.get_forces_in_space(Forces::ArvnTroop, SpaceIdentifiers::QuiNhon)?,
        2
    );
    assert_eq!(
        created_board.get_forces_in_space(Forces::ArvnPolice, SpaceIdentifiers::QuiNhon)?,
        2
    );
    assert_eq!(
        created_board.get_forces_in_space(Forces::ArvnTroop, SpaceIdentifiers::CamRanh)?,
        2
    );
    assert_eq!(
        created_board.get_forces_in_space(Forces::ArvnPolice, SpaceIdentifiers::CamRanh)?,
        2
    );
    assert_eq!(
        created_board.get_forces_in_space(Forces::ArvnTroop, SpaceIdentifiers::AnLoc)?,
        2
    );
    assert_eq!(
        created_board.get_forces_in_space(Forces::ArvnPolice, SpaceIdentifiers::AnLoc)?,
        2
    );
    assert_eq!(
        created_board.get_forces_in_space(Forces::ArvnTroop, SpaceIdentifiers::CanTho)?,
        2
    );
    assert_eq!(
        created_board.get_forces_in_space(Forces::ArvnPolice, SpaceIdentifiers::CanTho)?,
        2
    );

    assert_eq!(
        created_board.get_forces_in_space(Forces::UsTroop, SpaceIdentifiers::DaNang)?,
        2
    );
    assert_eq!(
        created_board.get_forces_in_space(Forces::ArvnPolice, SpaceIdentifiers::DaNang)?,
        1
    );
    assert_eq!(
        created_board.get_forces_in_space(Forces::UsTroop, SpaceIdentifiers::Kontum)?,
        2
    );
    assert_eq!(
        created_board.get_forces_in_space(Forces::ArvnPolice, SpaceIdentifiers::Kontum)?,
        1
    );

    assert_eq!(
        created_board.get_forces_in_space(
            Forces::UndergroundUsIrregular,
            SpaceIdentifiers::QuangTriThuaThien
        )?,
        1
    );
    assert_eq!(
        created_board.get_forces_in_space(Forces::UsTroop, SpaceIdentifiers::QuangTriThuaThien)?,
        1
    );
    assert_eq!(
        created_board.get_forces_in_space(Forces::VcBase, SpaceIdentifiers::QuangTriThuaThien)?,
        1
    );
    assert_eq!(
        created_board.get_forces_in_space(
            Forces::UndergroundVcGuerrilla,
            SpaceIdentifiers::QuangTriThuaThien
        )?,
        2
    );
    assert_eq!(
        created_board
            .get_forces_in_space(Forces::UndergroundUsIrregular, SpaceIdentifiers::BinhDinh)?,
        1
    );
    assert_eq!(
        created_board.get_forces_in_space(Forces::UsTroop, SpaceIdentifiers::BinhDinh)?,
        1
    );
    assert_eq!(
        created_board.get_forces_in_space(Forces::VcBase, SpaceIdentifiers::BinhDinh)?,
        1
    );
    assert_eq!(
        created_board
            .get_forces_in_space(Forces::UndergroundVcGuerrilla, SpaceIdentifiers::BinhDinh)?,
        2
    );

    assert_eq!(
        created_board
            .get_forces_in_space(Forces::UndergroundArvnRanger, SpaceIdentifiers::QuangNam)?,
        1
    );
    assert_eq!(
        created_board.get_forces_in_space(Forces::ArvnPolice, SpaceIdentifiers::QuangNam)?,
        1
    );

    assert_eq!(
        created_board.get_forces_in_space(Forces::UsBase, SpaceIdentifiers::PleikuDarlac)?,
        1
    );
    assert_eq!(
        created_board.get_forces_in_space(
            Forces::UndergroundUsIrregular,
            SpaceIdentifiers::PleikuDarlac
        )?,
        1
    );
    assert_eq!(
        created_board.get_forces_in_space(Forces::UsTroop, SpaceIdentifiers::PleikuDarlac)?,
        1
    );
    assert_eq!(
        created_board.get_forces_in_space(Forces::VcBase, SpaceIdentifiers::PleikuDarlac)?,
        1
    );
    assert_eq!(
        created_board.get_forces_in_space(
            Forces::UndergroundVcGuerrilla,
            SpaceIdentifiers::PleikuDarlac
        )?,
        2
    );

    assert_eq!(
        created_board.get_forces_in_space(Forces::VcBase, SpaceIdentifiers::QuangTinQuangNgai)?,
        1
    );
    assert_eq!(
        created_board.get_forces_in_space(
            Forces::UndergroundVcGuerrilla,
            SpaceIdentifiers::QuangTinQuangNgai
        )?,
        2
    );
    assert_eq!(
        created_board.get_forces_in_space(Forces::VcBase, SpaceIdentifiers::QuangDucLongKhanh)?,
        1
    );
    assert_eq!(
        created_board.get_forces_in_space(
            Forces::UndergroundVcGuerrilla,
            SpaceIdentifiers::QuangDucLongKhanh
        )?,
        2
    );
    assert_eq!(
        created_board.get_forces_in_space(Forces::VcBase, SpaceIdentifiers::BinhTuyBinhThuan)?,
        1
    );
    assert_eq!(
        created_board.get_forces_in_space(
            Forces::UndergroundVcGuerrilla,
            SpaceIdentifiers::BinhTuyBinhThuan
        )?,
        2
    );

    assert_eq!(
        created_board.get_forces_in_space(Forces::TunneledVcBase, SpaceIdentifiers::TayNinh)?,
        1
    );
    assert_eq!(
        created_board
            .get_forces_in_space(Forces::UndergroundVcGuerrilla, SpaceIdentifiers::TayNinh)?,
        2
    );

    assert_eq!(
        created_board.get_forces_in_space(Forces::ArvnPolice, SpaceIdentifiers::PhuBonPhuYen)?,
        1
    );
    assert_eq!(
        created_board.get_forces_in_space(Forces::ArvnPolice, SpaceIdentifiers::KhanhHoa)?,
        1
    );
    assert_eq!(
        created_board.get_forces_in_space(Forces::ArvnPolice, SpaceIdentifiers::KienHoaVinhBinh)?,
        1
    );
    assert_eq!(
        created_board.get_forces_in_space(Forces::ArvnPolice, SpaceIdentifiers::BaXuyen)?,
        1
    );

    assert_eq!(
        created_board
            .get_forces_in_space(Forces::UndergroundVcGuerrilla, SpaceIdentifiers::KienPhong)?,
        1
    );
    assert_eq!(
        created_board.get_forces_in_space(
            Forces::UndergroundVcGuerrilla,
            SpaceIdentifiers::KienGiangAnXuyen
        )?,
        1
    );

    assert_eq!(
        created_board.get_forces_in_space(Forces::NvaBase, SpaceIdentifiers::NorthVietnam)?,
        1
    );
    assert_eq!(
        created_board.get_forces_in_space(
            Forces::UndergroundNvaGuerrilla,
            SpaceIdentifiers::NorthVietnam
        )?,
        3
    );
    assert_eq!(
        created_board.get_forces_in_space(Forces::NvaBase, SpaceIdentifiers::CentralLaos)?,
        1
    );
    assert_eq!(
        created_board.get_forces_in_space(
            Forces::UndergroundNvaGuerrilla,
            SpaceIdentifiers::CentralLaos
        )?,
        3
    );
    assert_eq!(
        created_board.get_forces_in_space(Forces::NvaBase, SpaceIdentifiers::SouthernLaos)?,
        1
    );
    assert_eq!(
        created_board.get_forces_in_space(
            Forces::UndergroundNvaGuerrilla,
            SpaceIdentifiers::SouthernLaos
        )?,
        3
    );
    assert_eq!(
        created_board.get_forces_in_space(Forces::NvaBase, SpaceIdentifiers::TheParrotsBeak)?,
        1
    );
    assert_eq!(
        created_board.get_forces_in_space(
            Forces::UndergroundNvaGuerrilla,
            SpaceIdentifiers::TheParrotsBeak
        )?,
        3
    );

    Ok(())
}
