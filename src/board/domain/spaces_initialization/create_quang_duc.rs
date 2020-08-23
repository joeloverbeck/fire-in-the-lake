use board::domain::province::Province;
use board::domain::space::Spaces;
use game_definitions::geographic_areas::GeographicAreas;
use game_definitions::space_identifiers::SpaceIdentifiers;
use game_definitions::terrain_types::TerrainTypes;

pub fn create_quang_duc() -> (SpaceIdentifiers, Spaces) {
    (
        SpaceIdentifiers::QuangDucLongKhanh,
        Province::new(
            SpaceIdentifiers::QuangDucLongKhanh,
            1,
            GeographicAreas::SouthVietnam,
            TerrainTypes::Jungle,
            vec![
                SpaceIdentifiers::Route21,
                SpaceIdentifiers::KhanhHoa,
                SpaceIdentifiers::Route11,
                SpaceIdentifiers::Route20,
                SpaceIdentifiers::BinhTuyBinhThuan,
                SpaceIdentifiers::Saigon,
                SpaceIdentifiers::Route14South,
                SpaceIdentifiers::PhuocLong,
                SpaceIdentifiers::TheFishhook,
                SpaceIdentifiers::TayNinh,
                SpaceIdentifiers::PleikuDarlac,
                SpaceIdentifiers::Route14Central,
            ],
        )
        .into(),
    )
}
