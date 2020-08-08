use board::track::Track;

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
        track.set_aid(sum_i8_to_u8(amount, track.aid()));

        Ok(())
    }
}

fn sum_i8_to_u8(signed: i8, mut unsigned: u8) -> u8 {
    if signed < 0 {
        if (unsigned as i8 + signed) < 0 {
            // It would create an attempt to substract with overflow. Just return 0.
            return 0;
        }

        unsigned -= i8::abs(signed) as u8;

        return unsigned;
    }

    unsigned += i8::abs(signed) as u8;

    unsigned
}
