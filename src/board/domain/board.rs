use board::domain::initialize_hashmap_of_forces::initialize_hashmap_of_forces;
use board::domain::occupable_space::OccupableSpace;
use game_definitions::control_types::ControlTypes;
use game_definitions::faction_stats::FactionStats;
use game_definitions::forces::Forces;
use game_definitions::space_identifiers::SpaceIdentifiers;
use game_definitions::support_levels::SupportLevels;

use std::collections::HashMap;

#[derive(Clone)]
pub struct Board {
    faction_stats: HashMap<FactionStats, u8>,
    out_of_play: HashMap<Forces, u8>,
    occupable_spaces: HashMap<SpaceIdentifiers, OccupableSpace>,
}

impl Default for Board {
    fn default() -> Self {
        Self::new()
    }
}

impl Board {
    pub fn new() -> Board {
        Board {
            faction_stats: [
                (FactionStats::Aid, 0),
                (FactionStats::TotalEcon, 0),
                (FactionStats::Patronage, 0),
                (FactionStats::VcResources, 0),
                (FactionStats::NvaResources, 0),
                (FactionStats::ArvnResources, 0),
                (FactionStats::SupportPlusAvailable, 0),
                (FactionStats::CoinPlusPatronage, 0),
                (FactionStats::OppositionPlusBases, 0),
                (FactionStats::NvaPlusBases, 0),
                (FactionStats::TheTrail, 0),
            ]
            .iter()
            .cloned()
            .collect(),
            out_of_play: initialize_hashmap_of_forces(),
            occupable_spaces: [
                (SpaceIdentifiers::Saigon, OccupableSpace::new()),
                (SpaceIdentifiers::Hue, OccupableSpace::new()),
                (SpaceIdentifiers::AnLoc, OccupableSpace::new()),
                (SpaceIdentifiers::KienGiangAnXuyen, OccupableSpace::new()),
                (SpaceIdentifiers::BaXuyen, OccupableSpace::new()),
                (SpaceIdentifiers::QuangNam, OccupableSpace::new()),
                (SpaceIdentifiers::BinhDinh, OccupableSpace::new()),
                (SpaceIdentifiers::CanTho, OccupableSpace::new()),
                (SpaceIdentifiers::KienPhong, OccupableSpace::new()),
                (SpaceIdentifiers::QuangTriThuaThien, OccupableSpace::new()),
                (SpaceIdentifiers::NorthVietnam, OccupableSpace::new()),
                (SpaceIdentifiers::TheParrotsBeak, OccupableSpace::new()),
                (SpaceIdentifiers::QuangTinQuangNgai, OccupableSpace::new()),
                (SpaceIdentifiers::QuangDucLongKhanh, OccupableSpace::new()),
                (SpaceIdentifiers::BinhTuyBinhThuan, OccupableSpace::new()),
                (SpaceIdentifiers::PleikuDarlac, OccupableSpace::new()),
                (SpaceIdentifiers::CentralLaos, OccupableSpace::new()),
                (SpaceIdentifiers::SouthernLaos, OccupableSpace::new()),
                (SpaceIdentifiers::QuiNhon, OccupableSpace::new()),
                (SpaceIdentifiers::KhanhHoa, OccupableSpace::new()),
                (SpaceIdentifiers::KienHoaVinhBinh, OccupableSpace::new()),
                (SpaceIdentifiers::PhuBonPhuYen, OccupableSpace::new()),
                (SpaceIdentifiers::TayNinh, OccupableSpace::new()),
                (SpaceIdentifiers::Kontum, OccupableSpace::new()),
                (SpaceIdentifiers::DaNang, OccupableSpace::new()),
                (SpaceIdentifiers::CamRanh, OccupableSpace::new()),
            ]
            .iter()
            .cloned()
            .collect(),
        }
    }

    pub fn set_faction_stat(
        &mut self,
        faction_stat: FactionStats,
        value: u8,
    ) -> Result<(), String> {
        *self.faction_stats.get_mut(&faction_stat).unwrap() = value;

        Ok(())
    }

    pub fn get_faction_stat(&self, faction_stat: FactionStats) -> Result<u8, String> {
        // Sanity checks
        if self.faction_stats.is_empty() {
            return Err("Attempted to get a faction stat from the board, but the container of faction stats was empty! The setup hasn't been done properly.".to_string());
        }

        if !self.faction_stats.contains_key(&faction_stat) {
            return Err(format!("Attempted to get the faction stat {:?}, but it couldn't be found on the board! Stored faction stats: {:?}", faction_stat, self.faction_stats));
        }

        let faction_stat_retrieval_result = self.faction_stats.get(&faction_stat);

        if let Some(value) = faction_stat_retrieval_result {
            Ok(*value)
        } else {
            Err(format!("Attempted to get the faction stat {:?} from the board, but it wasn't found there! Stored faction stats: {:?}", faction_stat, self.faction_stats))
        }
    }

    pub fn get_forces_in_out_of_play(&self, forces: Forces) -> Result<u8, String> {
        // Sanity checks
        if self.out_of_play.is_empty() {
            return Err(format!("Attempted to get the number of {:?} out of play, but the out of play container was empty! The setup for this scenario wasn't done correctly.", forces));
        }

        if !self.out_of_play.contains_key(&forces) {
            return Err(format!("Attempted to get the number of {:?} out of play, but there wasn't an entry for that type! Stored Out of Play: {:?}", forces, self.out_of_play));
        }

        Ok(*self.out_of_play.get(&forces).unwrap())
    }

    pub fn set_forces_in_out_of_play(&mut self, forces: Forces, value: u8) -> Result<(), String> {
        *self.out_of_play.get_mut(&forces).unwrap() = value;

        Ok(())
    }

    pub fn set_forces_in_space(
        &mut self,
        forces: Forces,
        value: u8,
        space: SpaceIdentifiers,
    ) -> Result<(), String> {
        let occupable_space = self.get_space_mut(space)?;

        occupable_space.set_forces(forces, value)?;

        Ok(())
    }

    pub fn increase_faction_stat(
        &mut self,
        faction_stat: &FactionStats,
        value: u8,
    ) -> Result<(), String> {
        *self.faction_stats.get_mut(&faction_stat).unwrap() += value;

        Ok(())
    }

    pub fn reduce_faction_stat(
        &mut self,
        faction_stat: &FactionStats,
        value: u8,
    ) -> Result<(), String> {
        // Sanity check
        let previous_value = **self.faction_stats.get(&faction_stat).as_ref().unwrap();

        if previous_value < value {
            panic!(
                "Was going to reduce the faction stat {:?} by {:?}, when it was {:?}!",
                faction_stat, value, previous_value
            );
        }

        *self.faction_stats.get_mut(&faction_stat).unwrap() -= value;

        // Sanity check
        if **self.faction_stats.get(&faction_stat).as_mut().unwrap() == previous_value {
            panic!(
                "Had attempted to change the value of {:?} by {:?}, but the value hasn't changed!",
                faction_stat, value
            );
        }

        Ok(())
    }

    fn get_space_mut(&mut self, space: SpaceIdentifiers) -> Result<&mut OccupableSpace, String> {
        let possible_space = self.occupable_spaces.get_mut(&space);

        if let Some(occupable_space) = possible_space {
            Ok(occupable_space)
        } else {
            Err(format!("Was requested to return a mutable reference to {:?}, but couldn't retrieve it from the collection.", space))
        }
    }

    fn get_space(&self, space: SpaceIdentifiers) -> Result<&OccupableSpace, String> {
        let possible_space = self.occupable_spaces.get(&space);

        if possible_space.is_none() {
            Err(format!(
                "Requested the space {:?}, but it wasn't in the collection! Collection: {:?}",
                space, self.occupable_spaces
            ))
        } else if let Some(occupable_space) = possible_space {
            Ok(occupable_space)
        } else {
            Err(format!(
                "Failed to get the appropriate space for {:?}.",
                space
            ))
        }
    }

    pub fn get_forces_in_space(
        &self,
        forces: Forces,
        space: SpaceIdentifiers,
    ) -> Result<u8, String> {
        let occupable_space = self.get_space(space)?;

        let number_of_forces_result = occupable_space.get_forces(forces);

        if let Err(error) = number_of_forces_result {
            Err(error)
        } else if let Ok(value) = number_of_forces_result {
            Ok(value)
        } else {
            Err(format!(
                "Requested the number of {:?} in {:?}, but couldn't retrieve the number of forces!",
                forces, space
            ))
        }
    }

    pub fn get_support_level_of_space(
        &self,
        space: SpaceIdentifiers,
    ) -> Result<SupportLevels, String> {
        let occupable_space = self.get_space(space)?;

        Ok(occupable_space.get_support_level()?)
    }

    pub fn set_support_level_of_space(
        &mut self,
        support_level: SupportLevels,
        space: SpaceIdentifiers,
    ) -> Result<(), String> {
        let occupable_space = self.get_space_mut(space)?;

        occupable_space.set_support_level(support_level)?;

        Ok(())
    }

    pub fn get_control_type_of_space(
        &self,
        space: SpaceIdentifiers,
    ) -> Result<ControlTypes, String> {
        let occupable_space = self.get_space(space)?;

        Ok(occupable_space.get_control_type()?)
    }

    pub fn set_control_type_of_space(
        &mut self,
        control_type: ControlTypes,
        space: SpaceIdentifiers,
    ) -> Result<(), String> {
        let occupable_space = self.get_space_mut(space)?;

        occupable_space.set_control_type(control_type)?;

        Ok(())
    }
}
