
use super::command::Command;
use board::region::Region;
use board::support::Support;
use board::regions::Regions;

struct ShiftSupportOfRegion<'a> {
    region: &'a mut Region,
    levels_to_shift: i8
}

impl ShiftSupportOfRegion<'_> {
    fn new(region: &mut Region, levels_to_shift: i8) -> ShiftSupportOfRegion {
        ShiftSupportOfRegion {
            region: region,
            levels_to_shift: levels_to_shift
        }
    }
}

impl Command for ShiftSupportOfRegion<'_> {

    fn execute(&mut self) -> std::result::Result<(), std::string::String> { 
        // Shifting the support of a region has a particular logic that should be encapsulated
        // Passive Support -> Neutral

        if self.region.support() == Support::PassiveSupport && self.levels_to_shift == -1 {
            self.region.set_support(Support::Neutral);
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn should_be_able_to_create_shift_support_of_region_command() {
        let mut region = Region::new(Regions::Saigon);

        let _ = ShiftSupportOfRegion::new(&mut region, -1);
    }

    #[test]
    fn on_executing_the_shift_of_support_of_a_region_by_level_specified_the_level_should_have_dropped_in_the_region(){
        let mut region = Region::new(Regions::Saigon);

        region.set_support(Support::PassiveSupport);

        let mut command = ShiftSupportOfRegion::new(&mut region, -1);

        if let Err(error) = command.execute() {
            panic!(error);
        }        

        assert_eq!(region.support(), Support::Neutral, "The region's support after shifing down a level should have been neutral.");
    }

}