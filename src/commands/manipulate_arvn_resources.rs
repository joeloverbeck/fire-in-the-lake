use board::track::Track;
use math::sum_i8_to_u8::sum_i8_to_u8;

pub fn manipulate_arvn_resources(
    track: &mut Track,
    amount: i8,
) -> std::result::Result<(), std::string::String> {
    track.set_arvn_resources(sum_i8_to_u8(amount, track.get_arvn_resources()));
    Ok(())
}
