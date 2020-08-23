use board::domain::province::Province;
use board::domain::space::Spaces;
use game_definitions::geographic_areas::GeographicAreas;
use game_definitions::space_identifiers::SpaceIdentifiers;
use game_definitions::terrain_types::TerrainTypes;

pub fn create_tay_ninh() -> (SpaceIdentifiers, Spaces) {
    (
        SpaceIdentifiers::TayNinh,
        Province::new(
            SpaceIdentifiers::TayNinh,
            2,
            GeographicAreas::SouthVietnam,
            TerrainTypes::Jungle,
            vec![
                SpaceIdentifiers::Route13,
                SpaceIdentifiers::Route14South,
                SpaceIdentifiers::Saigon,
                SpaceIdentifiers::KienPhong,
                SpaceIdentifiers::TheParrotsBeak,
                SpaceIdentifiers::TheFishhook,
                SpaceIdentifiers::AnLoc,
                SpaceIdentifiers::PhuocLong,
                SpaceIdentifiers::QuangDucLongKhanh,
            ],
        )
        .into(),
    )
}
