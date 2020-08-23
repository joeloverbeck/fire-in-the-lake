use board::domain::loc::LoC;
use board::domain::space::Spaces;
use game_definitions::geographic_areas::GeographicAreas;
use game_definitions::space_identifiers::SpaceIdentifiers;

pub fn create_route_1_south() -> (SpaceIdentifiers, Spaces) {
    (
        SpaceIdentifiers::Route1South,
        LoC::new(
            SpaceIdentifiers::Route1South,
            GeographicAreas::SouthVietnam,
            vec![
                SpaceIdentifiers::Saigon,
                SpaceIdentifiers::BinhTuyBinhThuan,
                SpaceIdentifiers::CamRanh,
            ],
        )
        .into(),
    )
}
