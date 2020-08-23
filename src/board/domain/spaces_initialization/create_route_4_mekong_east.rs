use board::domain::loc::LoC;
use board::domain::space::Spaces;
use game_definitions::geographic_areas::GeographicAreas;
use game_definitions::space_identifiers::SpaceIdentifiers;

pub fn create_route_4_mekong_east() -> (SpaceIdentifiers, Spaces) {
    (
        SpaceIdentifiers::Route4MekongEast,
        LoC::new(
            SpaceIdentifiers::Route4MekongEast,
            GeographicAreas::SouthVietnam,
            vec![
                SpaceIdentifiers::Saigon,
                SpaceIdentifiers::KienPhong,
                SpaceIdentifiers::KienHoaVinhBinh,
                SpaceIdentifiers::CanTho,
            ],
        )
        .into(),
    )
}
