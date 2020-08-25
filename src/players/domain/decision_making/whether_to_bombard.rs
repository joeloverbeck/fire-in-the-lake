use board::domain::queries::board_level_queries::get_spaces_with_enough_concentration_of_coin_troops_to_bombard_but_nva_troops_only_adjacent::get_spaces_with_enough_concentration_of_coin_troops_to_bombard_but_nva_troops_only_adjacent;
use board::domain::queries::board_level_queries::get_spaces_with_enough_concentration_of_coin_troops_to_bombard_and_nva_troops_present::get_spaces_with_enough_concentration_of_coin_troops_to_bombard_and_nva_troops_present;
use board::domain::board::Board;
use board::domain::queries::space_level_queries::calculate_number_of_faction_group_bases_in_space::calculate_number_of_faction_group_bases_in_space;
use board::domain::space::Space;
use board::domain::space::Spaces;
use flags::controllers::flags_controller::FlagsController;
use game_definitions::event_types::EventTypes;
use game_definitions::faction_groups::FactionGroups;
use game_definitions::forces::Forces;
use game_definitions::nva_capabilities::NvaCapabilities;
use players::domain::decision::Decision;
use players::domain::decision_information::DecisionInformation;
use players::domain::decision_production::produce_decision_for_special_activity::produce_decision_for_special_activity;
use players::domain::forces_mutation::ForcesMutation;
use players::domain::mutations::Mutations;
use players::domain::mutations_production::produce_mutations_for_bombardment::produce_mutations_for_bombardment;
use std::collections::VecDeque;

pub fn whether_to_bombard(
    possible_previous_decision: Option<&Decision>,
    board: &Board,
    flags_controller: &FlagsController,
) -> Result<Option<Decision>, String> {
    // Rules: Bombard 2 spaces (in or adjacent to 1 or more spaces with at least 3 NVA Troops, and with either at least 3 COIN
    // troops or a COIN Base and at least 1 COIN Troop). If the "Long Range Guns" Capability is in effect, Bombard 1 or 3
    // spaces as allowed:

    // -First in spaces with NVA pieces
    // -Then in spaces adjacent to NVA Troops.
    // -Within both of the above priorities, first select spaces with a COIN Base; within that priority, select spaces
    // with and Bombard to remove US Troops before ARVN ones.

    // First need to find if there's any applicable space.
    let primary_bombard_targets: Vec<&Spaces> =
        get_spaces_with_enough_concentration_of_coin_troops_to_bombard_and_nva_troops_present(
            board,
        )?;

    let secondary_bombard_targets: Vec<&Spaces> = get_spaces_with_enough_concentration_of_coin_troops_to_bombard_but_nva_troops_only_adjacent(board)?;

    // Obviously if there are no targets, no decision is to be made.
    if primary_bombard_targets.is_empty() && secondary_bombard_targets.is_empty() {
        return Ok(None);
    }

    // We are going to bombard. We need to know how many spaces we can hit.
    // If the "Long Range Guns" capability is in effect, you either Bombard 1 or 3 spaces depending on what
    // side we are on that capability.
    let mut max_bombardments = 2;

    if flags_controller.has_nva_capability(NvaCapabilities::LongRangeGuns)? {
        if flags_controller.get_event_type_of_nva_capability(NvaCapabilities::LongRangeGuns)?
            == &EventTypes::Unshaded
        {
            max_bombardments = 1;
        } else {
            max_bombardments = 3;
        }
    }

    // We need to further separate the targets.
    let mut ordered_targets: VecDeque<&Spaces> = VecDeque::new();

    // Main targets: have coin bases and Us Troops
    for target in primary_bombard_targets.iter() {
        if calculate_number_of_faction_group_bases_in_space(FactionGroups::Coin, target)? > 0
            && target.get_forces(Forces::UsTroop)? > 0
        {
            ordered_targets.push_back(target);
        }
    }

    // Main targets: have coin bases but no US Troops
    for target in primary_bombard_targets.iter() {
        if calculate_number_of_faction_group_bases_in_space(FactionGroups::Coin, target)? > 0
            && target.get_forces(Forces::UsTroop)? == 0
        {
            ordered_targets.push_back(target);
        }
    }

    // Main targets: don't have coin bases.
    for target in primary_bombard_targets.iter() {
        if calculate_number_of_faction_group_bases_in_space(FactionGroups::Coin, target)? == 0 {
            ordered_targets.push_back(target);
        }
    }

    // Secondary targets: have coin bases and Us Troops
    for target in secondary_bombard_targets.iter() {
        if calculate_number_of_faction_group_bases_in_space(FactionGroups::Coin, target)? > 0
            && target.get_forces(Forces::UsTroop)? > 0
        {
            ordered_targets.push_back(target);
        }
    }

    // Secondary targets: have coin bases but no Us Troops
    for target in secondary_bombard_targets.iter() {
        if calculate_number_of_faction_group_bases_in_space(FactionGroups::Coin, target)? > 0
            && target.get_forces(Forces::UsTroop)? == 0
        {
            ordered_targets.push_back(target);
        }
    }

    // Secondary targets: don't have coin bases
    for target in secondary_bombard_targets.iter() {
        if calculate_number_of_faction_group_bases_in_space(FactionGroups::Coin, target)? == 0 {
            ordered_targets.push_back(target);
        }
    }

    // Sanity check:
    if ordered_targets.len() != primary_bombard_targets.len() + secondary_bombard_targets.len() {
        panic!("Ordering the primary and secondary targets should have created a VecDeque of equal length. Ordered targets: {:?}. Primary bombard targets: {:?}. Secondary bombard targets: {:?}", ordered_targets, primary_bombard_targets, secondary_bombard_targets)
    }

    if ordered_targets.is_empty() {
        panic!("After finally pushing all the ordered targets in a VecDeque, the deque was empty!");
    }

    let mut forces_mutations: Vec<ForcesMutation> = Vec::new();

    while max_bombardments > 0 {
        if !ordered_targets.is_empty() {
            produce_mutations_for_bombardment(
                ordered_targets.pop_front().unwrap(),
                &mut forces_mutations,
            )?;
        }

        max_bombardments -= 1;
    }

    let mut mutations = Mutations::new();

    mutations.set_forces_mutations(forces_mutations)?;

    let decision = Decision::new(mutations, None, Some(DecisionInformation::Bombard));

    Ok(Some(produce_decision_for_special_activity(
        possible_previous_decision,
        decision,
    )?))
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    use flags::controllers::flags_controller::FlagsController;
    use game_definitions::space_identifiers::SpaceIdentifiers;

    #[test]
    fn test_will_not_decide_to_bombard_if_there_are_no_appropriate_targets() -> Result<(), String> {
        let board = Board::new();

        let flags_controller = FlagsController::new();

        let possible_decision = whether_to_bombard(None, &board, &flags_controller)?;

        assert!(possible_decision.is_none());

        Ok(())
    }

    #[test]
    fn test_will_not_decide_to_bombard_if_theres_concentration_of_coin_forces_but_not_enough_nva_troops(
    ) -> Result<(), String> {
        let mut board = Board::new();

        board.set_forces_in_space(Forces::ArvnTroop, 1, SpaceIdentifiers::KienHoaVinhBinh)?;
        board.set_forces_in_space(Forces::UsTroop, 2, SpaceIdentifiers::KienHoaVinhBinh)?;

        let flags_controller = FlagsController::new();

        let possible_decision = whether_to_bombard(None, &board, &flags_controller)?;

        assert!(possible_decision.is_none());

        Ok(())
    }

    #[test]
    fn test_will_decide_to_bombard_if_theres_concentration_of_coin_forces_and_enough_adjacent_nva_troops(
    ) -> Result<(), String> {
        let mut board = Board::new();

        board.set_forces_in_space(Forces::ArvnTroop, 1, SpaceIdentifiers::KienHoaVinhBinh)?;
        board.set_forces_in_space(Forces::UsTroop, 2, SpaceIdentifiers::KienHoaVinhBinh)?;

        board.set_forces_in_space(Forces::NvaTroop, 3, SpaceIdentifiers::Route4MekongEast)?;

        let flags_controller = FlagsController::new();

        let possible_decision = whether_to_bombard(None, &board, &flags_controller)?;

        assert!(possible_decision.is_some());

        Ok(())
    }

    #[test]
    fn test_will_decide_to_bombard_if_theres_concentration_of_coin_forces_and_enough_nva_troops_in_same_location(
    ) -> Result<(), String> {
        let mut board = Board::new();

        board.set_forces_in_space(Forces::ArvnTroop, 1, SpaceIdentifiers::KienHoaVinhBinh)?;
        board.set_forces_in_space(Forces::UsTroop, 2, SpaceIdentifiers::KienHoaVinhBinh)?;

        board.set_forces_in_space(Forces::NvaTroop, 3, SpaceIdentifiers::KienHoaVinhBinh)?;

        let flags_controller = FlagsController::new();

        let possible_decision = whether_to_bombard(None, &board, &flags_controller)?;

        assert!(possible_decision.is_some());

        let decision = possible_decision.unwrap();

        assert_eq!(
            decision.get_secondary_action()?,
            &DecisionInformation::Bombard
        );
        assert!(decision.get_mutations()?.has_forces_mutations()?);

        Ok(())
    }
}
