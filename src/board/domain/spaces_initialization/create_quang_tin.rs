use board::domain::province::Province;
use board::domain::space::Spaces;
use game_definitions::geographic_areas::GeographicAreas;
use game_definitions::space_identifiers::SpaceIdentifiers;
use game_definitions::terrain_types::TerrainTypes;

pub fn create_quang_tin() -> (SpaceIdentifiers, Spaces) {
    (
        SpaceIdentifiers::QuangTinQuangNgai,
        Province::new(
            GeographicAreas::SouthVietnam,
            TerrainTypes::Lowland,
            vec![
                SpaceIdentifiers::DaNang,
                SpaceIdentifiers::Route1NorthEast,
                SpaceIdentifiers::BinhDinh,
                SpaceIdentifiers::Route14North,
                SpaceIdentifiers::SouthernLaos,
                SpaceIdentifiers::QuangNam,
            ],
        )
        .into(),
    )
}
