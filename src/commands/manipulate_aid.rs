use super::super::track::Track;
use super::command::Command;

struct ManipulateAid<'a> {
    track: &'a mut Track,
    amount: i8,
}

impl ManipulateAid<'_> {
    fn new(track: &mut Track, amount: i8) -> ManipulateAid {
        ManipulateAid { track, amount }
    }
}

fn sum_i8_to_u8(signed: i8, mut unsigned: u8) -> u8 {
    if signed < 0 {
        unsigned -= i8::abs(signed) as u8;

        return unsigned;
    }

    unsigned += i8::abs(signed) as u8;

    unsigned
}

impl Command for ManipulateAid<'_> {
    fn execute(&mut self) -> std::result::Result<(), std::string::String> {
        self.track
            .set_aid(sum_i8_to_u8(self.amount, self.track.aid()));

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_should_be_able_to_create_manipulate_aid() {
        let mut track = Track::new();

        let _ = ManipulateAid::new(&mut track, -12);
    }

    #[test]
    fn test_when_executing_manipulate_aid_that_manipulates_it_by_minus_12_from_15_it_should_result_in_3_aid(
    ) {
        let mut track = Track::new();
        track.set_aid(15);

        if let Err(error) = ManipulateAid::new(&mut track, -12).execute() {
            panic!(error);
        }

        assert_eq!(track.aid(), 3);
    }
}
