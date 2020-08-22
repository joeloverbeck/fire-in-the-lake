use board::domain::board::Board;
use cards::domain::card::Card;
use cards::domain::card::Cards;
use flags::controllers::flags_controller::FlagsController;
use game_definitions::event_types::EventTypes;
use game_definitions::factions::Factions;
use players::domain::events::effectivity::check_event_effectivity_for_card_1::check_event_effectivity_for_card_1;
use players::domain::events::effectivity::check_event_effectivity_for_card_100::check_event_effectivity_for_card_100;
use players::domain::events::effectivity::check_event_effectivity_for_card_103::check_event_effectivity_for_card_103;
use players::domain::events::effectivity::check_event_effectivity_for_card_106::check_event_effectivity_for_card_106;
use players::domain::events::effectivity::check_event_effectivity_for_card_107::check_event_effectivity_for_card_107;
use players::domain::events::effectivity::check_event_effectivity_for_card_108::check_event_effectivity_for_card_108;
use players::domain::events::effectivity::check_event_effectivity_for_card_111::check_event_effectivity_for_card_111;
use players::domain::events::effectivity::check_event_effectivity_for_card_112::check_event_effectivity_for_card_112;
use players::domain::events::effectivity::check_event_effectivity_for_card_113::check_event_effectivity_for_card_113;
use players::domain::events::effectivity::check_event_effectivity_for_card_114::check_event_effectivity_for_card_114;
use players::domain::events::effectivity::check_event_effectivity_for_card_115::check_event_effectivity_for_card_115;
use players::domain::events::effectivity::check_event_effectivity_for_card_116::check_event_effectivity_for_card_116;
use players::domain::events::effectivity::check_event_effectivity_for_card_118::check_event_effectivity_for_card_118;
use players::domain::events::effectivity::check_event_effectivity_for_card_119::check_event_effectivity_for_card_119;
use players::domain::events::effectivity::check_event_effectivity_for_card_12::check_event_effectivity_for_card_12;
use players::domain::events::effectivity::check_event_effectivity_for_card_16::check_event_effectivity_for_card_16;
use players::domain::events::effectivity::check_event_effectivity_for_card_17::check_event_effectivity_for_card_17;
use players::domain::events::effectivity::check_event_effectivity_for_card_2::check_event_effectivity_for_card_2;
use players::domain::events::effectivity::check_event_effectivity_for_card_21::check_event_effectivity_for_card_21;
use players::domain::events::effectivity::check_event_effectivity_for_card_22::check_event_effectivity_for_card_22;
use players::domain::events::effectivity::check_event_effectivity_for_card_23::check_event_effectivity_for_card_23;
use players::domain::events::effectivity::check_event_effectivity_for_card_24::check_event_effectivity_for_card_24;
use players::domain::events::effectivity::check_event_effectivity_for_card_25::check_event_effectivity_for_card_25;
use players::domain::events::effectivity::check_event_effectivity_for_card_26::check_event_effectivity_for_card_26;
use players::domain::events::effectivity::check_event_effectivity_for_card_27::check_event_effectivity_for_card_27;
use players::domain::events::effectivity::check_event_effectivity_for_card_3::check_event_effectivity_for_card_3;
use players::domain::events::effectivity::check_event_effectivity_for_card_30::check_event_effectivity_for_card_30;
use players::domain::events::effectivity::check_event_effectivity_for_card_36::check_event_effectivity_for_card_36;
use players::domain::events::effectivity::check_event_effectivity_for_card_38::check_event_effectivity_for_card_38;
use players::domain::events::effectivity::check_event_effectivity_for_card_39::check_event_effectivity_for_card_39;
use players::domain::events::effectivity::check_event_effectivity_for_card_40::check_event_effectivity_for_card_40;
use players::domain::events::effectivity::check_event_effectivity_for_card_41::check_event_effectivity_for_card_41;
use players::domain::events::effectivity::check_event_effectivity_for_card_42::check_event_effectivity_for_card_42;
use players::domain::events::effectivity::check_event_effectivity_for_card_46::check_event_effectivity_for_card_46;
use players::domain::events::effectivity::check_event_effectivity_for_card_48::check_event_effectivity_for_card_48;
use players::domain::events::effectivity::check_event_effectivity_for_card_50::check_event_effectivity_for_card_50;
use players::domain::events::effectivity::check_event_effectivity_for_card_51::check_event_effectivity_for_card_51;
use players::domain::events::effectivity::check_event_effectivity_for_card_52::check_event_effectivity_for_card_52;
use players::domain::events::effectivity::check_event_effectivity_for_card_53::check_event_effectivity_for_card_53;
use players::domain::events::effectivity::check_event_effectivity_for_card_54::check_event_effectivity_for_card_54;
use players::domain::events::effectivity::check_event_effectivity_for_card_55::check_event_effectivity_for_card_55;
use players::domain::events::effectivity::check_event_effectivity_for_card_59::check_event_effectivity_for_card_59;
use players::domain::events::effectivity::check_event_effectivity_for_card_6::check_event_effectivity_for_card_6;
use players::domain::events::effectivity::check_event_effectivity_for_card_60::check_event_effectivity_for_card_60;
use players::domain::events::effectivity::check_event_effectivity_for_card_62::check_event_effectivity_for_card_62;
use players::domain::events::effectivity::check_event_effectivity_for_card_63::check_event_effectivity_for_card_63;
use players::domain::events::effectivity::check_event_effectivity_for_card_64::check_event_effectivity_for_card_64;
use players::domain::events::effectivity::check_event_effectivity_for_card_65::check_event_effectivity_for_card_65;
use players::domain::events::effectivity::check_event_effectivity_for_card_67::check_event_effectivity_for_card_67;
use players::domain::events::effectivity::check_event_effectivity_for_card_68::check_event_effectivity_for_card_68;
use players::domain::events::effectivity::check_event_effectivity_for_card_69::check_event_effectivity_for_card_69;
use players::domain::events::effectivity::check_event_effectivity_for_card_7::check_event_effectivity_for_card_7;
use players::domain::events::effectivity::check_event_effectivity_for_card_70::check_event_effectivity_for_card_70;
use players::domain::events::effectivity::check_event_effectivity_for_card_72::check_event_effectivity_for_card_72;
use players::domain::events::effectivity::check_event_effectivity_for_card_74::check_event_effectivity_for_card_74;
use players::domain::events::effectivity::check_event_effectivity_for_card_75::check_event_effectivity_for_card_75;
use players::domain::events::effectivity::check_event_effectivity_for_card_76::check_event_effectivity_for_card_76;
use players::domain::events::effectivity::check_event_effectivity_for_card_77::check_event_effectivity_for_card_77;
use players::domain::events::effectivity::check_event_effectivity_for_card_78::check_event_effectivity_for_card_78;
use players::domain::events::effectivity::check_event_effectivity_for_card_79::check_event_effectivity_for_card_79;
use players::domain::events::effectivity::check_event_effectivity_for_card_80::check_event_effectivity_for_card_80;
use players::domain::events::effectivity::check_event_effectivity_for_card_81::check_event_effectivity_for_card_81;
use players::domain::events::effectivity::check_event_effectivity_for_card_82::check_event_effectivity_for_card_82;
use players::domain::events::effectivity::check_event_effectivity_for_card_84::check_event_effectivity_for_card_84;
use players::domain::events::effectivity::check_event_effectivity_for_card_87::check_event_effectivity_for_card_87;
use players::domain::events::effectivity::check_event_effectivity_for_card_88::check_event_effectivity_for_card_88;
use players::domain::events::effectivity::check_event_effectivity_for_card_90::check_event_effectivity_for_card_90;
use players::domain::events::effectivity::check_event_effectivity_for_card_93::check_event_effectivity_for_card_93;
use players::domain::events::effectivity::check_event_effectivity_for_card_95::check_event_effectivity_for_card_95;
use players::domain::events::effectivity::check_event_effectivity_for_card_96::check_event_effectivity_for_card_96;
use players::domain::events::effectivity::check_event_effectivity_for_card_97::check_event_effectivity_for_card_97;
use players::domain::events::effectivity::check_event_effectivity_for_card_98::check_event_effectivity_for_card_98;
use players::domain::events::effectivity::check_event_effectivity_for_card_99::check_event_effectivity_for_card_99;
use players::domain::player_type::PlayerType;
use sequence_of_play::controllers::sequence_of_play_controller::SequenceOfPlayController;
use std::collections::HashMap;

#[allow(clippy::too_many_arguments)]
pub fn is_current_non_capability_event_effective(
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
        12 => Ok(check_event_effectivity_for_card_12(
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
        30 => Ok(check_event_effectivity_for_card_30(
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
        46 => Ok(check_event_effectivity_for_card_46(
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
        65 => Ok(check_event_effectivity_for_card_65(
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
        70 => Ok(check_event_effectivity_for_card_70(
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
        64 => Ok(check_event_effectivity_for_card_64(
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
        69 => Ok(check_event_effectivity_for_card_69(
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
        74 => Ok(check_event_effectivity_for_card_74(
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
        103 => Ok(check_event_effectivity_for_card_103(
            active_card,
            preview_card,
            player_types,
            faction,
            preferible_event_type,
            board,
            sequence_of_play_controller,
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
        100 => Ok(check_event_effectivity_for_card_100(
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
        _ => panic!(
            "Checking event effectivity not implemented for event {:?}",
            active_card.get_number()?
        ),
    }
}
