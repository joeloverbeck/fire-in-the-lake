use board::domain::province::Province;
use board::domain::space::Spaces;
use game_definitions::geographic_areas::GeographicAreas;
use game_definitions::space_identifiers::SpaceIdentifiers;
use game_definitions::terrain_types::TerrainTypes;

pub fn create_pleiku() -> (SpaceIdentifiers, Spaces) {
    (
        SpaceIdentifiers::PleikuDarlac,
        Province::new(
            SpaceIdentifiers::PleikuDarlac,
            1,
            GeographicAreas::SouthVietnam,
            TerrainTypes::Highland,
            vec![
                SpaceIdentifiers::Route14South,
                SpaceIdentifiers::Route21,
                SpaceIdentifiers::Route14Central,
                SpaceIdentifiers::Route14CentralNorth,
                SpaceIdentifiers::Route14North,
                SpaceIdentifiers::QuangDucLongKhanh,
                SpaceIdentifiers::TheFishhook,
                SpaceIdentifiers::NortheastCambodia,
                SpaceIdentifiers::SouthernLaos,
                SpaceIdentifiers::Kontum,
                SpaceIdentifiers::BinhDinh,
                SpaceIdentifiers::PhuBonPhuYen,
                SpaceIdentifiers::KhanhHoa,
            ],
        )
        .into(),
    )
}
