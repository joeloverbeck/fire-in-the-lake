use board::domain::city::City;
use board::domain::space::Spaces;
use game_definitions::geographic_areas::GeographicAreas;
use game_definitions::space_identifiers::SpaceIdentifiers;

pub fn create_the_fishhook() -> (SpaceIdentifiers, Spaces) {
    (
        SpaceIdentifiers::TheFishhook,
        City::new(
            GeographicAreas::Cambodia,
            vec![
                SpaceIdentifiers::NortheastCambodia,
                SpaceIdentifiers::PleikuDarlac,
                SpaceIdentifiers::QuangDucLongKhanh,
                SpaceIdentifiers::PhuocLong,
                SpaceIdentifiers::AnLoc,
                SpaceIdentifiers::TayNinh,
                SpaceIdentifiers::TheParrotsBeak,
            ],
        )
        .into(),
    )
}
