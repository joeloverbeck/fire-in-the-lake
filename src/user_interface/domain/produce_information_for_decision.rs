use game_definitions::factions::Factions;
use players::domain::decision::Decision;
use players::domain::decision_information::DecisionInformation;

pub fn produce_information_for_decision(
    decision: &Decision,
    faction: &Factions,
) -> Result<Vec<String>, String> {
    // Depending on the slot it has chosen to occupy, we can determine what it
    // Intended to do.
    if decision
        .get_mutations()?
        .get_sequence_of_play_mutations()?
        .len()
        > 1
    {
        panic!("Was going to produce information for a decision, but couldn't handle the case that the sequence of play mutations would be more than one.");
    }

    let mut information: Vec<String> = Vec::new();

    match decision.get_main_action()? {
        DecisionInformation::Event => {
            information.push(format!("{} chose to play the card for the event", faction));
        }
        DecisionInformation::Pass => {
            information.push(format!("{} chose to pass", faction));
        }
        DecisionInformation::Terror => {
            information.push(format!("{} commits terror attacks!", faction));
        }
        _ => panic!("Not implemented for {:?}", decision.get_main_action()?),
    }

    Ok(information)
}
