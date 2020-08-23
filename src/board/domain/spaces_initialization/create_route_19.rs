use board::domain::loc::LoC;
use board::domain::space::Spaces;
use game_definitions::geographic_areas::GeographicAreas;
use game_definitions::space_identifiers::SpaceIdentifiers;

pub fn create_route_19() -> (SpaceIdentifiers, Spaces) {
    (
        SpaceIdentifiers::Route19,
        LoC::new(
            SpaceIdentifiers::Route19,
            GeographicAreas::SouthVietnam,
            vec![
                SpaceIdentifiers::QuiNhon,
                SpaceIdentifiers::BinhDinh,
                SpaceIdentifiers::PhuBonPhuYen,
                SpaceIdentifiers::Kontum,
            ],
        )
        .into(),
    )
}
