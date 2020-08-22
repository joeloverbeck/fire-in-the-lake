use board::domain::province::Province;
use board::domain::space::Spaces;
use game_definitions::geographic_areas::GeographicAreas;
use game_definitions::space_identifiers::SpaceIdentifiers;
use game_definitions::terrain_types::TerrainTypes;

pub fn create_ba_xuyen() -> (SpaceIdentifiers, Spaces) {
    (
        SpaceIdentifiers::BaXuyen,
        Province::new(
            GeographicAreas::SouthVietnam,
            TerrainTypes::Lowland,
            vec![
                SpaceIdentifiers::KienGiangAnXuyen,
                SpaceIdentifiers::Route4West,
                SpaceIdentifiers::CanTho,
                SpaceIdentifiers::MekongSouth,
                SpaceIdentifiers::KienHoaVinhBinh,
            ],
        )
        .into(),
    )
}
