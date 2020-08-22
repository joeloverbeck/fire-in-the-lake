use board::domain::space::Space;
use board::domain::space::Spaces;
use game_definitions::support_levels::SupportLevels;

pub fn does_space_have_support(space: &Spaces) -> Result<bool, String> {
    Ok(space.get_support_level()? == &SupportLevels::PassiveSupport
        || space.get_support_level()? == &SupportLevels::ActiveSupport)
}
