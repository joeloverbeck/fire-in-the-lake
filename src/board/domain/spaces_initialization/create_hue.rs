use board::domain::city::City;
use board::domain::space::Spaces;
use game_definitions::geographic_areas::GeographicAreas;
use game_definitions::space_identifiers::SpaceIdentifiers;

pub fn create_hue() -> (SpaceIdentifiers, Spaces) {
    (
        SpaceIdentifiers::Hue,
        City::new(
            GeographicAreas::SouthVietnam,
            vec![
                SpaceIdentifiers::Route1North,
                SpaceIdentifiers::QuangTriThuaThien,
            ],
        )
        .into(),
    )
}
