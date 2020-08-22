use board::domain::province::Province;
use board::domain::space::Spaces;
use game_definitions::geographic_areas::GeographicAreas;
use game_definitions::space_identifiers::SpaceIdentifiers;
use game_definitions::terrain_types::TerrainTypes;

pub fn create_southern_laos() -> (SpaceIdentifiers, Spaces) {
    (
        SpaceIdentifiers::SouthernLaos,
        Province::new(
            GeographicAreas::Laos,
            TerrainTypes::Jungle,
            vec![
                SpaceIdentifiers::Route14CentralNorth,
                SpaceIdentifiers::Route14North,
                SpaceIdentifiers::NortheastCambodia,
                SpaceIdentifiers::PleikuDarlac,
                SpaceIdentifiers::BinhDinh,
                SpaceIdentifiers::QuangTinQuangNgai,
                SpaceIdentifiers::QuangNam,
                SpaceIdentifiers::CentralLaos,
            ],
        )
        .into(),
    )
}
