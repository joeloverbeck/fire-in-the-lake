use board::domain::board::Board;
use game_definitions::control_types::ControlTypes;
use game_definitions::faction_stats::FactionStats;
use game_definitions::forces::Forces;
use game_definitions::space_identifiers::SpaceIdentifiers;
use game_definitions::support_levels::SupportLevels;

pub struct SetupController {}

impl Default for SetupController {
    fn default() -> Self {
        Self::new()
    }
}

impl SetupController {
    pub fn new() -> SetupController {
        SetupController {}
    }

    pub fn setup_full(&self) -> Result<(Board, Vec<Vec<String>>), String> {
        let mut collection_of_instructions: Vec<Vec<String>> = Vec::new();

        let mut board = Board::new();

        // Set the full scenario's faction stats
        board.set_faction_stat(FactionStats::Aid, 15)?;
        board.set_faction_stat(FactionStats::TotalEcon, 15)?;
        board.set_faction_stat(FactionStats::Patronage, 15)?;
        board.set_faction_stat(FactionStats::VcResources, 5)?;
        board.set_faction_stat(FactionStats::NvaResources, 10)?;
        board.set_faction_stat(FactionStats::ArvnResources, 30)?;
        board.set_faction_stat(FactionStats::SupportPlusAvailable, 38)?;
        board.set_faction_stat(FactionStats::CoinPlusPatronage, 35)?;
        board.set_faction_stat(FactionStats::OppositionPlusBases, 27)?;
        board.set_faction_stat(FactionStats::NvaPlusBases, 4)?;
        board.set_faction_stat(FactionStats::TheTrail, 1)?;

        // Push the instructions to the player in order to put those markers
        let mut faction_stats_instructions: Vec<String> = Vec::new();

        faction_stats_instructions.push(format!("Set {} to 15.", FactionStats::Aid));
        faction_stats_instructions.push(format!("Set {} to 15.", FactionStats::TotalEcon));
        faction_stats_instructions.push(format!("Set {} to 15.", FactionStats::Patronage));
        faction_stats_instructions.push(format!("Set {} to 5.", FactionStats::VcResources));
        faction_stats_instructions.push(format!("Set {} to 10.", FactionStats::NvaResources));
        faction_stats_instructions.push(format!("Set {} to 30.", FactionStats::ArvnResources));
        faction_stats_instructions
            .push(format!("Set {} to 38.", FactionStats::SupportPlusAvailable));
        faction_stats_instructions.push(format!("Set {} to 35.", FactionStats::CoinPlusPatronage));
        faction_stats_instructions
            .push(format!("Set {} to 27.", FactionStats::OppositionPlusBases));
        faction_stats_instructions.push(format!("Set {} to 4.", FactionStats::NvaPlusBases));
        faction_stats_instructions.push(format!("Set {} to 1.", FactionStats::TheTrail));

        collection_of_instructions.push(faction_stats_instructions);

        // Set forces out of play
        board.set_forces_in_out_of_play(Forces::UsBase, 2)?;
        board.set_forces_in_out_of_play(Forces::UsTroop, 10)?;
        board.set_forces_in_out_of_play(Forces::ArvnBase, 2)?;
        board.set_forces_in_out_of_play(Forces::ArvnTroop, 10)?;
        board.set_forces_in_out_of_play(Forces::UndergroundArvnRanger, 3)?;

        // Push the instructions to the player in order to put those pieces in Out of Play
        let mut out_of_play_instructions: Vec<String> = Vec::new();

        out_of_play_instructions.push(format!(
            "Place {} {} in the Out of Play box.",
            2,
            Forces::UsBase
        ));
        out_of_play_instructions.push(format!(
            "Place {} {} in the Out of Play box.",
            10,
            Forces::UsTroop
        ));
        out_of_play_instructions.push(format!(
            "Place {} {} in the Out of Play box.",
            2,
            Forces::ArvnBase
        ));
        out_of_play_instructions.push(format!(
            "Place {} {} in the Out of Play box.",
            10,
            Forces::ArvnTroop
        ));
        out_of_play_instructions.push(format!(
            "Place {} {} in the Out of Play box.",
            3,
            Forces::UndergroundArvnRanger
        ));

        collection_of_instructions.push(out_of_play_instructions);

        // Set forces in spaces
        board.set_forces_in_space(Forces::UsBase, 1, SpaceIdentifiers::Saigon)?;
        board.set_forces_in_space(Forces::UsTroop, 2, SpaceIdentifiers::Saigon)?;
        board.set_forces_in_space(Forces::ArvnTroop, 2, SpaceIdentifiers::Saigon)?;
        board.set_forces_in_space(Forces::ArvnPolice, 3, SpaceIdentifiers::Saigon)?;

        board.set_forces_in_space(Forces::ArvnTroop, 2, SpaceIdentifiers::Hue)?;
        board.set_forces_in_space(Forces::ArvnPolice, 2, SpaceIdentifiers::Hue)?;

        board.set_forces_in_space(Forces::ArvnTroop, 2, SpaceIdentifiers::QuiNhon)?;
        board.set_forces_in_space(Forces::ArvnPolice, 2, SpaceIdentifiers::QuiNhon)?;
        board.set_forces_in_space(Forces::ArvnTroop, 2, SpaceIdentifiers::CamRanh)?;
        board.set_forces_in_space(Forces::ArvnPolice, 2, SpaceIdentifiers::CamRanh)?;
        board.set_forces_in_space(Forces::ArvnTroop, 2, SpaceIdentifiers::AnLoc)?;
        board.set_forces_in_space(Forces::ArvnPolice, 2, SpaceIdentifiers::AnLoc)?;
        board.set_forces_in_space(Forces::ArvnTroop, 2, SpaceIdentifiers::CanTho)?;
        board.set_forces_in_space(Forces::ArvnPolice, 2, SpaceIdentifiers::CanTho)?;

        board.set_forces_in_space(Forces::UsTroop, 2, SpaceIdentifiers::DaNang)?;
        board.set_forces_in_space(Forces::ArvnPolice, 1, SpaceIdentifiers::DaNang)?;
        board.set_forces_in_space(Forces::UsTroop, 2, SpaceIdentifiers::Kontum)?;
        board.set_forces_in_space(Forces::ArvnPolice, 1, SpaceIdentifiers::Kontum)?;

        board.set_forces_in_space(
            Forces::UndergroundUsIrregular,
            1,
            SpaceIdentifiers::QuangTriThuaThien,
        )?;
        board.set_forces_in_space(Forces::UsTroop, 1, SpaceIdentifiers::QuangTriThuaThien)?;
        board.set_forces_in_space(Forces::VcBase, 1, SpaceIdentifiers::QuangTriThuaThien)?;
        board.set_forces_in_space(
            Forces::UndergroundVcGuerrilla,
            2,
            SpaceIdentifiers::QuangTriThuaThien,
        )?;
        board.set_forces_in_space(
            Forces::UndergroundUsIrregular,
            1,
            SpaceIdentifiers::BinhDinh,
        )?;
        board.set_forces_in_space(Forces::UsTroop, 1, SpaceIdentifiers::BinhDinh)?;
        board.set_forces_in_space(Forces::VcBase, 1, SpaceIdentifiers::BinhDinh)?;
        board.set_forces_in_space(
            Forces::UndergroundVcGuerrilla,
            2,
            SpaceIdentifiers::BinhDinh,
        )?;

        board.set_forces_in_space(Forces::UndergroundArvnRanger, 1, SpaceIdentifiers::QuangNam)?;
        board.set_forces_in_space(Forces::ArvnPolice, 1, SpaceIdentifiers::QuangNam)?;

        board.set_forces_in_space(Forces::UsBase, 1, SpaceIdentifiers::PleikuDarlac)?;
        board.set_forces_in_space(
            Forces::UndergroundUsIrregular,
            1,
            SpaceIdentifiers::PleikuDarlac,
        )?;
        board.set_forces_in_space(Forces::UsTroop, 1, SpaceIdentifiers::PleikuDarlac)?;
        board.set_forces_in_space(Forces::VcBase, 1, SpaceIdentifiers::PleikuDarlac)?;
        board.set_forces_in_space(
            Forces::UndergroundVcGuerrilla,
            2,
            SpaceIdentifiers::PleikuDarlac,
        )?;

        board.set_forces_in_space(Forces::VcBase, 1, SpaceIdentifiers::QuangTinQuangNgai)?;
        board.set_forces_in_space(
            Forces::UndergroundVcGuerrilla,
            2,
            SpaceIdentifiers::QuangTinQuangNgai,
        )?;
        board.set_forces_in_space(Forces::VcBase, 1, SpaceIdentifiers::QuangDucLongKhanh)?;
        board.set_forces_in_space(
            Forces::UndergroundVcGuerrilla,
            2,
            SpaceIdentifiers::QuangDucLongKhanh,
        )?;
        board.set_forces_in_space(Forces::VcBase, 1, SpaceIdentifiers::BinhTuyBinhThuan)?;
        board.set_forces_in_space(
            Forces::UndergroundVcGuerrilla,
            2,
            SpaceIdentifiers::BinhTuyBinhThuan,
        )?;

        board.set_forces_in_space(Forces::TunneledVcBase, 1, SpaceIdentifiers::TayNinh)?;
        board.set_forces_in_space(Forces::UndergroundVcGuerrilla, 2, SpaceIdentifiers::TayNinh)?;

        board.set_forces_in_space(Forces::ArvnPolice, 1, SpaceIdentifiers::PhuBonPhuYen)?;
        board.set_forces_in_space(Forces::ArvnPolice, 1, SpaceIdentifiers::KhanhHoa)?;
        board.set_forces_in_space(Forces::ArvnPolice, 1, SpaceIdentifiers::KienHoaVinhBinh)?;
        board.set_forces_in_space(Forces::ArvnPolice, 1, SpaceIdentifiers::BaXuyen)?;

        board.set_forces_in_space(
            Forces::UndergroundVcGuerrilla,
            1,
            SpaceIdentifiers::KienPhong,
        )?;
        board.set_forces_in_space(
            Forces::UndergroundVcGuerrilla,
            1,
            SpaceIdentifiers::KienGiangAnXuyen,
        )?;

        board.set_forces_in_space(Forces::NvaBase, 1, SpaceIdentifiers::NorthVietnam)?;
        board.set_forces_in_space(
            Forces::UndergroundNvaGuerrilla,
            3,
            SpaceIdentifiers::NorthVietnam,
        )?;
        board.set_forces_in_space(Forces::NvaBase, 1, SpaceIdentifiers::CentralLaos)?;
        board.set_forces_in_space(
            Forces::UndergroundNvaGuerrilla,
            3,
            SpaceIdentifiers::CentralLaos,
        )?;
        board.set_forces_in_space(Forces::NvaBase, 1, SpaceIdentifiers::SouthernLaos)?;
        board.set_forces_in_space(
            Forces::UndergroundNvaGuerrilla,
            3,
            SpaceIdentifiers::SouthernLaos,
        )?;
        board.set_forces_in_space(Forces::NvaBase, 1, SpaceIdentifiers::TheParrotsBeak)?;
        board.set_forces_in_space(
            Forces::UndergroundNvaGuerrilla,
            3,
            SpaceIdentifiers::TheParrotsBeak,
        )?;

        // Push all the damn instructions about placing forces to the player
        let mut forces_in_spaces_instructions: Vec<String> = Vec::new();

        forces_in_spaces_instructions.push(format!(
            "Place {} {} in {}.",
            1,
            Forces::UsBase,
            SpaceIdentifiers::Saigon,
        ));
        forces_in_spaces_instructions.push(format!(
            "Place {} {} in {}.",
            2,
            Forces::UsTroop,
            SpaceIdentifiers::Saigon,
        ));
        forces_in_spaces_instructions.push(format!(
            "Place {} {} in {}.",
            2,
            Forces::ArvnTroop,
            SpaceIdentifiers::Saigon,
        ));
        forces_in_spaces_instructions.push(format!(
            "Place {} {} in {}.",
            3,
            Forces::ArvnPolice,
            SpaceIdentifiers::Saigon,
        ));

        forces_in_spaces_instructions.push(format!(
            "Place {} {} in {}.",
            2,
            Forces::ArvnTroop,
            SpaceIdentifiers::Hue,
        ));
        forces_in_spaces_instructions.push(format!(
            "Place {} {} in {}.",
            2,
            Forces::ArvnPolice,
            SpaceIdentifiers::Hue,
        ));

        forces_in_spaces_instructions.push(format!(
            "Place {} {} in {}.",
            2,
            Forces::ArvnTroop,
            SpaceIdentifiers::QuiNhon,
        ));
        forces_in_spaces_instructions.push(format!(
            "Place {} {} in {}.",
            2,
            Forces::ArvnPolice,
            SpaceIdentifiers::QuiNhon,
        ));
        forces_in_spaces_instructions.push(format!(
            "Place {} {} in {}.",
            2,
            Forces::ArvnTroop,
            SpaceIdentifiers::CamRanh,
        ));
        forces_in_spaces_instructions.push(format!(
            "Place {} {} in {}.",
            2,
            Forces::ArvnPolice,
            SpaceIdentifiers::CamRanh,
        ));
        forces_in_spaces_instructions.push(format!(
            "Place {} {} in {}.",
            2,
            Forces::ArvnTroop,
            SpaceIdentifiers::AnLoc,
        ));
        forces_in_spaces_instructions.push(format!(
            "Place {} {} in {}.",
            2,
            Forces::ArvnPolice,
            SpaceIdentifiers::AnLoc,
        ));
        forces_in_spaces_instructions.push(format!(
            "Place {} {} in {}.",
            2,
            Forces::ArvnTroop,
            SpaceIdentifiers::CanTho,
        ));
        forces_in_spaces_instructions.push(format!(
            "Place {} {} in {}.",
            2,
            Forces::ArvnPolice,
            SpaceIdentifiers::CanTho,
        ));

        forces_in_spaces_instructions.push(format!(
            "Place {} {} in {}.",
            2,
            Forces::UsTroop,
            SpaceIdentifiers::DaNang,
        ));
        forces_in_spaces_instructions.push(format!(
            "Place {} {} in {}.",
            1,
            Forces::ArvnPolice,
            SpaceIdentifiers::DaNang,
        ));
        forces_in_spaces_instructions.push(format!(
            "Place {} {} in {}.",
            2,
            Forces::UsTroop,
            SpaceIdentifiers::Kontum,
        ));
        forces_in_spaces_instructions.push(format!(
            "Place {} {} in {}.",
            1,
            Forces::ArvnPolice,
            SpaceIdentifiers::Kontum,
        ));

        forces_in_spaces_instructions.push(format!(
            "Place {} {} in {}.",
            1,
            Forces::UndergroundUsIrregular,
            SpaceIdentifiers::QuangTriThuaThien
        ));
        forces_in_spaces_instructions.push(format!(
            "Place {} {} in {}.",
            1,
            Forces::UsTroop,
            SpaceIdentifiers::QuangTriThuaThien
        ));
        forces_in_spaces_instructions.push(format!(
            "Place {} {} in {}.",
            1,
            Forces::VcBase,
            SpaceIdentifiers::QuangTriThuaThien
        ));
        forces_in_spaces_instructions.push(format!(
            "Place {} {} in {}.",
            2,
            Forces::UndergroundVcGuerrilla,
            SpaceIdentifiers::QuangTriThuaThien
        ));
        forces_in_spaces_instructions.push(format!(
            "Place {} {} in {}.",
            1,
            Forces::UndergroundUsIrregular,
            SpaceIdentifiers::BinhDinh
        ));
        forces_in_spaces_instructions.push(format!(
            "Place {} {} in {}.",
            1,
            Forces::UsTroop,
            SpaceIdentifiers::BinhDinh
        ));
        forces_in_spaces_instructions.push(format!(
            "Place {} {} in {}.",
            1,
            Forces::VcBase,
            SpaceIdentifiers::BinhDinh
        ));
        forces_in_spaces_instructions.push(format!(
            "Place {} {} in {}.",
            2,
            Forces::UndergroundVcGuerrilla,
            SpaceIdentifiers::BinhDinh
        ));

        forces_in_spaces_instructions.push(format!(
            "Place {} {} in {}.",
            1,
            Forces::UndergroundArvnRanger,
            SpaceIdentifiers::QuangNam
        ));
        forces_in_spaces_instructions.push(format!(
            "Place {} {} in {}.",
            1,
            Forces::ArvnPolice,
            SpaceIdentifiers::QuangNam
        ));

        forces_in_spaces_instructions.push(format!(
            "Place {} {} in {}.",
            1,
            Forces::UsBase,
            SpaceIdentifiers::PleikuDarlac
        ));
        forces_in_spaces_instructions.push(format!(
            "Place {} {} in {}.",
            1,
            Forces::UndergroundUsIrregular,
            SpaceIdentifiers::PleikuDarlac
        ));
        forces_in_spaces_instructions.push(format!(
            "Place {} {} in {}.",
            1,
            Forces::UsTroop,
            SpaceIdentifiers::PleikuDarlac
        ));
        forces_in_spaces_instructions.push(format!(
            "Place {} {} in {}.",
            1,
            Forces::VcBase,
            SpaceIdentifiers::PleikuDarlac
        ));
        forces_in_spaces_instructions.push(format!(
            "Place {} {} in {}.",
            2,
            Forces::UndergroundVcGuerrilla,
            SpaceIdentifiers::PleikuDarlac
        ));

        forces_in_spaces_instructions.push(format!(
            "Place {} {} in {}.",
            1,
            Forces::VcBase,
            SpaceIdentifiers::QuangTinQuangNgai
        ));
        forces_in_spaces_instructions.push(format!(
            "Place {} {} in {}.",
            2,
            Forces::UndergroundVcGuerrilla,
            SpaceIdentifiers::QuangTinQuangNgai
        ));
        forces_in_spaces_instructions.push(format!(
            "Place {} {} in {}.",
            1,
            Forces::VcBase,
            SpaceIdentifiers::QuangDucLongKhanh
        ));
        forces_in_spaces_instructions.push(format!(
            "Place {} {} in {}.",
            2,
            Forces::UndergroundVcGuerrilla,
            SpaceIdentifiers::QuangDucLongKhanh
        ));
        forces_in_spaces_instructions.push(format!(
            "Place {} {} in {}.",
            1,
            Forces::VcBase,
            SpaceIdentifiers::BinhTuyBinhThuan
        ));
        forces_in_spaces_instructions.push(format!(
            "Place {} {} in {}.",
            2,
            Forces::UndergroundVcGuerrilla,
            SpaceIdentifiers::BinhTuyBinhThuan
        ));

        forces_in_spaces_instructions.push(format!(
            "Place {} {} in {}.",
            1,
            Forces::TunneledVcBase,
            SpaceIdentifiers::TayNinh
        ));
        forces_in_spaces_instructions.push(format!(
            "Place {} {} in {}.",
            2,
            Forces::UndergroundVcGuerrilla,
            SpaceIdentifiers::TayNinh
        ));

        forces_in_spaces_instructions.push(format!(
            "Place {} {} in {}.",
            1,
            Forces::ArvnPolice,
            SpaceIdentifiers::PhuBonPhuYen
        ));
        forces_in_spaces_instructions.push(format!(
            "Place {} {} in {}.",
            1,
            Forces::ArvnPolice,
            SpaceIdentifiers::KhanhHoa
        ));
        forces_in_spaces_instructions.push(format!(
            "Place {} {} in {}.",
            1,
            Forces::ArvnPolice,
            SpaceIdentifiers::KienHoaVinhBinh
        ));
        forces_in_spaces_instructions.push(format!(
            "Place {} {} in {}.",
            1,
            Forces::ArvnPolice,
            SpaceIdentifiers::BaXuyen
        ));

        forces_in_spaces_instructions.push(format!(
            "Place {} {} in {}.",
            1,
            Forces::UndergroundVcGuerrilla,
            SpaceIdentifiers::KienPhong
        ));
        forces_in_spaces_instructions.push(format!(
            "Place {} {} in {}.",
            1,
            Forces::UndergroundVcGuerrilla,
            SpaceIdentifiers::KienGiangAnXuyen
        ));

        forces_in_spaces_instructions.push(format!(
            "Place {} {} in {}.",
            1,
            Forces::NvaBase,
            SpaceIdentifiers::NorthVietnam
        ));
        forces_in_spaces_instructions.push(format!(
            "Place {} {} in {}.",
            3,
            Forces::UndergroundNvaGuerrilla,
            SpaceIdentifiers::NorthVietnam
        ));
        forces_in_spaces_instructions.push(format!(
            "Place {} {} in {}.",
            1,
            Forces::NvaBase,
            SpaceIdentifiers::CentralLaos
        ));
        forces_in_spaces_instructions.push(format!(
            "Place {} {} in {}.",
            3,
            Forces::UndergroundNvaGuerrilla,
            SpaceIdentifiers::CentralLaos
        ));
        forces_in_spaces_instructions.push(format!(
            "Place {} {} in {}.",
            1,
            Forces::NvaBase,
            SpaceIdentifiers::SouthernLaos
        ));
        forces_in_spaces_instructions.push(format!(
            "Place {} {} in {}.",
            3,
            Forces::UndergroundNvaGuerrilla,
            SpaceIdentifiers::SouthernLaos
        ));
        forces_in_spaces_instructions.push(format!(
            "Place {} {} in {}.",
            1,
            Forces::NvaBase,
            SpaceIdentifiers::TheParrotsBeak
        ));
        forces_in_spaces_instructions.push(format!(
            "Place {} {} in {}.",
            3,
            Forces::UndergroundNvaGuerrilla,
            SpaceIdentifiers::TheParrotsBeak
        ));

        collection_of_instructions.push(forces_in_spaces_instructions);

        // Setup the initial control types for the occupable spaces.
        board.set_control_type_of_space(ControlTypes::Coin, SpaceIdentifiers::Saigon)?;
        board.set_control_type_of_space(ControlTypes::Coin, SpaceIdentifiers::Hue)?;
        board.set_control_type_of_space(ControlTypes::Coin, SpaceIdentifiers::QuiNhon)?;
        board.set_control_type_of_space(ControlTypes::Coin, SpaceIdentifiers::CamRanh)?;
        board.set_control_type_of_space(ControlTypes::Coin, SpaceIdentifiers::AnLoc)?;
        board.set_control_type_of_space(ControlTypes::Coin, SpaceIdentifiers::CanTho)?;
        board.set_control_type_of_space(ControlTypes::Coin, SpaceIdentifiers::DaNang)?;
        board.set_control_type_of_space(ControlTypes::Coin, SpaceIdentifiers::Kontum)?;
        board.set_control_type_of_space(ControlTypes::Coin, SpaceIdentifiers::QuangNam)?;
        board.set_control_type_of_space(ControlTypes::Coin, SpaceIdentifiers::PhuBonPhuYen)?;
        board.set_control_type_of_space(ControlTypes::Coin, SpaceIdentifiers::KhanhHoa)?;
        board.set_control_type_of_space(ControlTypes::Coin, SpaceIdentifiers::KienHoaVinhBinh)?;
        board.set_control_type_of_space(ControlTypes::Coin, SpaceIdentifiers::BaXuyen)?;
        board.set_control_type_of_space(ControlTypes::Nva, SpaceIdentifiers::NorthVietnam)?;
        board.set_control_type_of_space(ControlTypes::Nva, SpaceIdentifiers::CentralLaos)?;
        board.set_control_type_of_space(ControlTypes::Nva, SpaceIdentifiers::SouthernLaos)?;
        board.set_control_type_of_space(ControlTypes::Nva, SpaceIdentifiers::TheParrotsBeak)?;

        // Push instructions regarding initial control.
        let mut control_types_in_spaces_instructions: Vec<String> = Vec::new();

        control_types_in_spaces_instructions.push(format!(
            "Place a {} marker in {}.",
            board.get_control_type_of_space(SpaceIdentifiers::Saigon)?,
            SpaceIdentifiers::Saigon
        ));
        control_types_in_spaces_instructions.push(format!(
            "Place a {} marker in {}.",
            board.get_control_type_of_space(SpaceIdentifiers::Hue)?,
            SpaceIdentifiers::Hue
        ));
        control_types_in_spaces_instructions.push(format!(
            "Place a {} marker in {}.",
            board.get_control_type_of_space(SpaceIdentifiers::QuiNhon)?,
            SpaceIdentifiers::QuiNhon
        ));
        control_types_in_spaces_instructions.push(format!(
            "Place a {} marker in {}.",
            board.get_control_type_of_space(SpaceIdentifiers::CamRanh)?,
            SpaceIdentifiers::CamRanh
        ));
        control_types_in_spaces_instructions.push(format!(
            "Place a {} marker in {}.",
            board.get_control_type_of_space(SpaceIdentifiers::AnLoc)?,
            SpaceIdentifiers::AnLoc
        ));
        control_types_in_spaces_instructions.push(format!(
            "Place a {} marker in {}.",
            board.get_control_type_of_space(SpaceIdentifiers::CanTho)?,
            SpaceIdentifiers::CanTho
        ));
        control_types_in_spaces_instructions.push(format!(
            "Place a {} marker in {}.",
            board.get_control_type_of_space(SpaceIdentifiers::DaNang)?,
            SpaceIdentifiers::DaNang
        ));
        control_types_in_spaces_instructions.push(format!(
            "Place a {} marker in {}.",
            board.get_control_type_of_space(SpaceIdentifiers::Kontum)?,
            SpaceIdentifiers::Kontum
        ));
        control_types_in_spaces_instructions.push(format!(
            "Place a {} marker in {}.",
            board.get_control_type_of_space(SpaceIdentifiers::QuangNam)?,
            SpaceIdentifiers::QuangNam
        ));
        control_types_in_spaces_instructions.push(format!(
            "Place a {} marker in {}.",
            board.get_control_type_of_space(SpaceIdentifiers::PhuBonPhuYen)?,
            SpaceIdentifiers::PhuBonPhuYen
        ));
        control_types_in_spaces_instructions.push(format!(
            "Place a {} marker in {}.",
            board.get_control_type_of_space(SpaceIdentifiers::KhanhHoa)?,
            SpaceIdentifiers::KhanhHoa
        ));
        control_types_in_spaces_instructions.push(format!(
            "Place a {} marker in {}.",
            board.get_control_type_of_space(SpaceIdentifiers::KienHoaVinhBinh)?,
            SpaceIdentifiers::KienHoaVinhBinh
        ));
        control_types_in_spaces_instructions.push(format!(
            "Place a {} marker in {}.",
            board.get_control_type_of_space(SpaceIdentifiers::BaXuyen)?,
            SpaceIdentifiers::BaXuyen
        ));
        control_types_in_spaces_instructions.push(format!(
            "Place a {} marker in {}.",
            board.get_control_type_of_space(SpaceIdentifiers::NorthVietnam)?,
            SpaceIdentifiers::NorthVietnam
        ));
        control_types_in_spaces_instructions.push(format!(
            "Place a {} marker in {}.",
            board.get_control_type_of_space(SpaceIdentifiers::CentralLaos)?,
            SpaceIdentifiers::CentralLaos
        ));
        control_types_in_spaces_instructions.push(format!(
            "Place a {} marker in {}.",
            board.get_control_type_of_space(SpaceIdentifiers::SouthernLaos)?,
            SpaceIdentifiers::SouthernLaos
        ));
        control_types_in_spaces_instructions.push(format!(
            "Place a {} marker in {}.",
            board.get_control_type_of_space(SpaceIdentifiers::TheParrotsBeak)?,
            SpaceIdentifiers::TheParrotsBeak
        ));

        collection_of_instructions.push(control_types_in_spaces_instructions);

        // Set up initial Support levels
        board
            .set_support_level_of_space(SupportLevels::PassiveSupport, SpaceIdentifiers::Saigon)?;
        board
            .set_support_level_of_space(SupportLevels::PassiveSupport, SpaceIdentifiers::QuiNhon)?;
        board
            .set_support_level_of_space(SupportLevels::PassiveSupport, SpaceIdentifiers::CamRanh)?;
        board.set_support_level_of_space(SupportLevels::PassiveSupport, SpaceIdentifiers::AnLoc)?;
        board
            .set_support_level_of_space(SupportLevels::PassiveSupport, SpaceIdentifiers::CanTho)?;
        board.set_support_level_of_space(
            SupportLevels::ActiveOpposition,
            SpaceIdentifiers::QuangTinQuangNgai,
        )?;
        board.set_support_level_of_space(
            SupportLevels::ActiveOpposition,
            SpaceIdentifiers::QuangDucLongKhanh,
        )?;
        board.set_support_level_of_space(
            SupportLevels::ActiveOpposition,
            SpaceIdentifiers::BinhTuyBinhThuan,
        )?;
        board.set_support_level_of_space(
            SupportLevels::ActiveOpposition,
            SpaceIdentifiers::TayNinh,
        )?;
        board.set_support_level_of_space(
            SupportLevels::PassiveSupport,
            SpaceIdentifiers::PhuBonPhuYen,
        )?;
        board.set_support_level_of_space(
            SupportLevels::PassiveSupport,
            SpaceIdentifiers::KhanhHoa,
        )?;
        board.set_support_level_of_space(
            SupportLevels::PassiveSupport,
            SpaceIdentifiers::KienHoaVinhBinh,
        )?;
        board
            .set_support_level_of_space(SupportLevels::PassiveSupport, SpaceIdentifiers::BaXuyen)?;
        board.set_support_level_of_space(
            SupportLevels::ActiveOpposition,
            SpaceIdentifiers::KienPhong,
        )?;
        board.set_support_level_of_space(
            SupportLevels::ActiveOpposition,
            SpaceIdentifiers::KienGiangAnXuyen,
        )?;

        // Push instructions regarding initial support levels.
        let mut support_levels_in_spaces_instructions: Vec<String> = Vec::new();

        support_levels_in_spaces_instructions.push(format!(
            "Place a {} marker in {}.",
            board.get_support_level_of_space(SpaceIdentifiers::Saigon)?,
            SpaceIdentifiers::Saigon
        ));
        support_levels_in_spaces_instructions.push(format!(
            "Place a {} marker in {}.",
            board.get_support_level_of_space(SpaceIdentifiers::QuiNhon)?,
            SpaceIdentifiers::QuiNhon
        ));
        support_levels_in_spaces_instructions.push(format!(
            "Place a {} marker in {}.",
            board.get_support_level_of_space(SpaceIdentifiers::CamRanh)?,
            SpaceIdentifiers::CamRanh
        ));
        support_levels_in_spaces_instructions.push(format!(
            "Place a {} marker in {}.",
            board.get_support_level_of_space(SpaceIdentifiers::AnLoc)?,
            SpaceIdentifiers::AnLoc
        ));
        support_levels_in_spaces_instructions.push(format!(
            "Place a {} marker in {}.",
            board.get_support_level_of_space(SpaceIdentifiers::CanTho)?,
            SpaceIdentifiers::CanTho
        ));
        support_levels_in_spaces_instructions.push(format!(
            "Place a {} marker in {}.",
            board.get_support_level_of_space(SpaceIdentifiers::QuangTinQuangNgai)?,
            SpaceIdentifiers::QuangTinQuangNgai
        ));
        support_levels_in_spaces_instructions.push(format!(
            "Place a {} marker in {}.",
            board.get_support_level_of_space(SpaceIdentifiers::QuangDucLongKhanh)?,
            SpaceIdentifiers::QuangDucLongKhanh
        ));
        support_levels_in_spaces_instructions.push(format!(
            "Place a {} marker in {}.",
            board.get_support_level_of_space(SpaceIdentifiers::BinhTuyBinhThuan)?,
            SpaceIdentifiers::BinhTuyBinhThuan
        ));
        support_levels_in_spaces_instructions.push(format!(
            "Place a {} marker in {}.",
            board.get_support_level_of_space(SpaceIdentifiers::TayNinh)?,
            SpaceIdentifiers::TayNinh
        ));
        support_levels_in_spaces_instructions.push(format!(
            "Place a {} marker in {}.",
            board.get_support_level_of_space(SpaceIdentifiers::PhuBonPhuYen)?,
            SpaceIdentifiers::PhuBonPhuYen
        ));
        support_levels_in_spaces_instructions.push(format!(
            "Place a {} marker in {}.",
            board.get_support_level_of_space(SpaceIdentifiers::KhanhHoa)?,
            SpaceIdentifiers::KhanhHoa
        ));
        support_levels_in_spaces_instructions.push(format!(
            "Place a {} marker in {}.",
            board.get_support_level_of_space(SpaceIdentifiers::KienHoaVinhBinh)?,
            SpaceIdentifiers::KienHoaVinhBinh
        ));
        support_levels_in_spaces_instructions.push(format!(
            "Place a {} marker in {}.",
            board.get_support_level_of_space(SpaceIdentifiers::BaXuyen)?,
            SpaceIdentifiers::BaXuyen
        ));
        support_levels_in_spaces_instructions.push(format!(
            "Place a {} marker in {}.",
            board.get_support_level_of_space(SpaceIdentifiers::KienPhong)?,
            SpaceIdentifiers::KienPhong
        ));
        support_levels_in_spaces_instructions.push(format!(
            "Place a {} marker in {}.",
            board.get_support_level_of_space(SpaceIdentifiers::KienGiangAnXuyen)?,
            SpaceIdentifiers::KienGiangAnXuyen
        ));

        collection_of_instructions.push(support_levels_in_spaces_instructions);

        Ok((board, collection_of_instructions))
    }
}
