use board::domain::city::City;
use board::domain::space::Spaces;
use game_definitions::geographic_area::GeographicArea;
use game_definitions::space_identifiers::SpaceIdentifiers;

pub fn create_hue() -> (SpaceIdentifiers, Spaces) {
    (
        SpaceIdentifiers::Hue,
        City::new(
            GeographicArea::SouthVietnam,
            vec![
                SpaceIdentifiers::Route1North,
                SpaceIdentifiers::QuangTriThuaThien,
            ],
        )
        .into(),
    )
}
