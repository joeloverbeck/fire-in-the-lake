use board::domain::province::Province;
use board::domain::space::Spaces;
use game_definitions::geographic_areas::GeographicAreas;
use game_definitions::space_identifiers::SpaceIdentifiers;
use game_definitions::terrain_types::TerrainTypes;

pub fn create_kien_phong() -> (SpaceIdentifiers, Spaces) {
    (
        SpaceIdentifiers::KienPhong,
        Province::new(
            GeographicAreas::SouthVietnam,
            TerrainTypes::Lowland,
            vec![
                SpaceIdentifiers::TheParrotsBeak,
                SpaceIdentifiers::TayNinh,
                SpaceIdentifiers::Route4MekongEast,
                SpaceIdentifiers::Saigon,
                SpaceIdentifiers::KienHoaVinhBinh,
                SpaceIdentifiers::CanTho,
                SpaceIdentifiers::MekongNorth,
                SpaceIdentifiers::KienGiangAnXuyen,
            ],
        )
        .into(),
    )
}
