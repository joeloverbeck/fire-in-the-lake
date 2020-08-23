use board::domain::city::City;
use board::domain::space::Spaces;
use game_definitions::geographic_areas::GeographicAreas;
use game_definitions::space_identifiers::SpaceIdentifiers;

pub fn create_da_nang() -> (SpaceIdentifiers, Spaces) {
    (
        SpaceIdentifiers::DaNang,
        City::new(
            SpaceIdentifiers::DaNang,
            1,
            GeographicAreas::SouthVietnam,
            vec![
                SpaceIdentifiers::Route1NorthEast,
                SpaceIdentifiers::Route14North,
                SpaceIdentifiers::Route1North,
                SpaceIdentifiers::QuangTinQuangNgai,
                SpaceIdentifiers::QuangNam,
            ],
        )
        .into(),
    )
}
