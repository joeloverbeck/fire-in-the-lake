use board::domain::province::Province;
use board::domain::space::Spaces;
use game_definitions::geographic_areas::GeographicAreas;
use game_definitions::space_identifiers::SpaceIdentifiers;
use game_definitions::terrain_types::TerrainTypes;

pub fn create_binh_dinh() -> (SpaceIdentifiers, Spaces) {
    (
        SpaceIdentifiers::BinhDinh,
        Province::new(
            GeographicAreas::SouthVietnam,
            TerrainTypes::Highland,
            vec![
                SpaceIdentifiers::QuiNhon,
                SpaceIdentifiers::Route1NorthEast,
                SpaceIdentifiers::Route19,
                SpaceIdentifiers::PhuBonPhuYen,
                SpaceIdentifiers::Kontum,
                SpaceIdentifiers::Route14CentralNorth,
                SpaceIdentifiers::PleikuDarlac,
                SpaceIdentifiers::Route14North,
                SpaceIdentifiers::SouthernLaos,
                SpaceIdentifiers::QuangTinQuangNgai,
            ],
        )
        .into(),
    )
}
