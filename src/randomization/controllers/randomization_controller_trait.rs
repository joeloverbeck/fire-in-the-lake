extern crate enum_dispatch;
use self::enum_dispatch::enum_dispatch;
use randomization::controllers::deterministic::randomization_controller_returns_number::RandomizationControllerReturnsNumber;
use randomization::controllers::randomization_controller::RandomizationController;

#[enum_dispatch]
pub trait RandomizationControllerTrait {
    fn roll_six_sided_die(&self) -> Result<u8, String>;
}

#[enum_dispatch(RandomizationControllerTrait)]
#[derive(Debug)]
pub enum RandomizationControllers {
    RandomizationController,
    RandomizationControllerReturnsNumber,
}
