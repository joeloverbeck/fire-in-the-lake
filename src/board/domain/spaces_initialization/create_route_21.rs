use board::domain::loc::LoC;
use board::domain::space::Spaces;
use game_definitions::geographic_areas::GeographicAreas;
use game_definitions::space_identifiers::SpaceIdentifiers;

pub fn create_route_21() -> (SpaceIdentifiers, Spaces) {
    (
        SpaceIdentifiers::Route21,
        LoC::new(
            SpaceIdentifiers::Route21,
            GeographicAreas::SouthVietnam,
            vec![
                SpaceIdentifiers::Route14South,
                SpaceIdentifiers::Route14Central,
                SpaceIdentifiers::Route20,
                SpaceIdentifiers::Route11,
                SpaceIdentifiers::PleikuDarlac,
                SpaceIdentifiers::KhanhHoa,
                SpaceIdentifiers::QuangDucLongKhanh,
                SpaceIdentifiers::BinhTuyBinhThuan,
            ],
        )
        .into(),
    )
}
