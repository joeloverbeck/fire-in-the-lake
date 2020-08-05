use std::collections::HashMap;

use board::space_identifiers::SpaceIdentifiers;
use board::region::Region;

pub struct Map {
    regions: HashMap<SpaceIdentifiers, Region>
}

impl Map {
    pub fn new() -> Map {
        let mut map = Map {
            regions: HashMap::new()
        };

        map.populate_regions();

        map
    }

    pub fn retrieve_region(&self, region_to_retrieve: SpaceIdentifiers) -> Result<&Region, String> {
        print!("I just entered the retrieve_region function.");
        let retrieved_region = self.regions.get(&region_to_retrieve);

        println!("Retrieved following region: {:?}", retrieved_region);
        
        match retrieved_region {
            Some(retrieved_region) => return Ok(retrieved_region),
            None => panic!("Couldn't find the corresponding region for {:?}. That should be impossible.", region_to_retrieve)
        }
    }

    fn populate_regions(&mut self) {
        // Add Saigon
        let saigon = Region::new(SpaceIdentifiers::Saigon);

        self.regions.insert(SpaceIdentifiers::Saigon, saigon);
    }
}



#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_should_be_able_to_create_map() -> Result<(), String> {
         
        let _ = Map::new();

        Ok(())
    }

    #[test]
    fn test_should_be_able_to_retrieve_saigon_from_map() -> Result<(), String> {

        let map = Map::new();

        // Saigon, as well as many other regions, should always exist. There's no region that you should
        // be able to retrieve that would not exist, given the immutability of the map, in which only things
        // like markers, pieces, etc. change.

        let retrieval_of_region_result = map.retrieve_region(SpaceIdentifiers::Saigon);

        if let Err(error) = retrieval_of_region_result {
            panic!(error);
        }

        if let Ok(region) = retrieval_of_region_result {

            assert_eq!(region.region_identifier(), SpaceIdentifiers::Saigon);

        }    

        Ok(())
    }
}