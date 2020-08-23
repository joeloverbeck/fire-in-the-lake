use board::domain::province::Province;
use board::domain::space::Spaces;
use game_definitions::geographic_areas::GeographicAreas;
use game_definitions::space_identifiers::SpaceIdentifiers;
use game_definitions::terrain_types::TerrainTypes;

pub fn create_the_parrots_beak() -> (SpaceIdentifiers, Spaces) {
    (
        SpaceIdentifiers::TheParrotsBeak,
        Province::new(
            SpaceIdentifiers::TheParrotsBeak,
            0,
            GeographicAreas::Cambodia,
            TerrainTypes::Jungle,
            vec![
                SpaceIdentifiers::Sihanoukville,
                SpaceIdentifiers::KienGiangAnXuyen,
                SpaceIdentifiers::MekongNorth,
                SpaceIdentifiers::TheFishhook,
                SpaceIdentifiers::TayNinh,
                SpaceIdentifiers::KienPhong,
            ],
        )
        .into(),
    )
}
