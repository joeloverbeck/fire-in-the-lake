
use board::map::Map;
use board::space::Spaces;
use board::space::Space;
use board::province::Province;
use board::city::City;
use board::space_identifiers::SpaceIdentifiers;
use board::controls::Controls;
use board::terrain_types::TerrainTypes;

#[derive(Debug)]
struct MapBuilder {

}

impl MapBuilder {

    fn new() -> MapBuilder {
        MapBuilder {

        }
    }

    fn build_initial_map (&self) -> Map {
        // Create Saigon
        let mut saigon: Spaces = City::new(SpaceIdentifiers::Saigon).into();
        saigon.set_population_value(6);

        // Create Kien Giang-An Xuyen
        let mut kien_giang_an_xuyen: Spaces = Province::new(SpaceIdentifiers::KienGiangAnXuyen).into();
        kien_giang_an_xuyen.set_population_value(2);

        // Create Ba Xuyen
        let mut ba_xuyen: Spaces = Province::new(SpaceIdentifiers::BaXuyen).into();
        ba_xuyen.set_population_value(1);

        let mut new_map = Map::new();

        new_map.add_space(saigon);
        new_map.add_space(kien_giang_an_xuyen);
        new_map.add_space(ba_xuyen);

        new_map
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use board::space_identifiers::SpaceIdentifiers;

    fn retrieve_space(map: &Map, space_to_retrieve: SpaceIdentifiers) -> &Spaces {
        println!("Will try to retrieve space {:?}", space_to_retrieve);
        let mut retrieved_space = map.get_space(space_to_retrieve);

        let space_to_return: &Spaces;

        match retrieved_space {
            Err(error) => panic!("Was unable to retrieve {:?}, although it should have been created. Error received: {:?}", space_to_retrieve, error),
            Ok(retrieved_space) => space_to_return = retrieved_space
        }

        space_to_return
    }

    #[test]
    fn test_upon_building_default_map_saigon_should_have_expected_initial_values() -> Result<(), String> {

        let map_builder = MapBuilder::new();

        let built_map = map_builder.build_initial_map();


        // Test Saigon
        let saigon: &Spaces = retrieve_space(&built_map, SpaceIdentifiers::Saigon);
        
        assert_eq!(saigon.get_population_value(), 6, "The population value of {:?} should have been six, but was {:?}", SpaceIdentifiers::Saigon, saigon.get_population_value());
        assert_eq!(saigon.get_control(), Controls::Uncontrolled);
        assert_eq!(saigon.get_terrain_type(), TerrainTypes::City);

        Ok(())
    }

    #[test]
    fn test_upon_building_default_map_kien_gian_an_xuyen_should_have_expected_initial_values() -> Result<(), String> {
        let map_builder = MapBuilder::new();

        let built_map = map_builder.build_initial_map();

        // Test Kien Giang-An Xuyen
        let kien_giang_an_xuyen: &Spaces = retrieve_space(&built_map, SpaceIdentifiers::KienGiangAnXuyen);

        assert_eq!(kien_giang_an_xuyen.get_population_value(), 2, "The population value of {:?} should have been two, but was {:?}", SpaceIdentifiers::KienGiangAnXuyen, kien_giang_an_xuyen.get_population_value());
        assert_eq!(kien_giang_an_xuyen.get_control(), Controls::Uncontrolled);
        assert_eq!(kien_giang_an_xuyen.get_terrain_type(), TerrainTypes::Lowland);

        Ok(())
    }

    #[test]
    fn test_upon_building_default_map_ba_xuyen_should_have_expected_initial_values() -> Result<(), String> {
        let map_builder = MapBuilder::new();

        let built_map = map_builder.build_initial_map();

        // Test Ba Xuyen
        let ba_xuyen: &Spaces = retrieve_space(&built_map, SpaceIdentifiers::BaXuyen);

        assert_eq!(ba_xuyen.get_population_value(), 1, "The population value of {:?} should have been two, but was {:?}", SpaceIdentifiers::KienGiangAnXuyen, ba_xuyen.get_population_value());
        assert_eq!(ba_xuyen.get_control(), Controls::Uncontrolled);
        assert_eq!(ba_xuyen.get_terrain_type(), TerrainTypes::Lowland);

        Ok(())
    }
   
}