use board::domain::loc::LoC;
use board::domain::space::Spaces;
use game_definitions::geographic_areas::GeographicAreas;
use game_definitions::space_identifiers::SpaceIdentifiers;

pub fn create_route_14_central_north() -> (SpaceIdentifiers, Spaces) {
    (
        SpaceIdentifiers::Route14CentralNorth,
        LoC::new(
            SpaceIdentifiers::Route14CentralNorth,
            GeographicAreas::SouthVietnam,
            vec![
                SpaceIdentifiers::Kontum,
                SpaceIdentifiers::BinhDinh,
                SpaceIdentifiers::PleikuDarlac,
                SpaceIdentifiers::SouthernLaos,
                SpaceIdentifiers::Route14North,
            ],
        )
        .into(),
    )
}
