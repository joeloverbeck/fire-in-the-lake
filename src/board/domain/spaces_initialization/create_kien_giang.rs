use board::domain::province::Province;
use board::domain::space::Spaces;
use game_definitions::geographic_areas::GeographicAreas;
use game_definitions::space_identifiers::SpaceIdentifiers;
use game_definitions::terrain_types::TerrainTypes;

pub fn create_kien_giang() -> (SpaceIdentifiers, Spaces) {
    (
        SpaceIdentifiers::KienGiangAnXuyen,
        Province::new(
            GeographicAreas::SouthVietnam,
            TerrainTypes::Lowland,
            vec![
                SpaceIdentifiers::Route4West,
                SpaceIdentifiers::BaXuyen,
                SpaceIdentifiers::CanTho,
                SpaceIdentifiers::Sihanoukville,
                SpaceIdentifiers::TheParrotsBeak,
                SpaceIdentifiers::KienPhong,
                SpaceIdentifiers::MekongNorth,
            ],
        )
        .into(),
    )
}
