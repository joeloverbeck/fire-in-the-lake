use board::map::Map;
use board::space::Space;
use board::space_identifiers::SpaceIdentifiers;

pub fn move_troops_from_space_to_space(
    troop_type: &str,
    amount: u8,
    from: SpaceIdentifiers,
    to: SpaceIdentifiers,
    map: &mut Map,
) -> Result<(), String> {
    // Gotta remove the specific amount of troops from the "from" space, and add them to the "to" space. But we have to
    // test the control of the place.
    let from_space = map.get_space_mut(from).unwrap();

    if troop_type == "underground_nva_guerrillas" {
        if from_space.get_number_of_underground_nva_guerrillas() < amount {
            panic!("Was going to move {:?} of underground nva guerrillas from the space '{:?}', but in the space there were only {:?}. Space: {:?}", amount, from, from_space.get_number_of_underground_nva_guerrillas(), from_space);
        }

        from_space.set_number_of_underground_nva_guerrillas(
            from_space.get_number_of_underground_nva_guerrillas() - amount,
        );

        let to_space = map.get_space_mut(to).unwrap();
        to_space.set_number_of_underground_nva_guerrillas(
            to_space.get_number_of_underground_nva_guerrillas() + amount,
        );
    } else {
        todo!()
    }

    // Should trigger the adjust control calculations in the involved space.
    let to_space = map.get_space_mut(to).unwrap();
    to_space.adjust_control();

    Ok(())
}
