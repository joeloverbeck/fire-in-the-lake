use board::domain::loc::LoC;
use board::domain::space::Spaces;
use game_definitions::geographic_areas::GeographicAreas;
use game_definitions::space_identifiers::SpaceIdentifiers;

pub fn create_route_14_south() -> (SpaceIdentifiers, Spaces) {
    (
        SpaceIdentifiers::Route14South,
        LoC::new(
            SpaceIdentifiers::Route14South,
            GeographicAreas::SouthVietnam,
            vec![
                SpaceIdentifiers::Route21,
                SpaceIdentifiers::Route14Central,
                SpaceIdentifiers::PleikuDarlac,
                SpaceIdentifiers::KhanhHoa,
                SpaceIdentifiers::QuangDucLongKhanh,
                SpaceIdentifiers::TheFishhook,
                SpaceIdentifiers::PhuocLong,
                SpaceIdentifiers::AnLoc,
                SpaceIdentifiers::TayNinh,
                SpaceIdentifiers::Saigon,
            ],
        )
        .into(),
    )
}
