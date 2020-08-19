use board::domain::board::Board;
use cards::domain::card::Card;
use cards::domain::card::Cards;
use game_definitions::event_type::EventType;
use game_definitions::factions::Factions;
use players::domain::events::effectivity::check_event_effectivity_for_card_1::check_event_effectivity_for_card_1;
use players::domain::events::effectivity::check_event_effectivity_for_card_100::check_event_effectivity_for_card_100;
use players::domain::events::effectivity::check_event_effectivity_for_card_106::check_event_effectivity_for_card_106;
use players::domain::events::effectivity::check_event_effectivity_for_card_111::check_event_effectivity_for_card_111;
use players::domain::events::effectivity::check_event_effectivity_for_card_112::check_event_effectivity_for_card_112;
use players::domain::events::effectivity::check_event_effectivity_for_card_113::check_event_effectivity_for_card_113;
use players::domain::events::effectivity::check_event_effectivity_for_card_114::check_event_effectivity_for_card_114;
use players::domain::events::effectivity::check_event_effectivity_for_card_115::check_event_effectivity_for_card_115;
use players::domain::events::effectivity::check_event_effectivity_for_card_116::check_event_effectivity_for_card_116;
use players::domain::events::effectivity::check_event_effectivity_for_card_118::check_event_effectivity_for_card_118;
use players::domain::events::effectivity::check_event_effectivity_for_card_119::check_event_effectivity_for_card_119;
use players::domain::events::effectivity::check_event_effectivity_for_card_16::check_event_effectivity_for_card_16;
use players::domain::events::effectivity::check_event_effectivity_for_card_17::check_event_effectivity_for_card_17;
use players::domain::events::effectivity::check_event_effectivity_for_card_2::check_event_effectivity_for_card_2;
use players::domain::events::effectivity::check_event_effectivity_for_card_21::check_event_effectivity_for_card_21;
use players::domain::events::effectivity::check_event_effectivity_for_card_24::check_event_effectivity_for_card_24;
use players::domain::events::effectivity::check_event_effectivity_for_card_25::check_event_effectivity_for_card_25;
use players::domain::events::effectivity::check_event_effectivity_for_card_27::check_event_effectivity_for_card_27;
use players::domain::events::effectivity::check_event_effectivity_for_card_30::check_event_effectivity_for_card_30;
use players::domain::events::effectivity::check_event_effectivity_for_card_38::check_event_effectivity_for_card_38;
use players::domain::events::effectivity::check_event_effectivity_for_card_41::check_event_effectivity_for_card_41;
use players::domain::events::effectivity::check_event_effectivity_for_card_46::check_event_effectivity_for_card_46;
use players::domain::events::effectivity::check_event_effectivity_for_card_48::check_event_effectivity_for_card_48;
use players::domain::events::effectivity::check_event_effectivity_for_card_63::check_event_effectivity_for_card_63;
use players::domain::events::effectivity::check_event_effectivity_for_card_67::check_event_effectivity_for_card_67;
use players::domain::events::effectivity::check_event_effectivity_for_card_69::check_event_effectivity_for_card_69;
use players::domain::events::effectivity::check_event_effectivity_for_card_70::check_event_effectivity_for_card_70;
use players::domain::events::effectivity::check_event_effectivity_for_card_76::check_event_effectivity_for_card_76;
use players::domain::events::effectivity::check_event_effectivity_for_card_78::check_event_effectivity_for_card_78;
use players::domain::events::effectivity::check_event_effectivity_for_card_79::check_event_effectivity_for_card_79;
use players::domain::events::effectivity::check_event_effectivity_for_card_80::check_event_effectivity_for_card_80;
use players::domain::events::effectivity::check_event_effectivity_for_card_81::check_event_effectivity_for_card_81;
use players::domain::events::effectivity::check_event_effectivity_for_card_93::check_event_effectivity_for_card_93;
use players::domain::events::effectivity::check_event_effectivity_for_card_95::check_event_effectivity_for_card_95;
use players::domain::events::effectivity::check_event_effectivity_for_card_97::check_event_effectivity_for_card_97;
use players::domain::events::effectivity::check_event_effectivity_for_card_98::check_event_effectivity_for_card_98;
use players::domain::player_type::PlayerType;
use std::collections::HashMap;

pub fn is_current_non_capability_event_effective(
    active_card: &Cards,
    preview_card: &Cards,
    player_types: HashMap<Factions, PlayerType>,
    faction: &Factions,
    preferible_event_type: EventType,
    board: &Board,
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
        38 => Ok(check_event_effectivity_for_card_38(
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
        63 => Ok(check_event_effectivity_for_card_63(
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
        76 => Ok(check_event_effectivity_for_card_76(
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
        106 => Ok(check_event_effectivity_for_card_106(
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
