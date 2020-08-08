extern crate fire_in_the_lake;

use fire_in_the_lake::board::map::Map;
use fire_in_the_lake::board::map_builder::MapBuilder;
use fire_in_the_lake::board::space::Space;
use fire_in_the_lake::board::space_identifiers::SpaceIdentifiers;
use fire_in_the_lake::board::support::SupportLevels;
use fire_in_the_lake::board::track::Track;
use fire_in_the_lake::cards::card_registry::CardRegistry;
use fire_in_the_lake::commands::execute_commands::execute_commands;
use fire_in_the_lake::commands::manipulate_aid::ManipulateAid;
use fire_in_the_lake::commands::shift_support_of_space::ShiftSupportOfSpace;
use fire_in_the_lake::decision_making::choices::Choices;
use fire_in_the_lake::decision_making::commands_producer::CommandsProducer;
use fire_in_the_lake::decision_making::decision::Decision;
use fire_in_the_lake::decision_making::decision_making_center::DecisionMakingCenter;
use fire_in_the_lake::decision_making::player::PlaybookFirstTurnNva;
use fire_in_the_lake::decision_making::player::PlaybookFirstTurnVc;
use fire_in_the_lake::display::announcer::Announcer;
use fire_in_the_lake::factions::Factions;
use fire_in_the_lake::game_flow::game_flow_handler::GameFlowHandler;
use fire_in_the_lake::game_flow::sequence_of_play::SequenceOfPlay;

#[test]
fn test_first_game_turn_vc() -> Result<(), String> {
    let card_registry = CardRegistry::new();
    let mut sequence_of_play = SequenceOfPlay::new();

    let mut game_flow_handler = GameFlowHandler::new(&card_registry, &mut sequence_of_play);

    // Start. Game turn 1 (1/4)

    // Draw Burning Bonze
    game_flow_handler.set_active_card(107);

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
        PlaybookFirstTurnVc::new().into(),
        PlaybookFirstTurnVc::new().into(),
    );

    let map_builder = MapBuilder::new();

    let mut built_map = map_builder.build_initial_map().unwrap();

    let mut track = Track::new();

    // Set up map to initial state.
    built_map
        .get_space_mut(SpaceIdentifiers::Saigon)
        .unwrap()
        .set_support_level(SupportLevels::PassiveSupport);
    track.set_aid(15);

    let mut vc_decision = decision_making_center.decide(
        game_flow_handler.get_active_card(),
        game_flow_handler.get_current_eligible(),
        &built_map,
        &track,
    );

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
        vc_decision.get_commands(),
        &mut built_map,
        &mut track,
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
    let mut nva_decision = decision_making_center.decide(
        game_flow_handler.get_active_card(),
        game_flow_handler.get_current_eligible(),
        &built_map,
        &track,
    );

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
        nva_decision.get_commands(),
        &mut built_map,
        &mut track,
    );

    assert_eq!(track.get_nva_resources(), 11);

    Ok(())
}
