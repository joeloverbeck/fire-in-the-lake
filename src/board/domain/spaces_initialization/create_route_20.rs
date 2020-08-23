use board::domain::loc::LoC;
use board::domain::space::Spaces;
use game_definitions::geographic_areas::GeographicAreas;
use game_definitions::space_identifiers::SpaceIdentifiers;

pub fn create_route_20() -> (SpaceIdentifiers, Spaces) {
    (
        SpaceIdentifiers::Route20,
        LoC::new(
            SpaceIdentifiers::Route20,
            GeographicAreas::SouthVietnam,
            vec![
                SpaceIdentifiers::Route21,
                SpaceIdentifiers::Route11,
                SpaceIdentifiers::KhanhHoa,
                SpaceIdentifiers::BinhTuyBinhThuan,
                SpaceIdentifiers::QuangDucLongKhanh,
                SpaceIdentifiers::Saigon,
            ],
        )
        .into(),
    )
}
