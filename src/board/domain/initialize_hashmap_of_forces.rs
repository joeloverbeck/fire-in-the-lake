use game_definitions::forces::Forces;
use std::collections::HashMap;

pub fn initialize_hashmap_of_forces() -> HashMap<Forces, u8> {
    [
        (Forces::UsBase, 0),
        (Forces::ActiveArvnRanger, 0),
        (Forces::ActiveNvaGuerrilla, 0),
        (Forces::ActiveUsIrregular, 0),
        (Forces::ActiveVcGuerrilla, 0),
        (Forces::ArvnBase, 0),
        (Forces::ArvnPolice, 0),
        (Forces::ArvnTroop, 0),
        (Forces::NvaBase, 0),
        (Forces::TunneledNvaBase, 0),
        (Forces::TunneledVcBase, 0),
        (Forces::UndergroundArvnRanger, 0),
        (Forces::UndergroundNvaGuerrilla, 0),
        (Forces::UndergroundUsIrregular, 0),
        (Forces::UndergroundVcGuerrilla, 0),
        (Forces::UsBase, 0),
        (Forces::UsTroop, 0),
        (Forces::VcBase, 0),
    ]
    .iter()
    .cloned()
    .collect()
}
