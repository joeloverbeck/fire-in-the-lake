use board::domain::queries::calculate_number_of_a_particular_force_in_space::calculate_number_of_a_particular_force_in_space;
use board::domain::queries::calculate_number_of_coin_pieces_minus_bases_in_space::calculate_number_of_coin_pieces_minus_bases_in_space;
use board::domain::space::Spaces;
use game_definitions::factions::Factions;
use game_definitions::forces::Forces;

pub fn can_attack_remove_a_number_of_enemies_in_space(
    faction: &Factions,
    space: &Spaces,
    minimum_number_of_enemies_to_remove: u8,
) -> Result<bool, String> {
    if faction == &Factions::NVA
        && (calculate_number_of_a_particular_force_in_space(Forces::ActiveNvaGuerrilla, &space)?
            + calculate_number_of_a_particular_force_in_space(
                Forces::UndergroundNvaGuerrilla,
                &space,
            )?)
            == 0
    {
        // Can't kill anybody here with any dice throw.
        return Ok(false);
    }

    if faction == &Factions::VC
        && (calculate_number_of_a_particular_force_in_space(Forces::ActiveVcGuerrilla, &space)?
            + calculate_number_of_a_particular_force_in_space(
                Forces::UndergroundVcGuerrilla,
                &space,
            )?)
            == 0
    {
        // Can't kill anybody here with any dice throw.
        return Ok(false);
    }

    // We know that it could kill up to 2 in a successful throw, but there must be enemy pieces here.
    if calculate_number_of_coin_pieces_minus_bases_in_space(&space)?
        >= minimum_number_of_enemies_to_remove
    {
        return Ok(true);
    }

    Ok(false)
}
