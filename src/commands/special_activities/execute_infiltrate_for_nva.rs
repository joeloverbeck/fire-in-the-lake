
use commands::atomic::shift_support_of_space::shift_support_of_space;
use board::available_forces::AvailableForces;
use decision_making::interpretation::operation_instructions::infiltrate_instructions::InfiltrateInstructions;
use board::get_space_from_map::get_space_from_map;
use board::map::Map;
use board::space::Space;
use board::space_identifiers::SpaceIdentifiers;
use board::track::Track;

pub fn execute_infiltrate_for_nva(
    infiltrate_instructions: &Vec<InfiltrateInstructions>,
    map: &mut Map,
    track: &mut Track,
    available_forces: &mut AvailableForces,
) -> Result<(), String> {
    // **Rules**
    // Place: trail + number of bases
    // Exchange: All NVA Guerrillas get exchanged for NVA Troops
    // Replace: Any VC piece gets replaced by an underground NVA guerrilla, and support goes up one level.
    for entry in infiltrate_instructions.iter() {
        if entry.get_type_of_infiltrate() == "place"{
            let nva_troops_to_place = track.get_trail() +  map.get_space(entry.get_space()).unwrap().get_number_of_nva_bases();

            if available_forces.get_amount_of_nva_troops() < nva_troops_to_place{
                panic!("Case not contemplated: would place more nva troops from available than there were.");
            }

            let previous_number_of_nva_troops_in_space = map.get_space(entry.get_space()).unwrap().get_number_of_nva_troops();

            map.get_space_mut(entry.get_space()).unwrap().set_number_of_nva_troops(previous_number_of_nva_troops_in_space + nva_troops_to_place);
        }
        else if entry.get_type_of_infiltrate() == "exchange"{
            let current_amount_of_nva_guerrillas = map.get_space_mut(entry.get_space()).unwrap().get_number_of_underground_nva_guerrillas();

            map.get_space_mut(entry.get_space()).unwrap().set_number_of_underground_nva_guerrillas(0);

            let previous_number_of_nva_troops_in_space = map.get_space(entry.get_space()).unwrap().get_number_of_nva_troops();

            if current_amount_of_nva_guerrillas > available_forces.get_amount_of_nva_troops() {
                panic!("Case not contemplated: was going to exchange more nva underground guerrillas than there are available nva troops.");
            }

            map.get_space_mut(entry.get_space()).unwrap().set_number_of_nva_troops(previous_number_of_nva_troops_in_space + current_amount_of_nva_guerrillas);
        }
        else if entry.get_type_of_infiltrate() == "replace"{
            // Sanity check: there must be a vc piece specified.

            if let None = entry.get_vc_piece() {
                panic!("Attempted to replace a chosen VC piece as part of infiltrate for NVA, but it had chosen no VC piece! {:?}", entry);
            }

            if let Some(vc_piece) = entry.get_vc_piece() {
                if vc_piece == "underground_vc_guerrilla"{
                    if map.get_space_mut(entry.get_space()).unwrap().get_number_of_underground_vc_guerrillas() < 1 {
                        panic!("Attempted to replace an underground vc guerrilla as part of replace for infiltrate for NVA, but there were no underground vc guerrillas.");
                    }

                    let previous_number_of_underground_vc_guerrillas = map.get_space(entry.get_space()).unwrap().get_number_of_underground_vc_guerrillas();

                    map.get_space_mut(entry.get_space()).unwrap().set_number_of_underground_vc_guerrillas(previous_number_of_underground_vc_guerrillas - 1);

                    let previous_number_of_nva_guerrillas = map.get_space(entry.get_space()).unwrap().get_number_of_underground_nva_guerrillas();

                    map.get_space_mut(entry.get_space()).unwrap().set_number_of_underground_nva_guerrillas(previous_number_of_nva_guerrillas + 1);

                    // Support should go up.
                    shift_support_of_space(map.get_space_mut(entry.get_space()).unwrap(), 1);
                }
                else{
                    panic!("Replace not contemplated for vc piece: {:?}", entry.get_vc_piece());
                }
            }
        }
        else{
            panic!("Not implemented: {:?}", entry);
        }

        // Check control of this space.
        map.get_space_mut(entry.get_space()).unwrap().adjust_control();
        track.adjust_nva_victory_marker(map);
    }

    Ok(())
}
