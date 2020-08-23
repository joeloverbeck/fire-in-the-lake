use board::domain::city::City;
use board::domain::space::Spaces;
use game_definitions::geographic_areas::GeographicAreas;
use game_definitions::space_identifiers::SpaceIdentifiers;

pub fn create_saigon() -> (SpaceIdentifiers, Spaces) {
    (
        SpaceIdentifiers::Saigon,
        City::new(
            SpaceIdentifiers::Saigon,
            6,
            GeographicAreas::SouthVietnam,
            vec![
                SpaceIdentifiers::KienHoaVinhBinh,
                SpaceIdentifiers::Route4MekongEast,
                SpaceIdentifiers::KienPhong,
                SpaceIdentifiers::TayNinh,
                SpaceIdentifiers::Route14South,
                SpaceIdentifiers::QuangDucLongKhanh,
                SpaceIdentifiers::BinhTuyBinhThuan,
                SpaceIdentifiers::Route20,
                SpaceIdentifiers::Route1South,
            ],
        )
        .into(),
    )
}
