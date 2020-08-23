use players::domain::decision::Decision;
use players::domain::faction_stats_mutation::FactionStatsMutation;
use players::domain::flags_mutation::FlagsMutation;
use players::domain::forces_mutation::ForcesMutation;
use players::domain::mutations::Mutations;
use players::domain::sequence_of_play_mutation::SequenceOfPlayMutation;
use players::domain::space_mutation::SpaceMutation;

pub fn produce_decision_for_special_activity(
    possible_operation_decision: Option<&Decision>,
    special_activity_decision: Decision,
) -> Result<Decision, String> {
    // If there isn't a decision for the main operation, just return the special activity's decision:

    if possible_operation_decision.is_none() {
        return Ok(special_activity_decision);
    }

    // Otherwise we need to create a new decision taking all the stuff from the first,
    // then adding the stuff from the decision of the special activity.

    let operation_decision = possible_operation_decision.as_ref().unwrap();

    let mutations = Mutations::new();

    let mut sequence_of_play_mutations: Vec<SequenceOfPlayMutation> = Vec::new();
    let mut space_mutations: Vec<SpaceMutation> = Vec::new();
    let mut faction_stats_mutations: Vec<FactionStatsMutation> = Vec::new();
    let mut forces_mutations: Vec<ForcesMutation> = Vec::new();
    let mut flags_mutations: Vec<FlagsMutation> = Vec::new();

    if operation_decision
        .get_mutations()?
        .has_sequence_of_play_mutations()?
    {
        for entry in operation_decision
            .get_mutations()?
            .get_sequence_of_play_mutations()?
            .iter()
        {
            sequence_of_play_mutations.push(*entry);
        }
    }
    if special_activity_decision
        .get_mutations()?
        .has_sequence_of_play_mutations()?
    {
        let mut from_special_activity = special_activity_decision
            .get_mutations()?
            .get_sequence_of_play_mutations()?
            .clone();
        sequence_of_play_mutations.append(&mut from_special_activity);
    }

    if operation_decision.get_mutations()?.has_space_mutations()? {
        let mut from_operation = operation_decision
            .get_mutations()?
            .get_space_mutations()?
            .clone();
        space_mutations.append(&mut from_operation);
    }
    if special_activity_decision
        .get_mutations()?
        .has_space_mutations()?
    {
        let mut from_special_activity = special_activity_decision
            .get_mutations()?
            .get_space_mutations()?
            .clone();
        space_mutations.append(&mut from_special_activity);
    }

    if operation_decision
        .get_mutations()?
        .has_faction_stats_mutations()?
    {
        let mut from_operation = operation_decision
            .get_mutations()?
            .get_faction_stats_mutations()?
            .clone();
        faction_stats_mutations.append(&mut from_operation);
    }
    if special_activity_decision
        .get_mutations()?
        .has_faction_stats_mutations()?
    {
        let mut from_special_activity = special_activity_decision
            .get_mutations()?
            .get_faction_stats_mutations()?
            .clone();
        faction_stats_mutations.append(&mut from_special_activity);
    }

    if operation_decision.get_mutations()?.has_forces_mutations()? {
        let mut from_operation = operation_decision
            .get_mutations()?
            .get_forces_mutations()?
            .clone();
        forces_mutations.append(&mut from_operation);
    }
    if special_activity_decision
        .get_mutations()?
        .has_forces_mutations()?
    {
        let mut from_special_activity = special_activity_decision
            .get_mutations()?
            .get_forces_mutations()?
            .clone();
        forces_mutations.append(&mut from_special_activity);
    }

    if operation_decision.get_mutations()?.has_flags_mutations()? {
        let mut from_operation = operation_decision
            .get_mutations()?
            .get_flags_mutations()?
            .clone();
        flags_mutations.append(&mut from_operation);
    }
    if special_activity_decision
        .get_mutations()?
        .has_flags_mutations()?
    {
        let mut from_special_activity = special_activity_decision
            .get_mutations()?
            .get_flags_mutations()?
            .clone();
        flags_mutations.append(&mut from_special_activity);
    }

    Ok(Decision::new(
        mutations,
        Some(*operation_decision.get_main_action()?),
        Some(*special_activity_decision.get_secondary_action()?),
    ))
}
