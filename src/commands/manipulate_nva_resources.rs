use board::track::Track;
use math::sum_i8_to_u8::sum_i8_to_u8;

#[derive(Debug)]
pub struct ManipulateNvaResources {}

impl Default for ManipulateNvaResources {
    fn default() -> Self {
        Self::new()
    }
}

impl ManipulateNvaResources {
    pub fn new() -> ManipulateNvaResources {
        ManipulateNvaResources {}
    }

    pub fn execute(
        &self,
        track: &mut Track,
        amount: i8,
    ) -> std::result::Result<(), std::string::String> {
        track.set_nva_resources(sum_i8_to_u8(amount, track.get_nva_resources()));

        Ok(())
    }
}
