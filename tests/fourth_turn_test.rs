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

    Ok(())
}
