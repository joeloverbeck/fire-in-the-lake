use board::domain::loc::LoC;
use board::domain::space::Spaces;
use game_definitions::geographic_areas::GeographicAreas;
use game_definitions::space_identifiers::SpaceIdentifiers;

pub fn create_mekong_south() -> (SpaceIdentifiers, Spaces) {
    (
        SpaceIdentifiers::MekongSouth,
        LoC::new(
            SpaceIdentifiers::MekongSouth,
            GeographicAreas::SouthVietnam,
            vec![
                SpaceIdentifiers::BaXuyen,
                SpaceIdentifiers::CanTho,
                SpaceIdentifiers::KienHoaVinhBinh,
            ],
        )
        .into(),
    )
}
