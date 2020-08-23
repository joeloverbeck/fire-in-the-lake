use board::domain::loc::LoC;
use board::domain::space::Spaces;
use game_definitions::geographic_areas::GeographicAreas;
use game_definitions::space_identifiers::SpaceIdentifiers;

pub fn create_route_1_east() -> (SpaceIdentifiers, Spaces) {
    (
        SpaceIdentifiers::Route1East,
        LoC::new(
            SpaceIdentifiers::Route1East,
            GeographicAreas::SouthVietnam,
            vec![
                SpaceIdentifiers::QuiNhon,
                SpaceIdentifiers::BinhDinh,
                SpaceIdentifiers::QuangTinQuangNgai,
                SpaceIdentifiers::DaNang,
            ],
        )
        .into(),
    )
}
