use board::domain::space::Spaces;
use board::domain::spaces_initialization::create_an_loc::create_an_loc;
use board::domain::spaces_initialization::create_ba_xuyen::create_ba_xuyen;
use board::domain::spaces_initialization::create_binh_dinh::create_binh_dinh;
use board::domain::spaces_initialization::create_binh_tuy::create_binh_tuy;
use board::domain::spaces_initialization::create_cam_ranh::create_cam_ranh;
use board::domain::spaces_initialization::create_can_tho::create_can_tho;
use board::domain::spaces_initialization::create_central_laos::create_central_laos;
use board::domain::spaces_initialization::create_da_nang::create_da_nang;
use board::domain::spaces_initialization::create_hue::create_hue;
use board::domain::spaces_initialization::create_khanh_hoa::create_khanh_hoa;
use board::domain::spaces_initialization::create_kien_giang::create_kien_giang;
use board::domain::spaces_initialization::create_kien_hoa::create_kien_hoa;
use board::domain::spaces_initialization::create_kien_phong::create_kien_phong;
use board::domain::spaces_initialization::create_kontum::create_kontum;
use board::domain::spaces_initialization::create_mekong_north::create_mekong_north;
use board::domain::spaces_initialization::create_mekong_south::create_mekong_south;
use board::domain::spaces_initialization::create_north_vietnam::create_north_vietnam;
use board::domain::spaces_initialization::create_northeast_cambodia::create_northeast_cambodia;
use board::domain::spaces_initialization::create_phu_bon::create_phu_bon;
use board::domain::spaces_initialization::create_phuoc_long::create_phuoc_long;
use board::domain::spaces_initialization::create_pleiku::create_pleiku;
use board::domain::spaces_initialization::create_quang_duc::create_quang_duc;
use board::domain::spaces_initialization::create_quang_nam::create_quang_nam;
use board::domain::spaces_initialization::create_quang_tin::create_quang_tin;
use board::domain::spaces_initialization::create_quang_tri::create_quang_tri;
use board::domain::spaces_initialization::create_qui_nhon::create_qui_nhon;
use board::domain::spaces_initialization::create_route_11::create_route_11;
use board::domain::spaces_initialization::create_route_14_central::create_route_14_central;
use board::domain::spaces_initialization::create_route_14_central_north::create_route_14_central_north;
use board::domain::spaces_initialization::create_route_14_north::create_route_14_north;
use board::domain::spaces_initialization::create_route_14_south::create_route_14_south;
use board::domain::spaces_initialization::create_route_19::create_route_19;
use board::domain::spaces_initialization::create_route_1_east::create_route_1_east;
use board::domain::spaces_initialization::create_route_1_north::create_route_1_north;
use board::domain::spaces_initialization::create_route_1_north_east::create_route_1_north_east;
use board::domain::spaces_initialization::create_route_1_south::create_route_1_south;
use board::domain::spaces_initialization::create_route_1_south_east::create_route_1_south_east;
use board::domain::spaces_initialization::create_route_20::create_route_20;
use board::domain::spaces_initialization::create_route_21::create_route_21;
use board::domain::spaces_initialization::create_route_4_mekong_east::create_route_4_mekong_east;
use board::domain::spaces_initialization::create_route_4_west::create_route_4_west;
use board::domain::spaces_initialization::create_saigon::create_saigon;
use board::domain::spaces_initialization::create_sihanoukville::create_sihanoukville;
use board::domain::spaces_initialization::create_southern_laos::create_southern_laos;
use board::domain::spaces_initialization::create_tay_ninh::create_tay_ninh;
use board::domain::spaces_initialization::create_the_fishhook::create_the_fishhook;
use board::domain::spaces_initialization::create_the_parrots_beak::create_the_parrots_beak;
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
        create_can_tho(),
        create_kien_phong(),
        create_quang_tri(),
        create_north_vietnam(),
        create_sihanoukville(),
        create_the_parrots_beak(),
        create_the_fishhook(),
        create_northeast_cambodia(),
        create_quang_tin(),
        create_phuoc_long(),
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
        create_kontum(),
        create_da_nang(),
        create_cam_ranh(),
        create_mekong_north(),
        create_mekong_south(),
        create_route_1_north(),
        create_route_1_east(),
        create_route_1_north_east(),
        create_route_1_south_east(),
        create_route_1_south(),
        create_route_4_west(),
        create_route_4_mekong_east(),
        create_route_11(),
        create_route_14_central(),
        create_route_14_central_north(),
        create_route_14_north(),
        create_route_14_south(),
        create_route_19(),
        create_route_20(),
        create_route_21(),
    ]
    .iter()
    .cloned()
    .collect())
}
