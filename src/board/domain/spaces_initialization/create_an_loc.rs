use board::domain::city::City;
use board::domain::space::Spaces;
use game_definitions::geographic_area::GeographicArea;
use game_definitions::space_identifiers::SpaceIdentifiers;

pub fn create_an_loc() -> (SpaceIdentifiers, Spaces) {
    (
        SpaceIdentifiers::AnLoc,
        City::new(
            GeographicArea::SouthVietnam,
            vec![
                SpaceIdentifiers::TheFishhook,
                SpaceIdentifiers::PhuocLong,
                SpaceIdentifiers::Route13,
                SpaceIdentifiers::TayNinh,
            ],
        )
        .into(),
    )
}
