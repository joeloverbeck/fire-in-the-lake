use board::domain::queries::space_level_queries::calculate_number_of_a_factions_bases_in_space::calculate_number_of_a_factions_bases_in_space;
use board::domain::space::Spaces;
use game_definitions::factions::Factions;

pub fn are_there_faction_bases_in_space(faction: Factions, space: &Spaces) -> Result<bool, String> {
    match faction {
        Factions::NVA => {
            Ok(calculate_number_of_a_factions_bases_in_space(faction, space).unwrap() > 0)
        }
        Factions::VC => {
            Ok(calculate_number_of_a_factions_bases_in_space(faction, space).unwrap() > 0)
        }
        _ => panic!(
            "Are there faction bases in space not implemented for faction {:?}",
            faction
        ),
    }
}
