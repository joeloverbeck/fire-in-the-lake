use board::domain::loc::LoC;
use board::domain::space::Spaces;
use game_definitions::geographic_areas::GeographicAreas;
use game_definitions::space_identifiers::SpaceIdentifiers;

pub fn create_route_1_south_east() -> (SpaceIdentifiers, Spaces) {
    (
        SpaceIdentifiers::Route1SouthEast,
        LoC::new(
            SpaceIdentifiers::Route1SouthEast,
            GeographicAreas::SouthVietnam,
            vec![
                SpaceIdentifiers::CamRanh,
                SpaceIdentifiers::KhanhHoa,
                SpaceIdentifiers::PhuBonPhuYen,
                SpaceIdentifiers::QuiNhon,
            ],
        )
        .into(),
    )
}
