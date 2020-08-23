use board::domain::loc::LoC;
use board::domain::space::Spaces;
use game_definitions::geographic_areas::GeographicAreas;
use game_definitions::space_identifiers::SpaceIdentifiers;

pub fn create_route_14_central() -> (SpaceIdentifiers, Spaces) {
    (
        SpaceIdentifiers::Route14Central,
        LoC::new(
            SpaceIdentifiers::Route14Central,
            GeographicAreas::SouthVietnam,
            vec![
                SpaceIdentifiers::Route21,
                SpaceIdentifiers::Route14South,
                SpaceIdentifiers::QuangDucLongKhanh,
                SpaceIdentifiers::KhanhHoa,
                SpaceIdentifiers::PhuBonPhuYen,
                SpaceIdentifiers::Kontum,
            ],
        )
        .into(),
    )
}
