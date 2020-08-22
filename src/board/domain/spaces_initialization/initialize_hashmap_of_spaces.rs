use board::domain::city::City;
use board::domain::loc::LoC;
use board::domain::space::Spaces;
use board::domain::spaces_initialization::create_an_loc::create_an_loc;
use board::domain::spaces_initialization::create_ba_xuyen::create_ba_xuyen;
use board::domain::spaces_initialization::create_binh_dinh::create_binh_dinh;
use board::domain::spaces_initialization::create_binh_tuy::create_binh_tuy;
use board::domain::spaces_initialization::create_central_laos::create_central_laos;
use board::domain::spaces_initialization::create_hue::create_hue;
use board::domain::spaces_initialization::create_khanh_hoa::create_khanh_hoa;
use board::domain::spaces_initialization::create_kien_giang::create_kien_giang;
use board::domain::spaces_initialization::create_kien_hoa::create_kien_hoa;
use board::domain::spaces_initialization::create_kien_phong::create_kien_phong;
use board::domain::spaces_initialization::create_north_vietnam::create_north_vietnam;
use board::domain::spaces_initialization::create_phu_bon::create_phu_bon;
use board::domain::spaces_initialization::create_pleiku::create_pleiku;
use board::domain::spaces_initialization::create_quang_duc::create_quang_duc;
use board::domain::spaces_initialization::create_quang_nam::create_quang_nam;
use board::domain::spaces_initialization::create_quang_tin::create_quang_tin;
use board::domain::spaces_initialization::create_quang_tri::create_quang_tri;
use board::domain::spaces_initialization::create_qui_nhon::create_qui_nhon;
use board::domain::spaces_initialization::create_saigon::create_saigon;
use board::domain::spaces_initialization::create_sihanoukville::create_sihanoukville;
use board::domain::spaces_initialization::create_southern_laos::create_southern_laos;
use board::domain::spaces_initialization::create_tay_ninh::create_tay_ninh;
use board::domain::spaces_initialization::create_the_fishhook::create_the_fishhook;
use board::domain::spaces_initialization::create_the_parrots_beak::create_the_parrots_beak;
use game_definitions::geographic_areas::GeographicAreas;
use game_definitions::space_identifiers::SpaceIdentifiers;
use std::collections::HashMap;

pub fn initialize_hashmap_of_spaces() -> Result<HashMap<SpaceIdentifiers, Spaces>, String> {
    Ok([
        create_saigon(),
        create_hue(),
        create_an_loc(),
        create_kien_giang(),
        create_ba_xuyen(),
        create_quang_nam(),
        create_binh_dinh(),
        (
            SpaceIdentifiers::CanTho,
            City::new(
                GeographicAreas::SouthVietnam,
                vec![
                    SpaceIdentifiers::Route4MekongEast,
                    SpaceIdentifiers::MekongSouth,
                    SpaceIdentifiers::KienHoaVinhBinh,
                    SpaceIdentifiers::BaXuyen,
                    SpaceIdentifiers::Route4West,
                    SpaceIdentifiers::KienGiangAnXuyen,
                    SpaceIdentifiers::MekongNorth,
                    SpaceIdentifiers::KienPhong,
                ],
            )
            .into(),
        ),
        create_kien_phong(),
        create_quang_tri(),
        create_north_vietnam(),
        create_sihanoukville(),
        create_the_parrots_beak(),
        create_the_fishhook(),
        create_quang_tin(),
        create_quang_duc(),
        create_binh_tuy(),
        create_pleiku(),
        create_central_laos(),
        create_southern_laos(),
        create_qui_nhon(),
        create_khanh_hoa(),
        create_kien_hoa(),
        create_phu_bon(),
        create_tay_ninh(),
        (
            SpaceIdentifiers::Kontum,
            City::new(
                GeographicAreas::SouthVietnam,
                vec![
                    SpaceIdentifiers::Route19,
                    SpaceIdentifiers::Route14Central,
                    SpaceIdentifiers::Route14CentralNorth,
                    SpaceIdentifiers::PhuBonPhuYen,
                    SpaceIdentifiers::PleikuDarlac,
                    SpaceIdentifiers::BinhDinh,
                ],
            )
            .into(),
        ),
        (
            SpaceIdentifiers::DaNang,
            City::new(
                GeographicAreas::SouthVietnam,
                vec![
                    SpaceIdentifiers::Route1NorthEast,
                    SpaceIdentifiers::Route14North,
                    SpaceIdentifiers::Route1North,
                    SpaceIdentifiers::QuangTinQuangNgai,
                    SpaceIdentifiers::QuangNam,
                ],
            )
            .into(),
        ),
        (
            SpaceIdentifiers::CamRanh,
            City::new(
                GeographicAreas::SouthVietnam,
                vec![
                    SpaceIdentifiers::Route1South,
                    SpaceIdentifiers::Route1SouthEast,
                    SpaceIdentifiers::Route11,
                    SpaceIdentifiers::BinhTuyBinhThuan,
                    SpaceIdentifiers::KhanhHoa,
                ],
            )
            .into(),
        ),
        (
            SpaceIdentifiers::MekongNorth,
            LoC::new(
                GeographicAreas::SouthVietnam,
                vec![
                    SpaceIdentifiers::CanTho,
                    SpaceIdentifiers::KienGiangAnXuyen,
                    SpaceIdentifiers::KienPhong,
                    SpaceIdentifiers::TheParrotsBeak,
                ],
            )
            .into(),
        ),
        (
            SpaceIdentifiers::Route4West,
            LoC::new(
                GeographicAreas::SouthVietnam,
                vec![
                    SpaceIdentifiers::KienGiangAnXuyen,
                    SpaceIdentifiers::BaXuyen,
                    SpaceIdentifiers::CanTho,
                ],
            )
            .into(),
        ),
    ]
    .iter()
    .cloned()
    .collect())
}
