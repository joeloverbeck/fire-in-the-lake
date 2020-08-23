use board::domain::province::Province;
use board::domain::space::Spaces;
use game_definitions::geographic_areas::GeographicAreas;
use game_definitions::space_identifiers::SpaceIdentifiers;
use game_definitions::terrain_types::TerrainTypes;

pub fn create_quang_nam() -> (SpaceIdentifiers, Spaces) {
    (
        SpaceIdentifiers::QuangNam,
        Province::new(
            SpaceIdentifiers::QuangNam,
            1,
            GeographicAreas::SouthVietnam,
            TerrainTypes::Highland,
            vec![
                SpaceIdentifiers::DaNang,
                SpaceIdentifiers::Route14North,
                SpaceIdentifiers::QuangTinQuangNgai,
                SpaceIdentifiers::Route1North,
                SpaceIdentifiers::CentralLaos,
                SpaceIdentifiers::QuangTriThuaThien,
                SpaceIdentifiers::SouthernLaos,
            ],
        )
        .into(),
    )
}
