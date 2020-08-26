use players::domain::decision_making::whether_to_bombard::whether_to_bombard;
use board::domain::queries::board_level_queries::can_nva_infiltrate_any_vc_base_anywhere::can_nva_infiltrate_any_vc_base_anywhere;
use board::domain::queries::board_level_queries::would_placing_nva_troops_anywhere_through_infiltrate_place_enough::would_placing_nva_troops_anywhere_through_infiltrate_place_enough;
use flags::controllers::flags_controller::FlagsController;
use board::domain::board::Board;
use players::domain::decision::Decision;
use game_definitions::flags::Flags;

pub fn whether_to_infiltrate(
    possible_previous_decision: Option<&Decision>,
    board: &Board,
    flags_controller: &FlagsController,
) -> Result<Option<Decision>, String> {
    // Infiltrate in up to two spaces (or 1 space if the unshaded "559th Transport Grp" Momentum event is in effect), if
    // and only if an NVA Base or at least 4 Troops would be placed as per follows:

    // -First replace VC bases in spaces with more NVA than VC pieces but no NVA Base already (Shifting Opposition, 4.4.1),
    // first tunneled bases, within that priority first in spaces with the highest population, finally in random spaces.
    // -If there are no NVA Bases available, they may first remove one base from the space in North Vietnam, Laos or cambodia
    // -With the most NVA pieces to Available in order to replace one VC Base.

    // -Then place Troops (at NVA Bases), first in those Base spaces with the most NVA Guerrillas, within that priority first
    // in spaces with the most NVA Guerrillas, within that priority first in spaces with 2 NVA Bases, and within those priorities
    // first in or adjacent to Cities or Provinces with no NVA Control.
    // If placing Troops, then also replace NVA Guerrillas in those spaces with Troops, but only where there are more than 2
    // NVA Guerrillas in a space and until 2 are left.

    // If Infiltrate would place neither a NVA Base nor at least 4 Troops, or if infiltration is impossible, instead Bombard (8.6.1)

    let times_it_can_infiltrate = {
        if flags_controller.has_flag(Flags::F559thTransportGrp)? {
            1
        } else {
            2
        }
    };

    if can_nva_infiltrate_any_vc_base_anywhere(board)?
        || would_placing_nva_troops_anywhere_through_infiltrate_place_enough(
            times_it_can_infiltrate,
            board,
        )?
    {
        panic!("Infiltrating not implemented");
    } else {
        Ok(whether_to_bombard(
            possible_previous_decision,
            board,
            flags_controller,
        )?)
    }
}
