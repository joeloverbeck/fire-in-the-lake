use board::domain::queries::are_there_any_nva_guerrillas_in_space::are_there_any_nva_guerrillas_in_space;
use board::domain::queries::are_there_any_vc_guerrillas_in_space::are_there_any_vc_guerrillas_in_space;
use board::domain::queries::are_there_coin_bases_in_space::are_there_coin_bases_in_space;
use board::domain::queries::calculate_number_of_coin_pieces_minus_bases_in_space::calculate_number_of_coin_pieces_minus_bases_in_space;
use board::domain::queries::can_nva_troops_obliterate_present_coin_forces_to_attack_bases::can_nva_troops_obliterate_present_coin_forces_to_attack_bases;
use board::domain::space::Spaces;
use game_definitions::factions::Factions;

pub fn can_attack_remove_base_in_space(
    occupable_space: &Spaces,
    faction: &Factions,
) -> Result<bool, String> {
    let mut can_attack_remove_base = false;

    if !are_there_coin_bases_in_space(&occupable_space)? {
        return Ok(can_attack_remove_base);
    }

    if faction == &Factions::NVA
        && can_nva_troops_obliterate_present_coin_forces_to_attack_bases(&occupable_space)?
    {
        // Any of the bases can indeed be removed.
        can_attack_remove_base = true;

        return Ok(can_attack_remove_base);
    }

    // Even NVA can do the following check:
    // Roll a die: if the roll is less than or equal to the number of the executing's
    // faction Guerrillas there (active or not), remove UP TO 2 enemy pieces of the executing faction's choice.

    // There must be at least a guerrilla present, and they must be of the executing faction.
    if faction == &Factions::NVA && are_there_any_nva_guerrillas_in_space(&occupable_space)? {
        // Could only remove up to 2 enemy pieces in a successful roll, so the number of enemy pieces that aren't bases shouldn't be superior to 1.
        if calculate_number_of_coin_pieces_minus_bases_in_space(&occupable_space)? < 2 {
            can_attack_remove_base = true;
        }
    } else if faction == &Factions::VC && are_there_any_vc_guerrillas_in_space(&occupable_space)? {
        panic!("Not handled for VC!");
    }

    Ok(can_attack_remove_base)
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use board::domain::board::Board;
    use game_definitions::factions::Factions;

    use game_definitions::forces::Forces;
    use game_definitions::space_identifiers::SpaceIdentifiers;

    #[test]
    fn test_an_attack_can_remove_a_base_where_there_are_no_other_coin_pieces_and_there_are_nva_guerrillas(
    ) -> Result<(), String> {
        let mut board = Board::new();

        board.set_forces_in_space(Forces::ArvnBase, 1, SpaceIdentifiers::Saigon)?;
        board.set_forces_in_space(Forces::UndergroundNvaGuerrilla, 1, SpaceIdentifiers::Saigon)?;

        assert_eq!(
            can_attack_remove_base_in_space(
                board.get_occupable_space(&SpaceIdentifiers::Saigon)?,
                &Factions::NVA
            )?,
            true
        );

        Ok(())
    }

    #[test]
    fn test_an_attack_cannot_remove_a_base_where_there_are_no_other_coin_pieces_but_no_nva_guerrillas(
    ) -> Result<(), String> {
        let mut board = Board::new();

        board.set_forces_in_space(Forces::ArvnBase, 1, SpaceIdentifiers::Saigon)?;

        assert_eq!(
            can_attack_remove_base_in_space(
                board.get_occupable_space(&SpaceIdentifiers::Saigon)?,
                &Factions::NVA
            )?,
            false
        );

        Ok(())
    }

    #[test]
    fn test_an_attack_cannot_remove_a_base_where_there_are_other_coin_pieces_despite_nva_guerrillas(
    ) -> Result<(), String> {
        let mut board = Board::new();

        board.set_forces_in_space(Forces::ArvnBase, 1, SpaceIdentifiers::Saigon)?;
        board.set_forces_in_space(Forces::ArvnPolice, 2, SpaceIdentifiers::Saigon)?;
        board.set_forces_in_space(Forces::UndergroundNvaGuerrilla, 2, SpaceIdentifiers::Saigon)?;

        assert_eq!(
            can_attack_remove_base_in_space(
                board.get_occupable_space(&SpaceIdentifiers::Saigon)?,
                &Factions::NVA
            )?,
            false
        );

        Ok(())
    }

    #[test]
    fn test_an_attack_cannot_remove_a_base_where_there_are_other_coin_pieces_if_there_are_many_nva_guerrillas_but_no_troops(
    ) -> Result<(), String> {
        let mut board = Board::new();

        board.set_forces_in_space(Forces::ArvnBase, 1, SpaceIdentifiers::Saigon)?;
        board.set_forces_in_space(Forces::ArvnPolice, 2, SpaceIdentifiers::Saigon)?;
        board.set_forces_in_space(Forces::UndergroundNvaGuerrilla, 6, SpaceIdentifiers::Saigon)?;

        assert_eq!(
            can_attack_remove_base_in_space(
                board.get_occupable_space(&SpaceIdentifiers::Saigon)?,
                &Factions::NVA
            )?,
            false
        );

        Ok(())
    }

    #[test]
    fn test_an_attack_can_remove_a_base_where_there_are_other_coin_pieces_if_there_are_enough_nva_troops(
    ) -> Result<(), String> {
        let mut board = Board::new();

        board.set_forces_in_space(Forces::ArvnBase, 1, SpaceIdentifiers::Saigon)?;
        board.set_forces_in_space(Forces::ArvnPolice, 2, SpaceIdentifiers::Saigon)?;
        board.set_forces_in_space(Forces::NvaTroop, 6, SpaceIdentifiers::Saigon)?;

        assert_eq!(
            can_attack_remove_base_in_space(
                board.get_occupable_space(&SpaceIdentifiers::Saigon)?,
                &Factions::NVA
            )?,
            true
        );

        Ok(())
    }
}
