use board::domain::city::City;
use board::domain::space::Spaces;
use game_definitions::geographic_areas::GeographicAreas;
use game_definitions::space_identifiers::SpaceIdentifiers;

pub fn create_can_tho() -> (SpaceIdentifiers, Spaces) {
    (
        SpaceIdentifiers::CanTho,
        City::new(
            SpaceIdentifiers::CanTho,
            1,
            GeographicAreas::SouthVietnam,
            vec![
                SpaceIdentifiers::Route4MekongEast,
                SpaceIdentifiers::MekongSouth,
                SpaceIdentifiers::KienHoaVinhBinh,
                SpaceIdentifiers::BaXuyen,
                SpaceIdentifiers::Route4West,
                SpaceIdentifiers::KienGiangAnXuyen,
                SpaceIdentifiers::MekongNorth,
                SpaceIdentifiers::KienPhong,
            ],
        )
        .into(),
    )
}
