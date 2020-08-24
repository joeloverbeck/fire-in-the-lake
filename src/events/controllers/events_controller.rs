use board::domain::board::Board;
use cards::domain::card::Card;
use cards::domain::card::Cards;
use events::domain::effectivity::check_event_effectivity_for_card_1::check_event_effectivity_for_card_1;
use events::domain::effectivity::check_event_effectivity_for_card_10::check_event_effectivity_for_card_10;
use events::domain::effectivity::check_event_effectivity_for_card_100::check_event_effectivity_for_card_100;
use events::domain::effectivity::check_event_effectivity_for_card_102::check_event_effectivity_for_card_102;
use events::domain::effectivity::check_event_effectivity_for_card_103::check_event_effectivity_for_card_103;
use events::domain::effectivity::check_event_effectivity_for_card_105::check_event_effectivity_for_card_105;
use events::domain::effectivity::check_event_effectivity_for_card_106::check_event_effectivity_for_card_106;
use events::domain::effectivity::check_event_effectivity_for_card_107::check_event_effectivity_for_card_107;
use events::domain::effectivity::check_event_effectivity_for_card_108::check_event_effectivity_for_card_108;
use events::domain::effectivity::check_event_effectivity_for_card_110::check_event_effectivity_for_card_110;
use events::domain::effectivity::check_event_effectivity_for_card_111::check_event_effectivity_for_card_111;
use events::domain::effectivity::check_event_effectivity_for_card_112::check_event_effectivity_for_card_112;
use events::domain::effectivity::check_event_effectivity_for_card_113::check_event_effectivity_for_card_113;
use events::domain::effectivity::check_event_effectivity_for_card_114::check_event_effectivity_for_card_114;
use events::domain::effectivity::check_event_effectivity_for_card_115::check_event_effectivity_for_card_115;
use events::domain::effectivity::check_event_effectivity_for_card_116::check_event_effectivity_for_card_116;
use events::domain::effectivity::check_event_effectivity_for_card_117::check_event_effectivity_for_card_117;
use events::domain::effectivity::check_event_effectivity_for_card_118::check_event_effectivity_for_card_118;
use events::domain::effectivity::check_event_effectivity_for_card_119::check_event_effectivity_for_card_119;
use events::domain::effectivity::check_event_effectivity_for_card_12::check_event_effectivity_for_card_12;
use events::domain::effectivity::check_event_effectivity_for_card_120::check_event_effectivity_for_card_120;
use events::domain::effectivity::check_event_effectivity_for_card_15::check_event_effectivity_for_card_15;
use events::domain::effectivity::check_event_effectivity_for_card_16::check_event_effectivity_for_card_16;
use events::domain::effectivity::check_event_effectivity_for_card_17::check_event_effectivity_for_card_17;
use events::domain::effectivity::check_event_effectivity_for_card_2::check_event_effectivity_for_card_2;
use events::domain::effectivity::check_event_effectivity_for_card_21::check_event_effectivity_for_card_21;
use events::domain::effectivity::check_event_effectivity_for_card_22::check_event_effectivity_for_card_22;
use events::domain::effectivity::check_event_effectivity_for_card_23::check_event_effectivity_for_card_23;
use events::domain::effectivity::check_event_effectivity_for_card_24::check_event_effectivity_for_card_24;
use events::domain::effectivity::check_event_effectivity_for_card_25::check_event_effectivity_for_card_25;
use events::domain::effectivity::check_event_effectivity_for_card_26::check_event_effectivity_for_card_26;
use events::domain::effectivity::check_event_effectivity_for_card_27::check_event_effectivity_for_card_27;
use events::domain::effectivity::check_event_effectivity_for_card_29::check_event_effectivity_for_card_29;
use events::domain::effectivity::check_event_effectivity_for_card_3::check_event_effectivity_for_card_3;
use events::domain::effectivity::check_event_effectivity_for_card_30::check_event_effectivity_for_card_30;
use events::domain::effectivity::check_event_effectivity_for_card_35::check_event_effectivity_for_card_35;
use events::domain::effectivity::check_event_effectivity_for_card_36::check_event_effectivity_for_card_36;
use events::domain::effectivity::check_event_effectivity_for_card_38::check_event_effectivity_for_card_38;
use events::domain::effectivity::check_event_effectivity_for_card_39::check_event_effectivity_for_card_39;
use events::domain::effectivity::check_event_effectivity_for_card_40::check_event_effectivity_for_card_40;
use events::domain::effectivity::check_event_effectivity_for_card_41::check_event_effectivity_for_card_41;
use events::domain::effectivity::check_event_effectivity_for_card_42::check_event_effectivity_for_card_42;
use events::domain::effectivity::check_event_effectivity_for_card_43::check_event_effectivity_for_card_43;
use events::domain::effectivity::check_event_effectivity_for_card_44::check_event_effectivity_for_card_44;
use events::domain::effectivity::check_event_effectivity_for_card_46::check_event_effectivity_for_card_46;
use events::domain::effectivity::check_event_effectivity_for_card_47::check_event_effectivity_for_card_47;
use events::domain::effectivity::check_event_effectivity_for_card_48::check_event_effectivity_for_card_48;
use events::domain::effectivity::check_event_effectivity_for_card_49::check_event_effectivity_for_card_49;
use events::domain::effectivity::check_event_effectivity_for_card_5::check_event_effectivity_for_card_5;
use events::domain::effectivity::check_event_effectivity_for_card_50::check_event_effectivity_for_card_50;
use events::domain::effectivity::check_event_effectivity_for_card_51::check_event_effectivity_for_card_51;
use events::domain::effectivity::check_event_effectivity_for_card_52::check_event_effectivity_for_card_52;
use events::domain::effectivity::check_event_effectivity_for_card_53::check_event_effectivity_for_card_53;
use events::domain::effectivity::check_event_effectivity_for_card_54::check_event_effectivity_for_card_54;
use events::domain::effectivity::check_event_effectivity_for_card_55::check_event_effectivity_for_card_55;
use events::domain::effectivity::check_event_effectivity_for_card_56::check_event_effectivity_for_card_56;
use events::domain::effectivity::check_event_effectivity_for_card_57::check_event_effectivity_for_card_57;
use events::domain::effectivity::check_event_effectivity_for_card_58::check_event_effectivity_for_card_58;
use events::domain::effectivity::check_event_effectivity_for_card_59::check_event_effectivity_for_card_59;
use events::domain::effectivity::check_event_effectivity_for_card_6::check_event_effectivity_for_card_6;
use events::domain::effectivity::check_event_effectivity_for_card_60::check_event_effectivity_for_card_60;
use events::domain::effectivity::check_event_effectivity_for_card_62::check_event_effectivity_for_card_62;
use events::domain::effectivity::check_event_effectivity_for_card_63::check_event_effectivity_for_card_63;
use events::domain::effectivity::check_event_effectivity_for_card_64::check_event_effectivity_for_card_64;
use events::domain::effectivity::check_event_effectivity_for_card_65::check_event_effectivity_for_card_65;
use events::domain::effectivity::check_event_effectivity_for_card_67::check_event_effectivity_for_card_67;
use events::domain::effectivity::check_event_effectivity_for_card_68::check_event_effectivity_for_card_68;
use events::domain::effectivity::check_event_effectivity_for_card_69::check_event_effectivity_for_card_69;
use events::domain::effectivity::check_event_effectivity_for_card_7::check_event_effectivity_for_card_7;
use events::domain::effectivity::check_event_effectivity_for_card_70::check_event_effectivity_for_card_70;
use events::domain::effectivity::check_event_effectivity_for_card_71::check_event_effectivity_for_card_71;
use events::domain::effectivity::check_event_effectivity_for_card_72::check_event_effectivity_for_card_72;
use events::domain::effectivity::check_event_effectivity_for_card_73::check_event_effectivity_for_card_73;
use events::domain::effectivity::check_event_effectivity_for_card_74::check_event_effectivity_for_card_74;
use events::domain::effectivity::check_event_effectivity_for_card_75::check_event_effectivity_for_card_75;
use events::domain::effectivity::check_event_effectivity_for_card_76::check_event_effectivity_for_card_76;
use events::domain::effectivity::check_event_effectivity_for_card_77::check_event_effectivity_for_card_77;
use events::domain::effectivity::check_event_effectivity_for_card_78::check_event_effectivity_for_card_78;
use events::domain::effectivity::check_event_effectivity_for_card_79::check_event_effectivity_for_card_79;
use events::domain::effectivity::check_event_effectivity_for_card_80::check_event_effectivity_for_card_80;
use events::domain::effectivity::check_event_effectivity_for_card_81::check_event_effectivity_for_card_81;
use events::domain::effectivity::check_event_effectivity_for_card_82::check_event_effectivity_for_card_82;
use events::domain::effectivity::check_event_effectivity_for_card_84::check_event_effectivity_for_card_84;
use events::domain::effectivity::check_event_effectivity_for_card_85::check_event_effectivity_for_card_85;
use events::domain::effectivity::check_event_effectivity_for_card_87::check_event_effectivity_for_card_87;
use events::domain::effectivity::check_event_effectivity_for_card_88::check_event_effectivity_for_card_88;
use events::domain::effectivity::check_event_effectivity_for_card_89::check_event_effectivity_for_card_89;
use events::domain::effectivity::check_event_effectivity_for_card_9::check_event_effectivity_for_card_9;
use events::domain::effectivity::check_event_effectivity_for_card_90::check_event_effectivity_for_card_90;
use events::domain::effectivity::check_event_effectivity_for_card_93::check_event_effectivity_for_card_93;
use events::domain::effectivity::check_event_effectivity_for_card_95::check_event_effectivity_for_card_95;
use events::domain::effectivity::check_event_effectivity_for_card_96::check_event_effectivity_for_card_96;
use events::domain::effectivity::check_event_effectivity_for_card_97::check_event_effectivity_for_card_97;
use events::domain::effectivity::check_event_effectivity_for_card_98::check_event_effectivity_for_card_98;
use events::domain::effectivity::check_event_effectivity_for_card_99::check_event_effectivity_for_card_99;
use flags::controllers::flags_controller::FlagsController;
use game_definitions::event_types::EventTypes;
use game_definitions::faction_stats::FactionStats;
use game_definitions::factions::Factions;
use game_definitions::flags::Flags;
use game_definitions::forces::Forces;
use game_definitions::space_identifiers::SpaceIdentifiers;
use players::domain::decision::Decision;
use players::domain::decision_information::DecisionInformation;
use players::domain::faction_stats_mutation::FactionStatsMutation;
use players::domain::flags_mutation::FlagsMutation;
use players::domain::mutation_types::MutationTypes;
use players::domain::mutations::Mutations;
use players::domain::mutations_production::request_forces_movement_from_human::request_forces_movement_from_human;
use players::domain::player_type::PlayerType;
use players::domain::sequence_of_play_mutation::SequenceOfPlayMutation;
use sequence_of_play::controllers::sequence_of_play_controller::SequenceOfPlayController;
use sequence_of_play::domain::sequence_of_play_slots::SequenceOfPlaySlots;
use sequence_of_play::domain::slot_occupancy::SlotOccupancy;
use std::collections::HashMap;
use user_interface::controllers::display_controller::DisplayController;
use user_interface::controllers::keyboard_input_controller::KeyboardInputController;

pub struct EventsController {}

impl Default for EventsController {
    fn default() -> Self {
        Self::new()
    }
}

impl EventsController {
    pub fn new() -> EventsController {
        EventsController {}
    }

    pub fn produce_decision_for_unshaded_event_when_us_human(
        &self,
        active_card: &Cards,
        board: &Board,
        keyboard_input_controller: &KeyboardInputController,
        display_controller: &DisplayController,
    ) -> Result<Decision, String> {
        let mut sequence_of_play_mutations: Vec<SequenceOfPlayMutation> = Vec::new();
        sequence_of_play_mutations.push(SequenceOfPlayMutation::new(
            SequenceOfPlaySlots::FirstFactionEvent,
            SlotOccupancy::Occupied,
            Factions::US,
        ));

        if active_card.get_number()? == 16 {
            // Aid +10. This Support phase, Pacify costs 1 Resource per step or Terror. MOMENTUM.
            let mut faction_stat_mutations: Vec<FactionStatsMutation> = Vec::new();
            faction_stat_mutations.push(FactionStatsMutation::new(
                FactionStats::Aid,
                MutationTypes::Increase,
                board.get_faction_stat(FactionStats::Aid)?,
                10,
            ));

            let mut flag_mutations: Vec<FlagsMutation> = Vec::new();
            flag_mutations.push(FlagsMutation::new(
                Flags::BlowtorchComer,
                MutationTypes::Set,
            ));

            let mut mutations = Mutations::new();

            mutations.set_sequence_of_play_mutations(sequence_of_play_mutations)?;
            mutations.set_faction_stats_mutations(faction_stat_mutations)?;
            mutations.set_flags_mutations(flag_mutations)?;

            Ok(Decision::new(
                mutations,
                Some(DecisionInformation::Event),
                None,
            ))
        } else if active_card.get_number()? == 22 {
            // US places up to 6 Troops in Da Nang, up to 3 from out of play.

            // Unfortunately in this case we require user input
            Ok(request_forces_movement_from_human(
                Forces::UsTroop,
                SpaceIdentifiers::DaNang,
                6,
                Some((SpaceIdentifiers::OutOfPlay, 3)),
                board,
                keyboard_input_controller,
                display_controller,
            )?)
        } else {
            panic!("Not contemplated for {:?}", active_card);
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn is_current_non_capability_event_effective(
        &self,
        active_card: &Cards,
        preview_card: &Cards,
        player_types: HashMap<Factions, PlayerType>,
        faction: &Factions,
        preferible_event_type: EventTypes,
        board: &Board,
        flags_controller: &FlagsController,
        sequence_of_play_controller: &SequenceOfPlayController,
    ) -> Result<bool, String> {
        // Sanity check: we specify that the event should be non-capability. That means that the caller should have checked first whether it was.
        if active_card.has_any_faction_capability()? {
            // Don't hard crash, though.
            return Ok(false);
        }

        match active_card.get_number()? {
            1 => Ok(check_event_effectivity_for_card_1(
                active_card,
                preview_card,
                player_types,
                faction,
                preferible_event_type,
                board,
            )?),
            2 => Ok(check_event_effectivity_for_card_2(
                active_card,
                preview_card,
                player_types,
                faction,
                preferible_event_type,
                board,
            )?),
            3 => Ok(check_event_effectivity_for_card_3(
                active_card,
                preview_card,
                player_types,
                faction,
                preferible_event_type,
                board,
            )?),
            5 => Ok(check_event_effectivity_for_card_5(
                active_card,
                preview_card,
                player_types,
                faction,
                preferible_event_type,
                board,
            )?),
            6 => Ok(check_event_effectivity_for_card_6(
                active_card,
                preview_card,
                player_types,
                faction,
                preferible_event_type,
                board,
            )?),
            7 => Ok(check_event_effectivity_for_card_7(
                active_card,
                preview_card,
                player_types,
                faction,
                preferible_event_type,
                board,
            )?),
            9 => Ok(check_event_effectivity_for_card_9(
                active_card,
                preview_card,
                player_types,
                faction,
                preferible_event_type,
                board,
            )?),
            10 => Ok(check_event_effectivity_for_card_10(
                active_card,
                preview_card,
                player_types,
                faction,
                preferible_event_type,
                board,
            )?),
            12 => Ok(check_event_effectivity_for_card_12(
                active_card,
                preview_card,
                player_types,
                faction,
                preferible_event_type,
                board,
            )?),
            15 => Ok(check_event_effectivity_for_card_15(
                active_card,
                preview_card,
                player_types,
                faction,
                preferible_event_type,
                board,
            )?),
            16 => Ok(check_event_effectivity_for_card_16(
                active_card,
                preview_card,
                player_types,
                faction,
                preferible_event_type,
                board,
            )?),
            17 => Ok(check_event_effectivity_for_card_17(
                active_card,
                preview_card,
                player_types,
                faction,
                preferible_event_type,
                board,
            )?),
            21 => Ok(check_event_effectivity_for_card_21(
                active_card,
                preview_card,
                player_types,
                faction,
                preferible_event_type,
                board,
            )?),
            22 => Ok(check_event_effectivity_for_card_22(
                active_card,
                preview_card,
                player_types,
                faction,
                preferible_event_type,
                board,
            )?),
            23 => Ok(check_event_effectivity_for_card_23(
                active_card,
                preview_card,
                player_types,
                faction,
                preferible_event_type,
                board,
            )?),
            24 => Ok(check_event_effectivity_for_card_24(
                active_card,
                preview_card,
                player_types,
                faction,
                preferible_event_type,
                board,
            )?),
            25 => Ok(check_event_effectivity_for_card_25(
                active_card,
                preview_card,
                player_types,
                faction,
                preferible_event_type,
                board,
            )?),
            26 => Ok(check_event_effectivity_for_card_26(
                active_card,
                preview_card,
                player_types,
                faction,
                preferible_event_type,
                board,
            )?),
            27 => Ok(check_event_effectivity_for_card_27(
                active_card,
                preview_card,
                player_types,
                faction,
                preferible_event_type,
                board,
            )?),
            29 => Ok(check_event_effectivity_for_card_29(
                active_card,
                preview_card,
                player_types,
                faction,
                preferible_event_type,
                board,
            )?),
            30 => Ok(check_event_effectivity_for_card_30(
                active_card,
                preview_card,
                player_types,
                faction,
                preferible_event_type,
                board,
            )?),
            35 => Ok(check_event_effectivity_for_card_35(
                active_card,
                preview_card,
                player_types,
                faction,
                preferible_event_type,
                board,
            )?),
            36 => Ok(check_event_effectivity_for_card_36(
                active_card,
                preview_card,
                player_types,
                faction,
                preferible_event_type,
                board,
            )?),
            38 => Ok(check_event_effectivity_for_card_38(
                active_card,
                preview_card,
                player_types,
                faction,
                preferible_event_type,
                board,
            )?),
            39 => Ok(check_event_effectivity_for_card_39(
                active_card,
                preview_card,
                player_types,
                faction,
                preferible_event_type,
                board,
            )?),
            40 => Ok(check_event_effectivity_for_card_40(
                active_card,
                preview_card,
                player_types,
                faction,
                preferible_event_type,
                board,
            )?),
            41 => Ok(check_event_effectivity_for_card_41(
                active_card,
                preview_card,
                player_types,
                faction,
                preferible_event_type,
                board,
            )?),
            42 => Ok(check_event_effectivity_for_card_42(
                active_card,
                preview_card,
                player_types,
                faction,
                preferible_event_type,
                board,
            )?),
            43 => Ok(check_event_effectivity_for_card_43(
                active_card,
                preview_card,
                player_types,
                faction,
                preferible_event_type,
                board,
            )?),
            44 => Ok(check_event_effectivity_for_card_44(
                active_card,
                preview_card,
                player_types,
                faction,
                preferible_event_type,
                board,
            )?),
            46 => Ok(check_event_effectivity_for_card_46(
                active_card,
                preview_card,
                player_types,
                faction,
                preferible_event_type,
                board,
            )?),
            47 => Ok(check_event_effectivity_for_card_47(
                active_card,
                preview_card,
                player_types,
                faction,
                preferible_event_type,
                board,
            )?),
            48 => Ok(check_event_effectivity_for_card_48(
                active_card,
                preview_card,
                player_types,
                faction,
                preferible_event_type,
                board,
            )?),
            49 => Ok(check_event_effectivity_for_card_49(
                active_card,
                preview_card,
                player_types,
                faction,
                preferible_event_type,
                board,
            )?),
            50 => Ok(check_event_effectivity_for_card_50(
                active_card,
                preview_card,
                player_types,
                faction,
                preferible_event_type,
                board,
            )?),
            51 => Ok(check_event_effectivity_for_card_51(
                active_card,
                preview_card,
                player_types,
                faction,
                preferible_event_type,
                board,
            )?),
            52 => Ok(check_event_effectivity_for_card_52(
                active_card,
                preview_card,
                player_types,
                faction,
                preferible_event_type,
                board,
                flags_controller,
            )?),
            53 => Ok(check_event_effectivity_for_card_53(
                active_card,
                preview_card,
                player_types,
                faction,
                preferible_event_type,
                board,
            )?),
            54 => Ok(check_event_effectivity_for_card_54(
                active_card,
                preview_card,
                player_types,
                faction,
                preferible_event_type,
                board,
                sequence_of_play_controller,
            )?),
            55 => Ok(check_event_effectivity_for_card_55(
                active_card,
                preview_card,
                player_types,
                faction,
                preferible_event_type,
                board,
            )?),
            56 => Ok(check_event_effectivity_for_card_56(
                active_card,
                preview_card,
                player_types,
                faction,
                preferible_event_type,
                board,
            )?),
            57 => Ok(check_event_effectivity_for_card_57(
                active_card,
                preview_card,
                player_types,
                faction,
                preferible_event_type,
                board,
            )?),
            58 => Ok(check_event_effectivity_for_card_58(
                active_card,
                preview_card,
                player_types,
                faction,
                preferible_event_type,
                board,
            )?),
            59 => Ok(check_event_effectivity_for_card_59(
                active_card,
                preview_card,
                player_types,
                faction,
                preferible_event_type,
                board,
            )?),
            60 => Ok(check_event_effectivity_for_card_60(
                active_card,
                preview_card,
                player_types,
                faction,
                preferible_event_type,
                board,
            )?),
            62 => Ok(check_event_effectivity_for_card_62(
                active_card,
                preview_card,
                player_types,
                faction,
                preferible_event_type,
                board,
            )?),
            63 => Ok(check_event_effectivity_for_card_63(
                active_card,
                preview_card,
                player_types,
                faction,
                preferible_event_type,
                board,
            )?),
            64 => Ok(check_event_effectivity_for_card_64(
                active_card,
                preview_card,
                player_types,
                faction,
                preferible_event_type,
                board,
            )?),
            65 => Ok(check_event_effectivity_for_card_65(
                active_card,
                preview_card,
                player_types,
                faction,
                preferible_event_type,
                board,
            )?),
            67 => Ok(check_event_effectivity_for_card_67(
                active_card,
                preview_card,
                player_types,
                faction,
                preferible_event_type,
                board,
            )?),
            68 => Ok(check_event_effectivity_for_card_68(
                active_card,
                preview_card,
                player_types,
                faction,
                preferible_event_type,
                board,
            )?),
            69 => Ok(check_event_effectivity_for_card_69(
                active_card,
                preview_card,
                player_types,
                faction,
                preferible_event_type,
                board,
            )?),
            70 => Ok(check_event_effectivity_for_card_70(
                active_card,
                preview_card,
                player_types,
                faction,
                preferible_event_type,
                board,
            )?),
            71 => Ok(check_event_effectivity_for_card_71(
                active_card,
                preview_card,
                player_types,
                faction,
                preferible_event_type,
                board,
            )?),
            72 => Ok(check_event_effectivity_for_card_72(
                active_card,
                preview_card,
                player_types,
                faction,
                preferible_event_type,
                board,
            )?),
            73 => Ok(check_event_effectivity_for_card_73(
                active_card,
                preview_card,
                player_types,
                faction,
                preferible_event_type,
                board,
            )?),
            74 => Ok(check_event_effectivity_for_card_74(
                active_card,
                preview_card,
                player_types,
                faction,
                preferible_event_type,
                board,
            )?),
            75 => Ok(check_event_effectivity_for_card_75(
                active_card,
                preview_card,
                player_types,
                faction,
                preferible_event_type,
                board,
            )?),
            76 => Ok(check_event_effectivity_for_card_76(
                active_card,
                preview_card,
                player_types,
                faction,
                preferible_event_type,
                board,
            )?),
            77 => Ok(check_event_effectivity_for_card_77(
                active_card,
                preview_card,
                player_types,
                faction,
                preferible_event_type,
                board,
            )?),
            78 => Ok(check_event_effectivity_for_card_78(
                active_card,
                preview_card,
                player_types,
                faction,
                preferible_event_type,
                board,
            )?),
            79 => Ok(check_event_effectivity_for_card_79(
                active_card,
                preview_card,
                player_types,
                faction,
                preferible_event_type,
                board,
            )?),
            80 => Ok(check_event_effectivity_for_card_80(
                active_card,
                preview_card,
                player_types,
                faction,
                preferible_event_type,
                board,
            )?),
            81 => Ok(check_event_effectivity_for_card_81(
                active_card,
                preview_card,
                player_types,
                faction,
                preferible_event_type,
                board,
            )?),
            82 => Ok(check_event_effectivity_for_card_82(
                active_card,
                preview_card,
                player_types,
                faction,
                preferible_event_type,
                board,
            )?),
            84 => Ok(check_event_effectivity_for_card_84(
                active_card,
                preview_card,
                player_types,
                faction,
                preferible_event_type,
                board,
            )?),
            85 => Ok(check_event_effectivity_for_card_85(
                active_card,
                preview_card,
                player_types,
                faction,
                preferible_event_type,
                board,
            )?),
            87 => Ok(check_event_effectivity_for_card_87(
                active_card,
                preview_card,
                player_types,
                faction,
                preferible_event_type,
                board,
            )?),
            88 => Ok(check_event_effectivity_for_card_88(
                active_card,
                preview_card,
                player_types,
                faction,
                preferible_event_type,
                board,
            )?),
            89 => Ok(check_event_effectivity_for_card_89(
                active_card,
                preview_card,
                player_types,
                faction,
                preferible_event_type,
                board,
            )?),
            90 => Ok(check_event_effectivity_for_card_90(
                active_card,
                preview_card,
                player_types,
                faction,
                preferible_event_type,
                board,
            )?),
            93 => Ok(check_event_effectivity_for_card_93(
                active_card,
                preview_card,
                player_types,
                faction,
                preferible_event_type,
                board,
            )?),
            95 => Ok(check_event_effectivity_for_card_95(
                active_card,
                preview_card,
                player_types,
                faction,
                preferible_event_type,
                board,
            )?),
            96 => Ok(check_event_effectivity_for_card_96(
                active_card,
                preview_card,
                player_types,
                faction,
                preferible_event_type,
                board,
            )?),
            97 => Ok(check_event_effectivity_for_card_97(
                active_card,
                preview_card,
                player_types,
                faction,
                preferible_event_type,
                board,
            )?),
            98 => Ok(check_event_effectivity_for_card_98(
                active_card,
                preview_card,
                player_types,
                faction,
                preferible_event_type,
                board,
            )?),
            99 => Ok(check_event_effectivity_for_card_99(
                active_card,
                preview_card,
                player_types,
                faction,
                preferible_event_type,
                board,
            )?),
            100 => Ok(check_event_effectivity_for_card_100(
                active_card,
                preview_card,
                player_types,
                faction,
                preferible_event_type,
                board,
            )?),
            102 => Ok(check_event_effectivity_for_card_102(
                active_card,
                preview_card,
                player_types,
                faction,
                preferible_event_type,
                board,
            )?),
            103 => Ok(check_event_effectivity_for_card_103(
                active_card,
                preview_card,
                player_types,
                faction,
                preferible_event_type,
                board,
                sequence_of_play_controller,
            )?),
            105 => Ok(check_event_effectivity_for_card_105(
                active_card,
                preview_card,
                player_types,
                faction,
                preferible_event_type,
                board,
            )?),
            106 => Ok(check_event_effectivity_for_card_106(
                active_card,
                preview_card,
                player_types,
                faction,
                preferible_event_type,
                board,
            )?),
            107 => Ok(check_event_effectivity_for_card_107(
                active_card,
                preview_card,
                player_types,
                faction,
                preferible_event_type,
                board,
            )?),
            108 => Ok(check_event_effectivity_for_card_108(
                active_card,
                preview_card,
                player_types,
                faction,
                preferible_event_type,
                board,
            )?),
            110 => Ok(check_event_effectivity_for_card_110(
                active_card,
                preview_card,
                player_types,
                faction,
                preferible_event_type,
                board,
                flags_controller,
            )?),
            111 => Ok(check_event_effectivity_for_card_111(
                active_card,
                preview_card,
                player_types,
                faction,
                preferible_event_type,
                board,
            )?),
            112 => Ok(check_event_effectivity_for_card_112(
                active_card,
                preview_card,
                player_types,
                faction,
                preferible_event_type,
                board,
            )?),
            113 => Ok(check_event_effectivity_for_card_113(
                active_card,
                preview_card,
                player_types,
                faction,
                preferible_event_type,
                board,
            )?),
            114 => Ok(check_event_effectivity_for_card_114(
                active_card,
                preview_card,
                player_types,
                faction,
                preferible_event_type,
                board,
            )?),
            115 => Ok(check_event_effectivity_for_card_115(
                active_card,
                preview_card,
                player_types,
                faction,
                preferible_event_type,
                board,
            )?),
            116 => Ok(check_event_effectivity_for_card_116(
                active_card,
                preview_card,
                player_types,
                faction,
                preferible_event_type,
                board,
            )?),
            117 => Ok(check_event_effectivity_for_card_117(
                active_card,
                preview_card,
                player_types,
                faction,
                preferible_event_type,
                board,
            )?),
            118 => Ok(check_event_effectivity_for_card_118(
                active_card,
                preview_card,
                player_types,
                faction,
                preferible_event_type,
                board,
            )?),
            119 => Ok(check_event_effectivity_for_card_119(
                active_card,
                preview_card,
                player_types,
                faction,
                preferible_event_type,
                board,
            )?),
            120 => Ok(check_event_effectivity_for_card_120(
                active_card,
                preview_card,
                player_types,
                faction,
                preferible_event_type,
                board,
            )?),
            _ => panic!(
                "Checking event effectivity not implemented for event {:?}",
                active_card.get_number()?
            ),
        }
    }
}
