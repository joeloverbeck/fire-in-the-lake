extern crate fire_in_the_lake;

use fire_in_the_lake::board::available_forces::AvailableForces;
use fire_in_the_lake::board::controls::Controls;
use fire_in_the_lake::board::map_builder::MapBuilder;
use fire_in_the_lake::board::space::Space;
use fire_in_the_lake::board::space_identifiers::SpaceIdentifiers;
use fire_in_the_lake::board::support::SupportLevels;
use fire_in_the_lake::board::track::Track;
use fire_in_the_lake::cards::card_registry::CardRegistry;
use fire_in_the_lake::commands::execute_commands::execute_commands;
use fire_in_the_lake::decision_making::choices::Choices;
use fire_in_the_lake::decision_making::commands_producer::CommandsProducer;
use fire_in_the_lake::decision_making::decision_making_center::DecisionMakingCenter;
use fire_in_the_lake::decision_making::testing::playbook_fourth_turn_us::PlaybookFourthTurnUs;
use fire_in_the_lake::decision_making::testing::playbook_second_turn_nva::PlaybookSecondTurnNva;
use fire_in_the_lake::decision_making::testing::playbook_second_turn_us::PlaybookSecondTurnUs;
use fire_in_the_lake::decision_making::testing::playbook_third_turn_arvn::PlaybookThirdTurnArvn;
use fire_in_the_lake::decision_making::testing::playbook_third_turn_vc::PlaybookThirdTurnVc;
use fire_in_the_lake::display::announcer::Announcer;
use fire_in_the_lake::factions::Factions;
use fire_in_the_lake::game_flow::game_flow_handler::GameFlowHandler;

#[test]
fn test_third_game_turn_playbook() -> Result<(), String> {
    let card_registry = CardRegistry::new();

    let mut game_flow_handler = GameFlowHandler::new(&card_registry);

    game_flow_handler.set_active_card(1);

    assert_eq!(
        game_flow_handler.get_active_card(),
        1,
        "After setting the active card, it should be the one expected."
    );

    game_flow_handler.set_faction_as_ineligible(Factions::ARVN);
    game_flow_handler.set_faction_as_ineligible(Factions::VC);

    assert_eq!(
        game_flow_handler.number_of_eligible_factions(),
        2,
        "The number of eligible factions should have been 2, but was {:?}",
        game_flow_handler.number_of_eligible_factions()
    );

    assert_eq!(
        game_flow_handler.get_current_eligible(),
        Factions::US,
        "The current eligible after first drawing {:?} should have been {:?}.",
        game_flow_handler.get_active_card(),
        Factions::US
    );

    let decision_making_center = DecisionMakingCenter::new(
        PlaybookThirdTurnVc::new().into(),
        PlaybookSecondTurnUs::new().into(),
        PlaybookFourthTurnUs::new().into(),
        PlaybookThirdTurnArvn::new().into(),
    );

    let mut built_map = MapBuilder::new().build_initial_map().unwrap();

    let mut track = Track::new();

    let mut available_forces = AvailableForces::new();

    let mut possible_us_decision = decision_making_center.decide(
        game_flow_handler.get_active_card(),
        game_flow_handler.get_current_eligible(),
        &built_map,
        &track,
        &available_forces,
    );

    let mut us_decision;

    match possible_us_decision {
        Ok(decision) => us_decision = decision,
        Err(error) => panic!("Something went wrong when producing the decision for US during the third turn. Error: {:?}", error)
    }

    assert_eq!(us_decision.get_choice(), Choices::UnshadedEvent);

    // Should inform the game_flow_handler
    game_flow_handler.report_choice(
        game_flow_handler.get_current_eligible(),
        us_decision.get_choice(),
    )?;

    assert_eq!(
        game_flow_handler.get_current_eligible(),
        Factions::NVA,
        "After US makes its choice, the next eligible should be the NVA."
    );
    assert_eq!(
        game_flow_handler.faction_present_in_first_eligible_event(),
        Factions::US,
    );
    assert_eq!(
        game_flow_handler.is_faction_eligible(Factions::US),
        false,
        "After making a choice, US should no longer be considered eligible."
    );

    // Setup board and all that
    built_map
        .get_space_mut(SpaceIdentifiers::QuangTriThuaThien)
        .unwrap()
        .set_number_of_underground_special_forces_irregulars(1);
    built_map
        .get_space_mut(SpaceIdentifiers::QuangTriThuaThien)
        .unwrap()
        .set_number_of_us_troops(1);
    built_map
        .get_space_mut(SpaceIdentifiers::QuangTriThuaThien)
        .unwrap()
        .set_number_of_active_vc_guerrillas(2);

    track.set_trail(2);

    available_forces.set_out_of_play_us_bases(1);
    available_forces.set_out_of_play_us_troops(5);

    let announcer = Announcer::new();

    // Execute the commands
    announcer
        .instruct_to_move_faction_cylinder_from_eligible_to_first_eligible_event_box(Factions::US);

    execute_commands(
        game_flow_handler.get_active_card(),
        us_decision.get_faction(),
        us_decision.get_interpreted_intentions(),
        &mut built_map,
        &mut track,
        &mut available_forces,
    );

    assert_eq!(
        built_map
            .get_space_mut(SpaceIdentifiers::QuangTriThuaThien)
            .unwrap()
            .get_number_of_active_vc_guerrillas(),
        0
    );

    assert_eq!(
        built_map
            .get_space_mut(SpaceIdentifiers::QuangTriThuaThien)
            .unwrap()
            .get_number_of_underground_special_forces_irregulars(),
        1
    );

    assert_eq!(
        built_map
            .get_space_mut(SpaceIdentifiers::QuangTriThuaThien)
            .unwrap()
            .get_number_of_us_troops(),
        1
    );

    assert_eq!(
        built_map
            .get_space_mut(SpaceIdentifiers::QuangTriThuaThien)
            .unwrap()
            .get_support_level(),
        SupportLevels::PassiveOpposition,
    );

    assert_eq!(track.get_vc_victory_marker(), 2,);

    assert_eq!(track.get_trail(), 1,);

    assert_eq!(
        built_map
            .get_space_mut(SpaceIdentifiers::Saigon)
            .unwrap()
            .get_number_of_us_troops(),
        2
    );

    assert_eq!(
        built_map
            .get_space_mut(SpaceIdentifiers::Hue)
            .unwrap()
            .get_number_of_us_troops(),
        3
    );

    Ok(())
}
