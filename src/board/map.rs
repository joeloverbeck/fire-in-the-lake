use std::collections::HashMap;

use board::space::Space;
use board::space::Spaces;
use board::space_identifiers::SpaceIdentifiers;

pub struct Map {
    spaces: HashMap<SpaceIdentifiers, Spaces>,
}

impl Default for Map {
    fn default() -> Self {
        Self::new()
    }
}

impl Map {
    pub fn new() -> Map {
        Map {
            spaces: HashMap::new(),
        }
    }

    pub fn get_space_mut(
        &mut self,
        space_to_retrieve: SpaceIdentifiers,
    ) -> Result<&mut Spaces, String> {
        let retrieved_space = self.spaces.get_mut(&space_to_retrieve);

        match retrieved_space {
            Some(retrieved_space) => Ok(retrieved_space),
            None => Err(format!(
                "Couldn't find the corresponding space for {:?}. That should be impossible.",
                space_to_retrieve
            )),
        }
    }

    pub fn get_space(&self, space_to_retrieve: SpaceIdentifiers) -> Result<&Spaces, String> {
        let retrieved_space = self.spaces.get(&space_to_retrieve);

        match retrieved_space {
            Some(retrieved_space) => Ok(retrieved_space),
            None => Err(format!(
                "Couldn't find the corresponding space for {:?}. That should be impossible.",
                space_to_retrieve
            )),
        }
    }

    pub fn add_space(&mut self, space_to_add: Spaces) -> Result<(), String> {
        // Check first if the collection already contains an entry for that identifier. Should never happen
        if self
            .spaces
            .contains_key(&space_to_add.get_space_identifier())
        {
            return Err(format!("Attempted to add space {:?}, but it was already contained in the collection:\n{:?}", space_to_add, self.spaces));
        }

        self.spaces
            .insert(space_to_add.get_space_identifier(), space_to_add);

        Ok(())
    }

    pub fn get_spaces(&self) -> &HashMap<SpaceIdentifiers, Spaces> {
        &self.spaces
    }

    pub fn spaces_stored(&self) -> usize {
        self.spaces.len()
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    use board::city::City;
    use board::province::Province;

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

    #[test]
    fn test_after_adding_two_different_spaces_to_the_hashmap_there_should_be_two_entries_on_the_hashmap(
    ) -> Result<(), String> {
        let mut map = Map::new();

        let mut saigon: Spaces = City::new(SpaceIdentifiers::Saigon).into();
        let mut kien_giang_an_xuyen: Spaces =
            Province::new(SpaceIdentifiers::KienGiangAnXuyen).into();

        map.add_space(saigon)?;
        map.add_space(kien_giang_an_xuyen)?;

        assert_eq!(map.spaces_stored(), 2);

        Ok(())
    }
}
