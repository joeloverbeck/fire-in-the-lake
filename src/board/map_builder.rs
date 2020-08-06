
use board::map::Map;
use board::space::Spaces;
use board::space::Space;
use board::province::Province;
use board::space_identifiers::SpaceIdentifiers;
use board::controls::Controls;

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
        let mut saigon: Spaces = Province::new(SpaceIdentifiers::Saigon).into();
        saigon.set_control(Controls::Neutral);
        saigon.set_population_value(6);

        let mut new_map = Map::new();

        new_map.add_space(saigon);

        new_map
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use board::space_identifiers::SpaceIdentifiers;

    #[test]
    fn test_upon_building_default_map_saigon_should_have_expected_initial_values() -> Result<(), String> {

        let map_builder = MapBuilder::new();

        let built_map = map_builder.build_initial_map();

        let retrieved_space = built_map.get_space(SpaceIdentifiers::Saigon);

        let saigon: &Spaces;

        match retrieved_space {
            Err(error) => panic!("Was unable to retrieve Saigon, although it should have been created."),
            Ok(retrieved_saigon) => saigon = retrieved_saigon
        }
        
        assert_eq!(saigon.get_population_value(), 6, "The population value of Saigon should have been six, but was {:?}", saigon.get_population_value());

        Ok(())
    }

}