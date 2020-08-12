extern crate fire_in_the_lake;

use fire_in_the_lake::board::available_forces::AvailableForces;
use fire_in_the_lake::board::current_rvn_leader::CurrentRvnLeader;
use fire_in_the_lake::board::map::Map;
use fire_in_the_lake::board::map_builder::MapBuilder;
use fire_in_the_lake::board::space::Space;
use fire_in_the_lake::board::space_identifiers::SpaceIdentifiers;
use fire_in_the_lake::board::support::SupportLevels;
use fire_in_the_lake::board::track::Track;
use fire_in_the_lake::cards::card_registry::CardRegistry;
use fire_in_the_lake::commands::execute_commands::execute_commands;
use fire_in_the_lake::decision_making::choices::Choices;
use fire_in_the_lake::decision_making::commands_producer::CommandsProducer;
use fire_in_the_lake::decision_making::decision::Decision;
use fire_in_the_lake::decision_making::decision_making_center::DecisionMakingCenter;
use fire_in_the_lake::decision_making::testing::playbook_first_turn_arvn::PlaybookFirstTurnArvn;
use fire_in_the_lake::decision_making::testing::playbook_first_turn_nva::PlaybookFirstTurnNva;
use fire_in_the_lake::decision_making::testing::playbook_first_turn_vc::PlaybookFirstTurnVc;
use fire_in_the_lake::display::announcer::Announcer;
use fire_in_the_lake::factions::Factions;
use fire_in_the_lake::game_flow::game_flow_handler::GameFlowHandler;

#[test]
fn test_first_game_turn_playbook() -> Result<(), String> {
    let card_registry = CardRegistry::new();

    let mut game_flow_handler = GameFlowHandler::new(&card_registry);

    // Start. Game turn 1 (1/4)

    // Draw Burning Bonze
    game_flow_handler.set_active_card(107);
    game_flow_handler.set_preview_card(55);

    assert_eq!(
        game_flow_handler.get_active_card(),
        107,
        "After setting the active card, it should be the one expected."
    );

    // The faction order implicit to the card should allows us to get immediately that VC is the current eligible.
    assert_eq!(
        game_flow_handler.get_current_eligible(),
        Factions::VC,
        "The current eligible after first drawing Burning Bonze should have been VC."
    );

    // Now for the hard part. There should be a construct that handles decision making, and it should be able to be "mocked" as in we just return
    // exactly what is decided but programmatically for tests.
    let decision_making_center = DecisionMakingCenter::new(
        PlaybookFirstTurnVc::new().into(),
        PlaybookFirstTurnNva::new().into(),
        PlaybookFirstTurnArvn::new().into(),
        PlaybookFirstTurnArvn::new().into(),
    );

    let map_builder = MapBuilder::new();

    let mut built_map = map_builder.build_initial_map().unwrap();

    let mut track = Track::new();

    let mut available_forces = AvailableForces::new();

    // Set up map to initial state.
    built_map
        .get_space_mut(SpaceIdentifiers::Saigon)
        .unwrap()
        .set_support_level(SupportLevels::PassiveSupport);
    track.set_aid(15);

    let mut possible_vc_decision = decision_making_center.decide(
        game_flow_handler.get_active_card(),
        game_flow_handler.get_current_eligible(),
        &built_map,
        &track,
        &available_forces,
    );

    let mut vc_decision;

    match possible_vc_decision {
        Ok(decision) => vc_decision = decision,
        Err(error) => {
            panic!("Something went wrong when producing the decision for VC during the first turn.")
        }
    }

    assert_eq!(
        vc_decision.get_choice(),
        Choices::ShadedEvent,
        "VC's choice should have been the shaded event."
    );

    // Should tell the game_flow_handler that VC decided for the ShadedEvent, so it can change whatever it needs to change.
    game_flow_handler.report_choice(
        game_flow_handler.get_current_eligible(),
        vc_decision.get_choice(),
    )?;

    // After learning that choice, it should have made internal changes.
    assert_eq!(
        game_flow_handler.get_current_eligible(),
        Factions::NVA,
        "After VC makes its choice, the next eligible should be the current eligible."
    );
    assert_eq!(
        game_flow_handler.faction_present_in_first_eligible_event(),
        Factions::VC,
        "The faction locked into the '1st Eligible event' should have been VC, but was {:?} ",
        game_flow_handler.faction_present_in_first_eligible_event()
    );
    assert_eq!(
        game_flow_handler.is_faction_eligible(Factions::VC),
        false,
        "After making a choice, VC should no longer be considered eligible."
    );

    // Should give the player an announcement: VC cylinder should move from eligible to 1st Eligible Event Box.
    let announcer = Announcer::new();

    announcer
        .instruct_to_move_faction_cylinder_from_eligible_to_first_eligible_event_box(Factions::VC);

    // The received decision just has the Choice that the sequence of play knows how to deal with,
    // and for the remainder we "just" have the equivalent to, or maybe literal, typed commands on the console.
    // We need some intermediary to interpret them.
    execute_commands(
        game_flow_handler.get_active_card(),
        vc_decision.get_faction(),
        vc_decision.get_interpreted_intentions(),
        &mut built_map,
        &mut track,
        &mut available_forces,
    );

    // The commands should have shifted Saigon's Passive Support to Neutral.
    assert_eq!(
        built_map
            .get_space(SpaceIdentifiers::Saigon)
            .unwrap()
            .get_support_level(),
        SupportLevels::Neutral
    );
    assert_eq!(
        track.get_aid(),
        3,
        "After executing VC's commands, Aid should have been 3, but was {:?}",
        track.get_aid()
    );

    // Start. Game turn 1 (2/4)
    track.set_nva_resources(10);

    // NVA should be next in consideration
    assert_eq!(game_flow_handler.get_current_eligible(), Factions::NVA);

    // In this occasion, it chooses to pass. We gotta send the decision making center
    // the needed information, and we should get back "pass".
    let mut possible_nva_decision = decision_making_center.decide(
        game_flow_handler.get_active_card(),
        game_flow_handler.get_current_eligible(),
        &built_map,
        &track,
        &available_forces,
    );

    let mut nva_decision;

    match possible_nva_decision {
        Ok(decision) => nva_decision = decision,
        Err(error) => panic!(
            "Something went wrong when producing the decision for NVA during the first turn."
        ),
    }

    assert_eq!(
        nva_decision.get_choice(),
        Choices::Pass,
        "NVA's choice should have been to pass."
    );

    // Should inform the game_flow_handler
    game_flow_handler.report_choice(
        game_flow_handler.get_current_eligible(),
        nva_decision.get_choice(),
    )?;

    // Given that NVA has passed, the next eligible should be the third in the card order.
    assert_eq!(
        game_flow_handler.get_current_eligible(),
        Factions::ARVN,
        "After NVA makes its choice, the next eligible should be the current eligible."
    );
    assert_eq!(
        game_flow_handler.has_faction_passed(Factions::NVA),
        true,
        "NVA should have been registered as having passed.",
    );
    assert_eq!(
        game_flow_handler.is_faction_eligible(Factions::NVA),
        false,
        "After making a choice, NVA should no longer be considered eligible."
    );

    announcer.instruct_to_move_faction_cylinder_from_eligible_to_passed_box(Factions::NVA);

    // Must execute the pass command, because it gives resources.
    execute_commands(
        game_flow_handler.get_active_card(),
        nva_decision.get_faction(),
        nva_decision.get_interpreted_intentions(),
        &mut built_map,
        &mut track,
        &mut available_forces,
    )?;

    assert_eq!(track.get_nva_resources(), 11);

    // Start. Game turn 1 (3/4)

    // Now ARVN should be eligible.
    assert_eq!(game_flow_handler.get_current_eligible(), Factions::ARVN);

    assert_eq!(game_flow_handler.is_execute_op_and_special_activity_available(), true, "Should be able to execute op and special activity, because the first eligible chose event.");

    // This player interaction is much more complicated than the previous one. The player in the playbook chooses Train in Saigon,
    // and then Pacify in Saigon (which is specially available for the op)
    // Then adjust US victory marker
    // Not content with that, it also chooses Special Activity, which is Govern, and in two locations, An Loc and Can Tho
    // That increases Aid +6 (+3 per each city, 3x1 Pop)
    // Also, for Aid it also matters the CURRENT RNV LEADER, who is Minh. If ARVN Trains, Aid receives +5 bonus.
    // After implementing all these systems, we are a long way there.
    let mut possible_arvn_decision = decision_making_center.decide(
        game_flow_handler.get_active_card(),
        game_flow_handler.get_current_eligible(),
        &built_map,
        &track,
        &available_forces,
    );

    let mut arvn_decision;

    match possible_arvn_decision {
        Ok(decision) => arvn_decision = decision,
        Err(error) => panic!("Something went wrong when producing the decision for ARVN during the first turn. Error: {:?}", error)
    }

    assert_eq!(
        arvn_decision.get_choice(),
        Choices::SecondOperationAndSpecialActivity,
        "ARVN's choice should have been to perform an operation."
    );

    // Should inform the game_flow_handler
    game_flow_handler.report_choice(
        game_flow_handler.get_current_eligible(),
        arvn_decision.get_choice(),
    )?;

    // Given that ARVN has chosen an operation, there should be no next eligible (two have already chosen)
    assert_eq!(
        game_flow_handler.get_current_eligible(),
        Factions::None,
        "After ARVN makes its choice, there should be no next eligible."
    );
    assert_eq!(
        game_flow_handler.is_execute_op_and_special_activity_available(),
        false,
        "The execute op and special activity should no longer be available.",
    );
    assert_eq!(
        game_flow_handler.is_faction_eligible(Factions::ARVN),
        false,
        "After making a choice, NVA should no longer be considered eligible."
    );

    announcer
        .instruct_to_move_faction_cylinder_from_eligible_to_second_operation_and_special_activity_box(
            Factions::ARVN,
        );

    // Last chance to add some setup to arvn's turn
    track.set_arvn_resources(30);
    available_forces.set_amount_of_arvn_troops(6);

    assert_eq!(
        built_map.get_current_rvn_leader(),
        CurrentRvnLeader::Minh,
        "The current ARVN leader should have been Minh"
    );

    // Must execute the operation command. That delegate function is going to do so much work.
    execute_commands(
        game_flow_handler.get_active_card(),
        arvn_decision.get_faction(),
        arvn_decision.get_interpreted_intentions(),
        &mut built_map,
        &mut track,
        &mut available_forces,
    )?;

    assert_eq!(
        track.get_arvn_resources(),
        24,
        "The amount of arvn resources should be 24"
    );

    let retrieved_saigon_result = built_map.get_space(SpaceIdentifiers::Saigon);

    if let Ok(retrieved_saigon) = retrieved_saigon_result {
        assert_eq!(
            retrieved_saigon.get_number_of_arvn_troops(),
            6,
            "There should be 6 arvn troops in Saigon"
        );
        assert_eq!(
            retrieved_saigon.get_support_level(),
            SupportLevels::PassiveSupport,
            "The support of Saigon should have been PassiveSupport, but was {:?}",
            retrieved_saigon.get_support_level()
        );
    } else {
        panic!("Should have been able to retrieve Saigon.");
    }

    assert_eq!(
        track.get_aid(),
        14,
        "Aid should have been 14, but was {:?}",
        track.get_aid()
    );

    // Start. Game turn 1 (4/4)

    // Real close to the end now.
    // Two eligible factions have acted, so the turn is over.
    assert_eq!(game_flow_handler.has_turn_ended(), true);

    game_flow_handler.perform_end_of_turn();

    assert_eq!(game_flow_handler.is_faction_eligible(Factions::NVA), true);
    assert_eq!(game_flow_handler.is_faction_eligible(Factions::US), true);
    assert_eq!(game_flow_handler.is_faction_eligible(Factions::VC), false);
    assert_eq!(game_flow_handler.is_faction_eligible(Factions::ARVN), false);

    assert_eq!(
        game_flow_handler.get_active_card(),
        55,
        "The current card should be 'Trucks'."
    );

    Ok(())
}
