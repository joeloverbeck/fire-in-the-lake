extern crate fire_in_the_lake;

use fire_in_the_lake::board::map::Map;
use fire_in_the_lake::board::map_builder::MapBuilder;
use fire_in_the_lake::board::space::Space;
use fire_in_the_lake::board::space_identifiers::SpaceIdentifiers;
use fire_in_the_lake::board::support::SupportLevels;
use fire_in_the_lake::board::track::Track;
use fire_in_the_lake::cards::card_registry::CardRegistry;
use fire_in_the_lake::commands::command::Commands;
use fire_in_the_lake::commands::manipulate_aid::ManipulateAid;
use fire_in_the_lake::commands::shift_support_of_space::ShiftSupportOfSpace;
use fire_in_the_lake::decision_making::choices::Choices;
use fire_in_the_lake::decision_making::commands_producer::CommandsProducer;
use fire_in_the_lake::decision_making::decision::Decision;
use fire_in_the_lake::display::announcer::Announcer;
use fire_in_the_lake::factions::Factions;
use fire_in_the_lake::game_flow::game_flow_handler::GameFlowHandler;
use fire_in_the_lake::game_flow::sequence_of_play::SequenceOfPlay;
use std::collections::VecDeque;

struct FakeCommandsProducer {}

impl<'a> CommandsProducer<'a> for FakeCommandsProducer {
    fn decide(
        &self,
        active_card: u8,
        current_eligible: Factions,
        map: &'a mut Map,
        track: &'a mut Track,
    ) -> Decision<'a> {
        // Normally, in a real struct for this trait, there should be a decision making process, whether an AI one
        // or asking the human player for input, but in this case we know what we have to return depending of the current eligible faction.
        if current_eligible == Factions::VC {
            // We need to execute the shaded event.
            // It should be like this: I create a Decision and pass to it the choice (ShadedEvent) as well as the
            // Commands that are ALL THE WAYS in which this current faction affects the board due to his choice. They should be in a deque
            // or something.

            // Create command to shift down support level in Saigon
            let shift_support_of_space: Commands =
                ShiftSupportOfSpace::new(map.get_space_mut(SpaceIdentifiers::Saigon).unwrap(), -1)
                    .into();

            // Create command to lower aid
            let manipulate_aid: Commands = ManipulateAid::new(track, -12).into();

            let mut buf: VecDeque<Commands> = VecDeque::new();

            buf.push_back(shift_support_of_space);
            buf.push_back(manipulate_aid);

            let decision = Decision::new(Choices::ShadedEvent, buf);

            return decision;
        }

        todo!()
    }
}

impl FakeCommandsProducer {
    fn new() -> FakeCommandsProducer {
        FakeCommandsProducer {}
    }
}

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
    let commands_producer = FakeCommandsProducer::new();

    let map_builder = MapBuilder::new();

    let mut built_map = map_builder.build_initial_map().unwrap();

    let mut track = Track::new();

    // Set up map to initial state.
    built_map
        .get_space_mut(SpaceIdentifiers::Saigon)
        .unwrap()
        .set_support_level(SupportLevels::PassiveSupport);
    track.set_aid(15);

    let mut vc_decision = commands_producer.decide(
        game_flow_handler.get_active_card(),
        game_flow_handler.get_current_eligible(),
        &mut built_map,
        &mut track,
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

    // Now for the real complicated stuff. The commands should have been received in the Decision structure,
    // and in a deque containing somehow different kinds of Commands. If I solve this, a significant part of
    // the entire project will be solved.
    vc_decision.execute_commands();
    track.adjust_us_victory_marker(&built_map);

    // The commands should have shifted Saigon's Passive Support to Neutral.
    assert_eq!(
        built_map
            .get_space(SpaceIdentifiers::Saigon)
            .unwrap()
            .get_support_level(),
        SupportLevels::Neutral
    );
    assert_eq!(
        track.aid(),
        3,
        "After executing VC's commands, Aid should have been 3, but was {:?}",
        track.aid()
    );

    Ok(())
}
