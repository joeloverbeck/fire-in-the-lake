use board::domain::loc::LoC;
use board::domain::space::Spaces;
use game_definitions::geographic_areas::GeographicAreas;
use game_definitions::space_identifiers::SpaceIdentifiers;

pub fn create_route_1_north() -> (SpaceIdentifiers, Spaces) {
    (
        SpaceIdentifiers::Route1North,
        LoC::new(
            SpaceIdentifiers::Route1North,
            GeographicAreas::SouthVietnam,
            vec![
                SpaceIdentifiers::Hue,
                SpaceIdentifiers::QuangTriThuaThien,
                SpaceIdentifiers::NorthVietnam,
                SpaceIdentifiers::CentralLaos,
            ],
        )
        .into(),
    )
}
