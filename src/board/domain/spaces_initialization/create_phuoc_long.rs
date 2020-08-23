use board::domain::province::Province;
use board::domain::space::Spaces;
use game_definitions::geographic_areas::GeographicAreas;
use game_definitions::space_identifiers::SpaceIdentifiers;
use game_definitions::terrain_types::TerrainTypes;

pub fn create_phuoc_long() -> (SpaceIdentifiers, Spaces) {
    (
        SpaceIdentifiers::PhuocLong,
        Province::new(
            SpaceIdentifiers::PhuocLong,
            0,
            GeographicAreas::SouthVietnam,
            TerrainTypes::Jungle,
            vec![
                SpaceIdentifiers::Route14South,
                SpaceIdentifiers::TheFishhook,
                SpaceIdentifiers::QuangDucLongKhanh,
                SpaceIdentifiers::TayNinh,
                SpaceIdentifiers::AnLoc,
            ],
        )
        .into(),
    )
}
