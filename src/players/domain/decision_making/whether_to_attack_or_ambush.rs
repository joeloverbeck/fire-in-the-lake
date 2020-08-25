use board::domain::board::Board;
use board::domain::queries::board_level_queries::can_attack_remove_a_number_of_enemies::can_attack_remove_a_number_of_enemies;
use board::domain::queries::board_level_queries::can_attack_remove_base::can_attack_remove_base;
use game_definitions::factions::Factions;
use players::domain::decision::Decision;
use randomization::controllers::randomization_controller_trait::RandomizationControllerTrait;
use randomization::controllers::randomization_controller_trait::RandomizationControllers;

pub fn whether_to_attack_or_ambush(
    board: &Board,
    randomization_controller: &RandomizationControllers,
) -> Result<Option<Decision>, String> {
    if can_attack_remove_base(&Factions::NVA, board)?
        || can_attack_remove_a_number_of_enemies(
            &Factions::NVA,
            randomization_controller.roll_six_sided_die()?,
            board,
        )?
    {
        panic!("NVA bot detected it can ambush or attack to remove base or 1d6 enemies.");
    }

    Ok(None)
}
