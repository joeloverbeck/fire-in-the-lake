use board::track::Track;
use math::sum_i8_to_u8::sum_i8_to_u8;

#[derive(Debug)]
pub struct ManipulateArvnResources {}

impl Default for ManipulateArvnResources {
    fn default() -> Self {
        Self::new()
    }
}

impl ManipulateArvnResources {
    pub fn new() -> ManipulateArvnResources {
        ManipulateArvnResources {}
    }

    pub fn execute(
        &self,
        track: &mut Track,
        amount: i8,
    ) -> std::result::Result<(), std::string::String> {
        track.set_arvn_resources(sum_i8_to_u8(amount, track.get_arvn_resources()));
        Ok(())
    }
}
