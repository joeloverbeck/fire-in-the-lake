use board::domain::province::Province;
use board::domain::space::Spaces;
use game_definitions::geographic_areas::GeographicAreas;
use game_definitions::space_identifiers::SpaceIdentifiers;
use game_definitions::terrain_types::TerrainTypes;

pub fn create_quang_tri() -> (SpaceIdentifiers, Spaces) {
    (
        SpaceIdentifiers::QuangTriThuaThien,
        Province::new(
            SpaceIdentifiers::QuangTriThuaThien,
            2,
            GeographicAreas::SouthVietnam,
            TerrainTypes::Highland,
            vec![
                SpaceIdentifiers::Hue,
                SpaceIdentifiers::Route1North,
                SpaceIdentifiers::CentralLaos,
                SpaceIdentifiers::QuangNam,
                SpaceIdentifiers::NorthVietnam,
            ],
        )
        .into(),
    )
}
