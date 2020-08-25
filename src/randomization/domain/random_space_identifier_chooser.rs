use game_definitions::space_identifiers::SpaceIdentifiers;
use randomization::controllers::randomization_controller_trait::RandomizationControllerTrait;
use randomization::controllers::randomization_controller_trait::RandomizationControllers;
use std::collections::HashMap;

pub struct RandomSpaceIdentifierChooser {
    ordered_spaces: HashMap<u8, (SpaceIdentifiers, u8)>,
}
impl Default for RandomSpaceIdentifierChooser {
    fn default() -> Self {
        Self::new()
    }
}

impl RandomSpaceIdentifierChooser {
    pub fn new() -> RandomSpaceIdentifierChooser {
        RandomSpaceIdentifierChooser {
            ordered_spaces: [
                (11, (SpaceIdentifiers::Saigon, 13)),
                (12, (SpaceIdentifiers::Saigon, 13)),
                (13, (SpaceIdentifiers::TheParrotsBeak, 14)),
                (14, (SpaceIdentifiers::KienHoaVinhBinh, 15)),
                (15, (SpaceIdentifiers::CanTho, 16)),
                (16, (SpaceIdentifiers::Sihanoukville, 21)),
                (21, (SpaceIdentifiers::KienGiangAnXuyen, 22)),
                (22, (SpaceIdentifiers::BaXuyen, 23)),
                (23, (SpaceIdentifiers::BinhTuyBinhThuan, 24)),
                (24, (SpaceIdentifiers::QuiNhon, 25)),
                (25, (SpaceIdentifiers::SouthernLaos, 26)),
                (26, (SpaceIdentifiers::QuangTriThuaThien, 31)),
                (31, (SpaceIdentifiers::CentralLaos, 32)),
                (32, (SpaceIdentifiers::NorthVietnam, 33)),
                (33, (SpaceIdentifiers::Hue, 36)),
                (34, (SpaceIdentifiers::Hue, 36)),
                (35, (SpaceIdentifiers::Hue, 36)),
                (36, (SpaceIdentifiers::QuangNam, 41)),
                (41, (SpaceIdentifiers::QuangTinQuangNgai, 42)),
                (42, (SpaceIdentifiers::DaNang, 43)),
                (43, (SpaceIdentifiers::BinhDinh, 45)),
                (44, (SpaceIdentifiers::BinhDinh, 45)),
                (45, (SpaceIdentifiers::PhuBonPhuYen, 46)),
                (46, (SpaceIdentifiers::Kontum, 66)),
                (51, (SpaceIdentifiers::TheFishhook, 52)),
                (52, (SpaceIdentifiers::KhanhHoa, 53)),
                (53, (SpaceIdentifiers::CamRanh, 54)),
                (54, (SpaceIdentifiers::NortheastCambodia, 55)),
                (55, (SpaceIdentifiers::TayNinh, 61)),
                (56, (SpaceIdentifiers::TayNinh, 61)),
                (61, (SpaceIdentifiers::AnLoc, 62)),
                (62, (SpaceIdentifiers::PhuocLong, 63)),
                (63, (SpaceIdentifiers::QuangDucLongKhanh, 63)),
                (64, (SpaceIdentifiers::KienPhong, 11)),
                (65, (SpaceIdentifiers::Saigon, 13)),
                (66, (SpaceIdentifiers::PleikuDarlac, 51)),
            ]
            .iter()
            .cloned()
            .collect(),
        }
    }

    pub fn choose_random_space_identifier(
        &self,
        randomization_controller: &RandomizationControllers,
    ) -> Result<&(SpaceIdentifiers, u8), String> {
        let combined_throw = (randomization_controller.roll_six_sided_die()? * 10)
            + randomization_controller.roll_six_sided_die()?;

        Ok(self.ordered_spaces.get(&combined_throw).unwrap())
    }
}
