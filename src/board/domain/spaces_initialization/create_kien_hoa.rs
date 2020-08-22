use board::domain::province::Province;
use board::domain::space::Spaces;
use game_definitions::geographic_areas::GeographicAreas;
use game_definitions::space_identifiers::SpaceIdentifiers;
use game_definitions::terrain_types::TerrainTypes;

pub fn create_kien_hoa() -> (SpaceIdentifiers, Spaces) {
    (
        SpaceIdentifiers::KienHoaVinhBinh,
        Province::new(
            GeographicAreas::SouthVietnam,
            TerrainTypes::Lowland,
            vec![
                SpaceIdentifiers::MekongSouth,
                SpaceIdentifiers::Route4MekongEast,
                SpaceIdentifiers::BaXuyen,
                SpaceIdentifiers::CanTho,
                SpaceIdentifiers::Saigon,
                SpaceIdentifiers::KienPhong,
            ],
        )
        .into(),
    )
}
