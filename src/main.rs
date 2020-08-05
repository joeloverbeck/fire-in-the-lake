
mod board;

use board::map::Map;
use board::regions::Regions;

pub fn main() {
    let map = Map::new();


    let retrieved_region = map.retrieve_region(Regions::Saigon);

    println!("I retrieved a region and all I got was this lousy region: {:?}", retrieved_region);
}
