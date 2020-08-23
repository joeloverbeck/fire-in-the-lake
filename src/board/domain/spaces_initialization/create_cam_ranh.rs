use board::domain::city::City;
use board::domain::space::Spaces;
use game_definitions::geographic_areas::GeographicAreas;
use game_definitions::space_identifiers::SpaceIdentifiers;

pub fn create_cam_ranh() -> (SpaceIdentifiers, Spaces) {
    (
        SpaceIdentifiers::CamRanh,
        City::new(
            SpaceIdentifiers::CamRanh,
            1,
            GeographicAreas::SouthVietnam,
            vec![
                SpaceIdentifiers::Route1South,
                SpaceIdentifiers::Route1SouthEast,
                SpaceIdentifiers::Route11,
                SpaceIdentifiers::BinhTuyBinhThuan,
                SpaceIdentifiers::KhanhHoa,
            ],
        )
        .into(),
    )
}
