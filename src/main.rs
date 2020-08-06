
mod board;

use board::map::Map;
use board::space_identifiers::SpaceIdentifiers;

pub fn main() {
    let map = Map::new();


    let retrieved_space = map.get_space(SpaceIdentifiers::Saigon);

    println!("I retrieved a space and all I got was this lousy space: {:?}", retrieved_space);
}
