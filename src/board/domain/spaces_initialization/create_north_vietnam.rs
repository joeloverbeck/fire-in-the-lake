use board::domain::province::Province;
use board::domain::space::Spaces;
use game_definitions::geographic_areas::GeographicAreas;
use game_definitions::space_identifiers::SpaceIdentifiers;
use game_definitions::terrain_types::TerrainTypes;

pub fn create_north_vietnam() -> (SpaceIdentifiers, Spaces) {
    (
        SpaceIdentifiers::NorthVietnam,
        Province::new(
            SpaceIdentifiers::NorthVietnam,
            0,
            GeographicAreas::NorthVietnam,
            TerrainTypes::Highland,
            vec![
                SpaceIdentifiers::Route1North,
                SpaceIdentifiers::CentralLaos,
                SpaceIdentifiers::QuangTriThuaThien,
            ],
        )
        .into(),
    )
}
