use board::domain::space::Space;
use board::domain::space::Spaces;
use game_definitions::factions::Factions;
use players::domain::forces_mutation::ForcesMutation;
use players::domain::mutations_production::produce_mutations_for_terror_at_space::produce_mutations_for_terror_at_space;
use players::domain::space_mutation::SpaceMutation;
use std::collections::VecDeque;

pub fn produce_as_many_terror_mutations_for_nva_as_possible(
    nva_resources: &mut u8,
    best_terror_targets: &mut VecDeque<&Spaces>,
    secondary_terror_targets: &mut VecDeque<&Spaces>,
    forces_mutations: &mut Vec<ForcesMutation>,
    space_mutations: &mut Vec<SpaceMutation>,
) -> Result<(), String> {
    while *nva_resources > 0 {
        // If there are no targets, just break.
        if best_terror_targets.is_empty() && secondary_terror_targets.is_empty() {
            break;
        }

        // Always will attempt to pick the best target in a tie.
        if best_terror_targets.is_empty() && !secondary_terror_targets.is_empty() {
            let terror_target = secondary_terror_targets.pop_front().unwrap();

            produce_mutations_for_terror_at_space(
                terror_target,
                Factions::NVA,
                forces_mutations,
                space_mutations,
            )?;
        }

        if !best_terror_targets.is_empty() && secondary_terror_targets.is_empty() {
            let terror_target = best_terror_targets.pop_front().unwrap();

            produce_mutations_for_terror_at_space(
                terror_target,
                Factions::NVA,
                forces_mutations,
                space_mutations,
            )?;
        }

        // If there are entries in both deques, then get the one that has the highest total support. If equal, get
        // the one that has a NVA Troop in it.
        if !best_terror_targets.is_empty() && !secondary_terror_targets.is_empty() {
            if best_terror_targets[0].get_total_support()?
                > secondary_terror_targets[0].get_total_support()?
                || best_terror_targets[0].get_total_support()?
                    == secondary_terror_targets[0].get_total_support()?
            {
                let terror_target = best_terror_targets.pop_front().unwrap();

                produce_mutations_for_terror_at_space(
                    terror_target,
                    Factions::NVA,
                    forces_mutations,
                    space_mutations,
                )?;
            } else if best_terror_targets[0].get_total_support()?
                < secondary_terror_targets[0].get_total_support()?
            {
                let terror_target = secondary_terror_targets.pop_front().unwrap();

                produce_mutations_for_terror_at_space(
                    terror_target,
                    Factions::NVA,
                    forces_mutations,
                    space_mutations,
                )?;
            }
        }

        *nva_resources -= 1;
    }

    Ok(())
}
