
use board::map::Map;
use board::space::Spaces;
use board::space::Space;
use board::province::Province;
use board::line_of_communication::LineOfCommunication;
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

        // Create Quan Nam
        let mut quang_nam: Spaces = Province::new(SpaceIdentifiers::QuangNam).into();
        quang_nam.set_population_value(1);
        quang_nam.set_terrain_type(TerrainTypes::Highland);

        // Create Route 4
        let mut route4: Spaces = LineOfCommunication::new(SpaceIdentifiers::Route4).into();
        route4.set_population_value(0);
        route4.set_terrain_type(TerrainTypes::LoC);


        // Create Can Tho
        let mut can_tho: Spaces = City::new(SpaceIdentifiers::CanTho).into();
        can_tho.set_population_value(1);
        can_tho.set_terrain_type(TerrainTypes::City);

        let mut new_map = Map::new();

        new_map.add_space(saigon);
        new_map.add_space(kien_giang_an_xuyen);
        new_map.add_space(ba_xuyen);
        new_map.add_space(quang_nam);
        new_map.add_space(route4);
        new_map.add_space(can_tho);

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

        let expected_population_value = 2;

        assert_eq!(kien_giang_an_xuyen.get_population_value(), 2, "The population value of {:?} should have been {:?}, but was {:?}", SpaceIdentifiers::KienGiangAnXuyen, expected_population_value, kien_giang_an_xuyen.get_population_value());
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

        let expected_population_value = 1;

        assert_eq!(ba_xuyen.get_population_value(), 1, "The population value of {:?} should have been {:?}, but was {:?}", ba_xuyen.get_space_identifier(), expected_population_value, ba_xuyen.get_population_value());
        assert_eq!(ba_xuyen.get_control(), Controls::Uncontrolled);
        assert_eq!(ba_xuyen.get_terrain_type(), TerrainTypes::Lowland);

        Ok(())
    }

    #[test]
    fn test_upon_building_default_map_quang_nam_should_have_expected_initial_values() -> Result<(), String> {
        let map_builder = MapBuilder::new();

        let built_map = map_builder.build_initial_map();

        // Test Quang nam
        let quang_nam: &Spaces = retrieve_space(&built_map, SpaceIdentifiers::QuangNam);

        let expected_population_value = 1;

        assert_eq!(quang_nam.get_population_value(), 1, "The population value of {:?} should have been {:?}, but was {:?}", quang_nam.get_space_identifier(), expected_population_value, quang_nam.get_population_value());
        assert_eq!(quang_nam.get_control(), Controls::Uncontrolled);
        assert_eq!(quang_nam.get_terrain_type(), TerrainTypes::Highland);

        Ok(())
    }

    #[test]
    fn test_upon_building_default_map_route4_should_have_expected_initial_values() -> Result<(), String> {
        let map_builder = MapBuilder::new();

        let built_map = map_builder.build_initial_map();

        // Test Route 4
        let route4: &Spaces = retrieve_space(&built_map, SpaceIdentifiers::Route4);

        let expected_population_value = 0;

        assert_eq!(route4.get_population_value(), 0, "The population value of {:?} should have been {:?}, but was {:?}", route4.get_space_identifier(), expected_population_value, route4.get_population_value());
        assert_eq!(route4.get_control(), Controls::Uncontrolled);
        assert_eq!(route4.get_terrain_type(), TerrainTypes::LoC);

        Ok(())
    }
   
    #[test]
    fn test_upon_building_default_map_can_tho_should_have_expected_initial_values() -> Result<(), String> {
        let map_builder = MapBuilder::new();

        let built_map = map_builder.build_initial_map();

        // Test Can Tho
        let can_tho: &Spaces = retrieve_space(&built_map, SpaceIdentifiers::CanTho);

        let expected_population_value = 1;

        assert_eq!(can_tho.get_population_value(), 1, "The population value of {:?} should have been {:?}, but was {:?}", can_tho.get_space_identifier(), expected_population_value, can_tho.get_population_value());
        assert_eq!(can_tho.get_control(), Controls::Uncontrolled);
        assert_eq!(can_tho.get_terrain_type(), TerrainTypes::City);

        Ok(())
    }
}