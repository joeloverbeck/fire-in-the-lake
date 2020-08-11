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
use fire_in_the_lake::decision_making::testing::playbook_second_turn_nva::PlaybookSecondTurnNva;
use fire_in_the_lake::decision_making::testing::playbook_second_turn_us::PlaybookSecondTurnUs;
use fire_in_the_lake::display::announcer::Announcer;
use fire_in_the_lake::factions::Factions;
use fire_in_the_lake::game_flow::game_flow_handler::GameFlowHandler;

#[test]
fn test_second_game_turn_playbook() -> Result<(), String> {
    let card_registry = CardRegistry::new();

    let mut game_flow_handler = GameFlowHandler::new(&card_registry);

    // Start. Game turn 2 (1/4)

    game_flow_handler.set_active_card(55);
    game_flow_handler.set_preview_card(68);

    assert_eq!(
        game_flow_handler.get_active_card(),
        55,
        "After setting the active card, it should be the one expected."
    );

    game_flow_handler.set_faction_as_ineligible(Factions::VC);
    game_flow_handler.set_faction_as_ineligible(Factions::ARVN);

    // Given the setup, only two factions should be eligible.
    assert_eq!(
        game_flow_handler.number_of_eligible_factions(),
        2,
        "The number of eligible factions should have been 2, but was {:?}",
        game_flow_handler.number_of_eligible_factions()
    );

    assert_eq!(
        game_flow_handler.get_current_eligible(),
        Factions::NVA,
        "The current eligible after first drawing {:?} should have been {:?}.",
        game_flow_handler.get_active_card(),
        Factions::NVA
    );

    // NVA decide to perform First Operation Only. Moves their eligibility token to execute First Op Only.

    let decision_making_center = DecisionMakingCenter::new(
        PlaybookSecondTurnNva::new().into(),
        PlaybookSecondTurnNva::new().into(),
        PlaybookSecondTurnUs::new().into(),
        PlaybookSecondTurnNva::new().into(),
    );

    let mut built_map = MapBuilder::new().build_initial_map().unwrap();

    let mut track = Track::new();

    let mut available_forces = AvailableForces::new();

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
        Err(error) => panic!("Something went wrong when producing the decision for NVA during the second turn. Error: {:?}", error)
    }

    assert_eq!(
        nva_decision.get_choice(),
        Choices::OperationOnly,
        "NVA's choice should have been Op Only."
    );

    // Should inform the game_flow_handler
    game_flow_handler.report_choice(
        game_flow_handler.get_current_eligible(),
        nva_decision.get_choice(),
    )?;

    // Given that NVA has chosen Op Only, and VC should have been ineligible, the next eligible should be the third in the card order.
    assert_eq!(
        game_flow_handler.get_current_eligible(),
        Factions::US,
        "After NVA makes its choice, the next eligible should be the US."
    );
    assert_eq!(
        game_flow_handler.faction_present_in_operation_only(),
        Factions::NVA,
        "NVA should have been registered as having chosen Operation Only.",
    );
    assert_eq!(
        game_flow_handler.is_faction_eligible(Factions::NVA),
        false,
        "After making a choice, NVA should no longer be considered eligible."
    );

    // Must add bases to north vietnam and parrot's beak
    built_map
        .get_space_mut(SpaceIdentifiers::NorthVietnam)
        .unwrap()
        .set_number_of_nva_bases(1);
    built_map
        .get_space_mut(SpaceIdentifiers::TheParrotsBeak)
        .unwrap()
        .set_number_of_nva_bases(1);

    built_map
        .get_space_mut(SpaceIdentifiers::KienGiangAnXuyen)
        .unwrap()
        .set_support_level(SupportLevels::ActiveOpposition);
    built_map
        .get_space_mut(SpaceIdentifiers::KienPhong)
        .unwrap()
        .set_support_level(SupportLevels::ActiveOpposition);

    track.set_nva_resources(11);
    track.set_trail(1);

    available_forces.set_amount_of_nva_guerrillas(10);

    let announcer = Announcer::new();

    // Execute the commands
    announcer.instruct_to_move_faction_cylinder_from_eligible_to_operation_only_box(Factions::NVA);

    // Must execute the pass command, because it gives resources.
    execute_commands(
        game_flow_handler.get_active_card(),
        nva_decision.get_faction(),
        nva_decision.get_interpreted_intentions(),
        &mut built_map,
        &mut track,
        &mut available_forces,
    );

    assert_eq!(
        built_map
            .get_space_mut(SpaceIdentifiers::NorthVietnam)
            .unwrap()
            .get_number_of_underground_nva_guerrillas(),
        2
    );
    assert_eq!(
        built_map
            .get_space_mut(SpaceIdentifiers::TheParrotsBeak)
            .unwrap()
            .get_number_of_underground_nva_guerrillas(),
        2
    );
    assert_eq!(
        built_map
            .get_space_mut(SpaceIdentifiers::KienGiangAnXuyen)
            .unwrap()
            .get_number_of_underground_nva_guerrillas(),
        1
    );
    assert_eq!(
        built_map
            .get_space_mut(SpaceIdentifiers::KienPhong)
            .unwrap()
            .get_number_of_underground_nva_guerrillas(),
        1
    );

    assert_eq!(
        track.get_nva_resources(),
        5,
        "At the end of all NVA's actions, their resources should have been at 5, but were {:?}",
        track.get_nva_resources()
    );
    assert_eq!(
        track.get_trail(),
        2,
        "At the end of all NVA's actions, their trail should have been at 2, but was {:?}",
        track.get_trail()
    );

    // NVA shouldn't have control in Kien Giang nor in Kien Phong
    let kien_giang = built_map
        .get_space(SpaceIdentifiers::KienGiangAnXuyen)
        .unwrap();
    assert_eq!(
        kien_giang.get_control(),
        Controls::Uncontrolled,
        "Kien Giang should have remained Uncontrolled"
    );

    let kien_phong = built_map.get_space(SpaceIdentifiers::KienPhong).unwrap();
    assert_eq!(
        kien_phong.get_control(),
        Controls::Uncontrolled,
        "Kien Phong should have remained Uncontrolled"
    );

    // Because the others are ineligible, it's US' turn.
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
        Err(error) => panic!("Something went wrong when producing the decision for US during the second turn. Error: {:?}", error)
    }

    assert_eq!(
        us_decision.get_choice(),
        Choices::SecondLimitedOperation,
        "US's choice should have been Second Limited Operation."
    );

    // Should inform the game_flow_handler
    game_flow_handler.report_choice(
        game_flow_handler.get_current_eligible(),
        us_decision.get_choice(),
    )?;

    // Given that US has chosen Op Only, and the remaining are ineligible, there should be no current eligible.
    assert_eq!(
        game_flow_handler.get_current_eligible(),
        Factions::None,
        "After US makes its choice, there should be no eligible faction."
    );
    assert_eq!(
        game_flow_handler.faction_present_in_second_limited_operation(),
        Factions::US,
        "US should have been registered as having chosen Second Limited Operation.",
    );
    assert_eq!(
        game_flow_handler.is_faction_eligible(Factions::US),
        false,
        "After making a choice, US should no longer be considered eligible."
    );

    // Execute the commands
    announcer.instruct_to_move_faction_cylinder_from_eligible_to_second_limited_operation_box(
        Factions::US,
    );

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
        .set_number_of_underground_vc_guerrillas(2);

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
            .get_number_of_underground_special_forces_irregulars(),
        1,
        "There should have been a unit of special forces irregulars"
    );

    assert_eq!(
        built_map
            .get_space_mut(SpaceIdentifiers::QuangTriThuaThien)
            .unwrap()
            .get_number_of_us_troops(),
        1,
        "There should have been a unit of us troops"
    );

    assert_eq!(
        built_map
            .get_space_mut(SpaceIdentifiers::QuangTriThuaThien)
            .unwrap()
            .get_number_of_active_vc_guerrillas(),
        2,
        "There should be two active units of vc guerrillas."
    );

    // Two eligible factions have acted, so the turn is over.
    assert_eq!(game_flow_handler.has_turn_ended(), true);

    game_flow_handler.perform_end_of_turn();

    assert_eq!(game_flow_handler.is_faction_eligible(Factions::NVA), false);
    assert_eq!(game_flow_handler.is_faction_eligible(Factions::US), false);
    assert_eq!(
        game_flow_handler.is_faction_eligible(Factions::VC),
        true,
        "VC should have been eligible, as it didn't act this turn."
    );
    assert_eq!(
        game_flow_handler.is_faction_eligible(Factions::ARVN),
        true,
        "ARVN should have been eligible, as it didn't act this turn."
    );

    assert_eq!(
        game_flow_handler.get_active_card(),
        68,
        "The current card should be 'Green Berets'."
    );

    Ok(())
}
