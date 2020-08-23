use board::domain::province::Province;
use board::domain::space::Spaces;
use game_definitions::geographic_areas::GeographicAreas;
use game_definitions::space_identifiers::SpaceIdentifiers;
use game_definitions::terrain_types::TerrainTypes;

pub fn create_binh_tuy() -> (SpaceIdentifiers, Spaces) {
    (
        SpaceIdentifiers::BinhTuyBinhThuan,
        Province::new(
            SpaceIdentifiers::BinhTuyBinhThuan,
            1,
            GeographicAreas::SouthVietnam,
            TerrainTypes::Jungle,
            vec![
                SpaceIdentifiers::Route20,
                SpaceIdentifiers::Route21,
                SpaceIdentifiers::Route11,
                SpaceIdentifiers::Route1South,
                SpaceIdentifiers::Saigon,
                SpaceIdentifiers::QuangDucLongKhanh,
                SpaceIdentifiers::KhanhHoa,
                SpaceIdentifiers::CamRanh,
            ],
        )
        .into(),
    )
}
