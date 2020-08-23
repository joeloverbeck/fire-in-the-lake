use board::domain::loc::LoC;
use board::domain::space::Spaces;
use game_definitions::geographic_areas::GeographicAreas;
use game_definitions::space_identifiers::SpaceIdentifiers;

pub fn create_route_11() -> (SpaceIdentifiers, Spaces) {
    (
        SpaceIdentifiers::Route11,
        LoC::new(
            SpaceIdentifiers::Route11,
            GeographicAreas::SouthVietnam,
            vec![
                SpaceIdentifiers::Route21,
                SpaceIdentifiers::Route20,
                SpaceIdentifiers::QuangDucLongKhanh,
                SpaceIdentifiers::BinhTuyBinhThuan,
                SpaceIdentifiers::KhanhHoa,
                SpaceIdentifiers::CamRanh,
            ],
        )
        .into(),
    )
}
