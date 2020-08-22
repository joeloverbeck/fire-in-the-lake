use board::domain::board::Board;
use game_definitions::faction_stats::FactionStats;
use game_definitions::factions::Factions;
use players::domain::decision::Decision;
use players::domain::decision_production::produce_decision_to_pass::produce_decision_to_pass;

pub fn whether_to_pass(faction: Factions, board: &Board) -> Result<Option<Decision>, String> {
    // NVA passes if NVA Resources == 0

    match faction {
        Factions::NVA => {
            if board.get_faction_stat(FactionStats::NvaResources)? == 0 {
                return Ok(Some(produce_decision_to_pass(faction, board)?));
            }
        }
        _ => panic!("Not implemented for faction {:?}", faction),
    }

    Ok(None)
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    use sequence_of_play::domain::sequence_of_play_slots::SequenceOfPlaySlots;

    #[test]
    fn test_when_nva_has_no_resources_it_will_produce_a_pass_decision() -> Result<(), String> {
        let possible_decision = whether_to_pass(Factions::NVA, &Board::new())?;

        assert!(possible_decision.is_some());

        if let Some(decision) = possible_decision {
            assert_eq!(
                decision.get_sequence_of_play_mutations()[0].get_sequence_of_play_slot(),
                &SequenceOfPlaySlots::Pass
            );
        }

        Ok(())
    }
}
