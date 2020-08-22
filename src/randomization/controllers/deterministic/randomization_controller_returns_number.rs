use randomization::controllers::randomization_controller_trait::RandomizationControllerTrait;

#[derive(Debug)]
pub struct RandomizationControllerReturnsNumber {
    number_to_return: u8,
}

impl RandomizationControllerTrait for RandomizationControllerReturnsNumber {
    fn roll_six_sided_die(&self) -> Result<u8, String> {
        Ok(self.number_to_return)
    }
}

impl RandomizationControllerReturnsNumber {
    pub fn new(number_to_return: u8) -> RandomizationControllerReturnsNumber {
        RandomizationControllerReturnsNumber { number_to_return }
    }
}
