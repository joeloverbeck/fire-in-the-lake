extern crate lib_dice;

use randomization::controllers::randomization_controller_trait::RandomizationControllerTrait;

#[derive(Debug)]
pub struct RandomizationController {}

impl Default for RandomizationController {
    fn default() -> Self {
        Self::new()
    }
}

impl RandomizationControllerTrait for RandomizationController {
    fn roll_six_sided_die(&self) -> Result<u8, String> {
        Ok(lib_dice::roll(1, 6, 0) as u8)
    }
}

impl RandomizationController {
    pub fn new() -> RandomizationController {
        RandomizationController {}
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    pub fn test_throwing_the_die_twenty_times_doesnt_produce_any_result_outside_its_boundaries(
    ) -> Result<(), String> {
        let randomization_controller = RandomizationController::new();

        for _ in 1..20 {
            let throw = randomization_controller.roll_six_sided_die()?;
            assert!(throw > 0 && throw < 7);
        }

        Ok(())
    }
}
