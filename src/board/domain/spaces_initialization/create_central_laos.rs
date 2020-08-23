use board::domain::province::Province;
use board::domain::space::Spaces;
use game_definitions::geographic_areas::GeographicAreas;
use game_definitions::space_identifiers::SpaceIdentifiers;
use game_definitions::terrain_types::TerrainTypes;

pub fn create_central_laos() -> (SpaceIdentifiers, Spaces) {
    (
        SpaceIdentifiers::CentralLaos,
        Province::new(
            SpaceIdentifiers::CentralLaos,
            0,
            GeographicAreas::Laos,
            TerrainTypes::Jungle,
            vec![
                SpaceIdentifiers::Route1North,
                SpaceIdentifiers::SouthernLaos,
                SpaceIdentifiers::QuangNam,
                SpaceIdentifiers::QuangTriThuaThien,
                SpaceIdentifiers::NorthVietnam,
            ],
        )
        .into(),
    )
}
