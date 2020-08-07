extern crate fire_in_the_lake;

use fire_in_the_lake::cards::card_registry::CardRegistry;
use fire_in_the_lake::decision_making::choices::Choices;
use fire_in_the_lake::decision_making::commands_producer::CommandsProducer;
use fire_in_the_lake::decision_making::decision::Decision;
use fire_in_the_lake::factions::Factions;
use fire_in_the_lake::game_flow::game_flow_handler::GameFlowHandler;
use fire_in_the_lake::game_flow::sequence_of_play::SequenceOfPlay;

struct FakeCommandsProducer {}

impl CommandsProducer for FakeCommandsProducer {
    fn decide(&self, active_card: u8, current_eligible: Factions) -> Decision {
        // Normally, in a real struct for this trait, there should be a decision making process, whether an AI one
        // or asking the human player for input, but in this case we know what we have to return depending of the current eligible faction.
        if current_eligible == Factions::VC {
            // We need to execute the shaded event.
            // It should be like this: I create a Decision and pass to it the choice (ShadedEvent) as well as the
            // Commands that are ALL THE WAYS in which this current faction affects the board due to his choice. They should be in a deque
            // or something.
            let decision = Decision::new(Choices::ShadedEvent);

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
fn test_first_game_turn() -> Result<(), String> {
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

    let vc_decision = commands_producer.decide(
        game_flow_handler.get_active_card(),
        game_flow_handler.get_current_eligible(),
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
    );

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

    Ok(())
}
