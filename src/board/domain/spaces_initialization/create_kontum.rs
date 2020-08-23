use board::domain::city::City;
use board::domain::space::Spaces;
use game_definitions::geographic_areas::GeographicAreas;
use game_definitions::space_identifiers::SpaceIdentifiers;

pub fn create_kontum() -> (SpaceIdentifiers, Spaces) {
    (
        SpaceIdentifiers::Kontum,
        City::new(
            SpaceIdentifiers::Kontum,
            1,
            GeographicAreas::SouthVietnam,
            vec![
                SpaceIdentifiers::Route19,
                SpaceIdentifiers::Route14Central,
                SpaceIdentifiers::Route14CentralNorth,
                SpaceIdentifiers::PhuBonPhuYen,
                SpaceIdentifiers::PleikuDarlac,
                SpaceIdentifiers::BinhDinh,
            ],
        )
        .into(),
    )
}
