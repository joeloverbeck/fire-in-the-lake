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
use fire_in_the_lake::decision_making::testing::playbook_third_turn_arvn::PlaybookThirdTurnArvn;
use fire_in_the_lake::display::announcer::Announcer;
use fire_in_the_lake::factions::Factions;
use fire_in_the_lake::game_flow::game_flow_handler::GameFlowHandler;
use fire_in_the_lake::game_flow::sequence_of_play::SequenceOfPlay;

#[test]
fn test_third_game_turn_playbook() -> Result<(), String> {
    let card_registry = CardRegistry::new();
    let mut sequence_of_play = SequenceOfPlay::new();

    let mut game_flow_handler = GameFlowHandler::new(&card_registry, &mut sequence_of_play);

    // Start. Game turn 3 (1/4)
    game_flow_handler.set_active_card(68);
    game_flow_handler.set_preview_card(1);

    assert_eq!(
        game_flow_handler.get_active_card(),
        68,
        "After setting the active card, it should be the one expected."
    );

    game_flow_handler.set_faction_as_ineligible(Factions::NVA);
    game_flow_handler.set_faction_as_ineligible(Factions::US);

    // Given the setup, only two factions should be eligible.
    assert_eq!(
        game_flow_handler.number_of_eligible_factions(),
        2,
        "The number of eligible factions should have been 2, but was {:?}",
        game_flow_handler.number_of_eligible_factions()
    );

    assert_eq!(
        game_flow_handler.get_current_eligible(),
        Factions::ARVN,
        "The current eligible after first drawing {:?} should have been {:?}.",
        game_flow_handler.get_active_card(),
        Factions::ARVN
    );

    let decision_making_center = DecisionMakingCenter::new(
        PlaybookSecondTurnNva::new().into(),
        PlaybookSecondTurnUs::new().into(),
        PlaybookSecondTurnNva::new().into(),
        PlaybookThirdTurnArvn::new().into(),
    );

    let mut built_map = MapBuilder::new().build_initial_map().unwrap();

    let mut track = Track::new();

    let mut available_forces = AvailableForces::new();

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
        Err(error) => panic!("Something went wrong when producing the decision for ARVN during the third turn. Error: {:?}", error)
    }

    assert_eq!(
        arvn_decision.get_choice(),
        Choices::UnshadedEvent,
        "NVA's choice should have been Op Only."
    );

    // Should inform the game_flow_handler
    game_flow_handler.report_choice(
        game_flow_handler.get_current_eligible(),
        arvn_decision.get_choice(),
    )?;

    assert_eq!(
        game_flow_handler.get_current_eligible(),
        Factions::VC,
        "After ARVN makes its choice, the next eligible should be the VC."
    );
    assert_eq!(
        game_flow_handler.faction_present_in_first_eligible_event(),
        Factions::ARVN,
        "ARVN should have been registered as having chosen First Faction Event.",
    );
    assert_eq!(
        game_flow_handler.is_faction_eligible(Factions::ARVN),
        false,
        "After making a choice, ARVN should no longer be considered eligible."
    );

    // Setup board and all that
    available_forces.set_amount_of_us_irregulars(6);

    // Binh Dinh already had a troop and an irregular in the example.
    built_map
        .get_space_mut(SpaceIdentifiers::BinhDinh)
        .unwrap()
        .set_number_of_underground_special_forces_irregulars(1);
    built_map
        .get_space_mut(SpaceIdentifiers::BinhDinh)
        .unwrap()
        .set_number_of_us_troops(1);

    // Binh Dinh already had a vc base and two active vc guerrillas.
    built_map
        .get_space_mut(SpaceIdentifiers::BinhDinh)
        .unwrap()
        .set_number_of_active_vc_guerrillas(2);
    built_map
        .get_space_mut(SpaceIdentifiers::BinhDinh)
        .unwrap()
        .set_number_of_vc_bases(1);

    track.set_patronage(35);

    let announcer = Announcer::new();

    // Execute the commands
    announcer.instruct_to_move_faction_cylinder_from_eligible_to_first_eligible_event_box(
        Factions::ARVN,
    );

    // Must execute the pass command, because it gives resources.
    execute_commands(
        game_flow_handler.get_active_card(),
        arvn_decision.get_faction(),
        arvn_decision.get_commands(),
        &mut built_map,
        &mut track,
        &mut available_forces,
    );

    assert_eq!(
        built_map
            .get_space_mut(SpaceIdentifiers::BinhDinh)
            .unwrap()
            .get_total_number_of_us_pieces(),
        5,
        "The total number of US pieces should have been 5, but was {:?}.",
        built_map
            .get_space_mut(SpaceIdentifiers::BinhDinh)
            .unwrap()
            .get_total_number_of_us_pieces()
    );
    assert_eq!(
        built_map
            .get_space_mut(SpaceIdentifiers::BinhDinh)
            .unwrap()
            .get_total_number_of_vc_pieces(),
        3,
        "The total number of VC pieces should have been 3, but was {:?}.",
        built_map
            .get_space_mut(SpaceIdentifiers::BinhDinh)
            .unwrap()
            .get_total_number_of_vc_pieces()
    );

    assert_eq!(
        built_map
            .get_space_mut(SpaceIdentifiers::BinhDinh)
            .unwrap()
            .get_control(),
        Controls::Counterinsurgent,
        "The control of the province should have been Counterinsurgent, but was {:?}.",
        built_map
            .get_space_mut(SpaceIdentifiers::BinhDinh)
            .unwrap()
            .get_control()
    );

    assert_eq!(
        built_map
            .get_space_mut(SpaceIdentifiers::BinhDinh)
            .unwrap()
            .get_support_level(),
        SupportLevels::ActiveSupport,
        "The support level should have been ActiveSupport"
    );

    assert_eq!(
        track.get_control_plus_patronage(),
        37,
        "Control + Patronage should be 37, but was {:?}",
        track.get_control_plus_patronage()
    );

    // VC's turn.
    assert_eq!(
        game_flow_handler.get_current_eligible(),
        Factions::VC,
        "After ARVN makes its choice, the next eligible should be the VC."
    );

    // They choose an Operation & Special Activity

    // Prove VC's turn
    //assert_eq!(track.get_vc_victory_marker(), 23, "VC's victory marker should have been 23, but was {:?}", track.get_vc_victory_marker());

    Ok(())
}