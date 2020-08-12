use board::track::Track;
use math::sum_i8_to_u8::sum_i8_to_u8;

pub fn manipulate_aid(
    track: &mut Track,
    amount: i8,
) -> std::result::Result<(), std::string::String> {
    track.set_aid(sum_i8_to_u8(amount, track.get_aid()));

    Ok(())
}
