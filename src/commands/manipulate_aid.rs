use board::track::Track;
use math::sum_i8_to_u8::sum_i8_to_u8;

#[derive(Debug)]
pub struct ManipulateAid {}

impl Default for ManipulateAid {
    fn default() -> Self {
        Self::new()
    }
}

impl ManipulateAid {
    pub fn new() -> ManipulateAid {
        ManipulateAid {}
    }

    pub fn execute(
        &self,
        track: &mut Track,
        amount: i8,
    ) -> std::result::Result<(), std::string::String> {
        track.set_aid(sum_i8_to_u8(amount, track.get_aid()));

        Ok(())
    }
}
