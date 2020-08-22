use board::domain::province::Province;
use board::domain::space::Spaces;
use game_definitions::geographic_areas::GeographicAreas;
use game_definitions::space_identifiers::SpaceIdentifiers;
use game_definitions::terrain_types::TerrainTypes;

pub fn create_sihanoukville() -> (SpaceIdentifiers, Spaces) {
    (
        SpaceIdentifiers::Sihanoukville,
        Province::new(
            GeographicAreas::Cambodia,
            TerrainTypes::Jungle,
            vec![
                SpaceIdentifiers::TheParrotsBeak,
                SpaceIdentifiers::KienGiangAnXuyen,
            ],
        )
        .into(),
    )
}
