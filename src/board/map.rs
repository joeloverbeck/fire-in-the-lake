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

        map
    }

    pub fn get_space(&self, space_to_retrieve: SpaceIdentifiers) -> Result<&Spaces, String> {
        print!("I just entered the retrieve_space function.");
        let retrieved_space = self.spaces.get(&space_to_retrieve);

        println!("Retrieved following region: {:?}", retrieved_space);
        
        match retrieved_space {
            Some(retrieved_space) => return Ok(retrieved_space),
            None => panic!("Couldn't find the corresponding space for {:?}. That should be impossible.", space_to_retrieve)
        }
    }

    pub fn add_space(&mut self, space_to_add: Spaces) {
        self.spaces.insert(space_to_add.get_space_identifier(), space_to_add);
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

        let mut map = Map::new();

        // Saigon, as well as many other spaces, should always exist. There's no space that you should
        // be able to retrieve that would not exist, given the immutability of the map, in which only things
        // like markers, pieces, etc. change.
        let mut saigon: Spaces = Province::new(SpaceIdentifiers::Saigon).into();

        map.add_space(saigon);

        let retrieval_of_space_result = map.get_space(SpaceIdentifiers::Saigon);

        if let Err(error) = retrieval_of_space_result {
            panic!(error);
        }

        if let Ok(space) = retrieval_of_space_result {

            assert_eq!(space.get_space_identifier(), SpaceIdentifiers::Saigon);

        }    

        Ok(())
    }
}