use board::domain::loc::LoC;
use board::domain::space::Spaces;
use game_definitions::geographic_areas::GeographicAreas;
use game_definitions::space_identifiers::SpaceIdentifiers;

pub fn create_route_4_west() -> (SpaceIdentifiers, Spaces) {
    (
        SpaceIdentifiers::Route4West,
        LoC::new(
            SpaceIdentifiers::Route4West,
            GeographicAreas::SouthVietnam,
            vec![
                SpaceIdentifiers::KienGiangAnXuyen,
                SpaceIdentifiers::BaXuyen,
                SpaceIdentifiers::CanTho,
            ],
        )
        .into(),
    )
}
