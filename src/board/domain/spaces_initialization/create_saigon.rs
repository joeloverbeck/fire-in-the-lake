use board::domain::city::City;
use board::domain::space::Spaces;
use game_definitions::geographic_area::GeographicArea;
use game_definitions::space_identifiers::SpaceIdentifiers;

pub fn create_saigon() -> (SpaceIdentifiers, Spaces) {
    (
        SpaceIdentifiers::Saigon,
        City::new(
            GeographicArea::SouthVietnam,
            vec![
                SpaceIdentifiers::KienHoaVinhBinh,
                SpaceIdentifiers::Route4MekongEast,
                SpaceIdentifiers::KienPhong,
                SpaceIdentifiers::TayNinh,
                SpaceIdentifiers::Route13,
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
