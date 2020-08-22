use board::domain::province::Province;
use board::domain::space::Spaces;
use game_definitions::geographic_areas::GeographicAreas;
use game_definitions::space_identifiers::SpaceIdentifiers;
use game_definitions::terrain_types::TerrainTypes;

pub fn create_khanh_hoa() -> (SpaceIdentifiers, Spaces) {
    (
        SpaceIdentifiers::KhanhHoa,
        Province::new(
            GeographicAreas::SouthVietnam,
            TerrainTypes::Highland,
            vec![
                SpaceIdentifiers::Route1SouthEast,
                SpaceIdentifiers::Route11,
                SpaceIdentifiers::Route20,
                SpaceIdentifiers::Route21,
                SpaceIdentifiers::Route14South,
                SpaceIdentifiers::Route14Central,
                SpaceIdentifiers::PhuBonPhuYen,
                SpaceIdentifiers::CamRanh,
                SpaceIdentifiers::BinhTuyBinhThuan,
                SpaceIdentifiers::QuangDucLongKhanh,
                SpaceIdentifiers::PleikuDarlac,
            ],
        )
        .into(),
    )
}
