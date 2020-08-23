use board::domain::loc::LoC;
use board::domain::space::Spaces;
use game_definitions::geographic_areas::GeographicAreas;
use game_definitions::space_identifiers::SpaceIdentifiers;

pub fn create_mekong_north() -> (SpaceIdentifiers, Spaces) {
    (
        SpaceIdentifiers::MekongNorth,
        LoC::new(
            SpaceIdentifiers::MekongNorth,
            GeographicAreas::SouthVietnam,
            vec![
                SpaceIdentifiers::CanTho,
                SpaceIdentifiers::KienGiangAnXuyen,
                SpaceIdentifiers::KienPhong,
                SpaceIdentifiers::TheParrotsBeak,
            ],
        )
        .into(),
    )
}
