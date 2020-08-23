use board::domain::city::City;
use board::domain::space::Spaces;
use game_definitions::geographic_areas::GeographicAreas;
use game_definitions::space_identifiers::SpaceIdentifiers;

pub fn create_qui_nhon() -> (SpaceIdentifiers, Spaces) {
    (
        SpaceIdentifiers::QuiNhon,
        City::new(
            SpaceIdentifiers::QuiNhon,
            1,
            GeographicAreas::SouthVietnam,
            vec![
                SpaceIdentifiers::Route19,
                SpaceIdentifiers::Route1SouthEast,
                SpaceIdentifiers::Route1East,
                SpaceIdentifiers::PhuBonPhuYen,
                SpaceIdentifiers::BinhDinh,
            ],
        )
        .into(),
    )
}
