use board::space_identifiers::SpaceIdentifiers;
use std::collections::HashMap;

use board::city::City;
use board::province::Province;
use board::line_of_communication::LineOfCommunication;

extern crate enum_dispatch;
use self::enum_dispatch::enum_dispatch;


#[enum_dispatch]
pub trait Space {
    fn get_space_identifier(&self) -> SpaceIdentifiers;
}

#[enum_dispatch(Space)]
enum Spaces {
    City,
    Province,
    LineOfCommunication
}


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_should_be_able_to_create_a_hashmap_of_all_possible_spaces() -> Result<(), String> {
         
        let city = City::new().into();
        let province = Province::new().into();
        let line_of_communication = LineOfCommunication::new().into();

        let mut registered_spaces: HashMap<SpaceIdentifiers, Spaces> = HashMap::new();

        registered_spaces.insert(SpaceIdentifiers::Saigon, city);
        registered_spaces.insert(SpaceIdentifiers::Saigon, province);
        registered_spaces.insert(SpaceIdentifiers::Saigon, line_of_communication);

        if let Some(retrieved_space) = registered_spaces.get(&SpaceIdentifiers::Saigon){
            assert!(retrieved_space.get_space_identifier() == SpaceIdentifiers::Saigon);
        }
        else{
            return Err(String::from("Could not retrieve the inserted space from the hashmap"));
        }

        Ok(())
    }

}