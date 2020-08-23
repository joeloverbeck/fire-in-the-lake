use board::domain::province::Province;
use board::domain::space::Spaces;
use game_definitions::geographic_areas::GeographicAreas;
use game_definitions::space_identifiers::SpaceIdentifiers;
use game_definitions::terrain_types::TerrainTypes;

pub fn create_the_fishhook() -> (SpaceIdentifiers, Spaces) {
    (
        SpaceIdentifiers::TheFishhook,
        Province::new(
            SpaceIdentifiers::TheFishhook,
            0,
            GeographicAreas::Cambodia,
            TerrainTypes::Jungle,
            vec![
                SpaceIdentifiers::NortheastCambodia,
                SpaceIdentifiers::PleikuDarlac,
                SpaceIdentifiers::QuangDucLongKhanh,
                SpaceIdentifiers::PhuocLong,
                SpaceIdentifiers::AnLoc,
                SpaceIdentifiers::TayNinh,
                SpaceIdentifiers::TheParrotsBeak,
            ],
        )
        .into(),
    )
}
