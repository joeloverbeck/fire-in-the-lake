use board::domain::board::Board;
use board::domain::queries::board_level_queries::get_spaces_where_nva_can_place_base_through_rally::get_spaces_where_nva_can_place_base_through_rally;

pub fn can_nva_place_base_through_rally(board: &Board) -> Result<bool, String> {
    // For that there must be at least 4 NVA or VC Guerrillas or Troops (including 2 NVA Guerrillas and room for the Base)
    Ok(!get_spaces_where_nva_can_place_base_through_rally(board)?.is_empty())
}
