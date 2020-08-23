use board::domain::loc::LoC;
use board::domain::space::Spaces;
use game_definitions::geographic_areas::GeographicAreas;
use game_definitions::space_identifiers::SpaceIdentifiers;

pub fn create_route_14_north() -> (SpaceIdentifiers, Spaces) {
    (
        SpaceIdentifiers::Route14North,
        LoC::new(
            SpaceIdentifiers::Route14North,
            GeographicAreas::SouthVietnam,
            vec![
                SpaceIdentifiers::DaNang,
                SpaceIdentifiers::QuangNam,
                SpaceIdentifiers::QuangTinQuangNgai,
                SpaceIdentifiers::Route14CentralNorth,
                SpaceIdentifiers::Route14CentralNorth,
                SpaceIdentifiers::SouthernLaos,
                SpaceIdentifiers::BinhDinh,
                SpaceIdentifiers::PleikuDarlac,
            ],
        )
        .into(),
    )
}
