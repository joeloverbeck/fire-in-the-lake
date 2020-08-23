use board::domain::loc::LoC;
use board::domain::space::Spaces;
use game_definitions::geographic_areas::GeographicAreas;
use game_definitions::space_identifiers::SpaceIdentifiers;

pub fn create_route_1_north_east() -> (SpaceIdentifiers, Spaces) {
    (
        SpaceIdentifiers::Route1NorthEast,
        LoC::new(
            SpaceIdentifiers::Route1NorthEast,
            GeographicAreas::SouthVietnam,
            vec![
                SpaceIdentifiers::DaNang,
                SpaceIdentifiers::QuangNam,
                SpaceIdentifiers::QuangTriThuaThien,
                SpaceIdentifiers::Hue,
            ],
        )
        .into(),
    )
}
