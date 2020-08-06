use std::collections::HashMap;

use board::space_identifiers::SpaceIdentifiers;
use board::space::Spaces;
use board::space::Space;
use board::province::Province;

pub struct Map {
    spaces: HashMap<SpaceIdentifiers, Spaces>
}

impl Map {
    pub fn new() -> Map {
        let mut map = Map {
            spaces: HashMap::new()
        };

        map.populate_spaces();

        map
    }

    pub fn retrieve_space(&self, space_to_retrieve: SpaceIdentifiers) -> Result<&Spaces, String> {
        print!("I just entered the retrieve_space function.");
        let retrieved_space = self.spaces.get(&space_to_retrieve);

        println!("Retrieved following region: {:?}", retrieved_space);
        
        match retrieved_space {
            Some(retrieved_space) => return Ok(retrieved_space),
            None => panic!("Couldn't find the corresponding space for {:?}. That should be impossible.", space_to_retrieve)
        }
    }

    fn populate_spaces(&mut self) {
        // Add Saigon
        let saigon = Province::new(SpaceIdentifiers::Saigon).into();

        self.spaces.insert(SpaceIdentifiers::Saigon, saigon);
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

        // Saigon, as well as many other spaces, should always exist. There's no space that you should
        // be able to retrieve that would not exist, given the immutability of the map, in which only things
        // like markers, pieces, etc. change.

        let retrieval_of_space_result = map.retrieve_space(SpaceIdentifiers::Saigon);

        if let Err(error) = retrieval_of_space_result {
            panic!(error);
        }

        if let Ok(space) = retrieval_of_space_result {

            assert_eq!(space.get_space_identifier(), SpaceIdentifiers::Saigon);

        }    

        Ok(())
    }
}