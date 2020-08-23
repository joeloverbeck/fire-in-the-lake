use board::domain::province::Province;
use board::domain::space::Spaces;
use game_definitions::geographic_areas::GeographicAreas;
use game_definitions::space_identifiers::SpaceIdentifiers;
use game_definitions::terrain_types::TerrainTypes;

pub fn create_northeast_cambodia() -> (SpaceIdentifiers, Spaces) {
    (
        SpaceIdentifiers::NortheastCambodia,
        Province::new(
            SpaceIdentifiers::NortheastCambodia,
            0,
            GeographicAreas::Cambodia,
            TerrainTypes::Jungle,
            vec![
                SpaceIdentifiers::TheFishhook,
                SpaceIdentifiers::PleikuDarlac,
                SpaceIdentifiers::SouthernLaos,
            ],
        )
        .into(),
    )
}
