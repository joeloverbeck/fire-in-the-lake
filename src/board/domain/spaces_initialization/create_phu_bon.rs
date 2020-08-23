use board::domain::province::Province;
use board::domain::space::Spaces;
use game_definitions::geographic_areas::GeographicAreas;
use game_definitions::space_identifiers::SpaceIdentifiers;
use game_definitions::terrain_types::TerrainTypes;

pub fn create_phu_bon() -> (SpaceIdentifiers, Spaces) {
    (
        SpaceIdentifiers::PhuBonPhuYen,
        Province::new(
            SpaceIdentifiers::PhuBonPhuYen,
            1,
            GeographicAreas::SouthVietnam,
            TerrainTypes::Lowland,
            vec![
                SpaceIdentifiers::Route19,
                SpaceIdentifiers::Route1SouthEast,
                SpaceIdentifiers::Route14Central,
                SpaceIdentifiers::KhanhHoa,
                SpaceIdentifiers::PleikuDarlac,
                SpaceIdentifiers::Kontum,
                SpaceIdentifiers::BinhDinh,
                SpaceIdentifiers::QuiNhon,
            ],
        )
        .into(),
    )
}
