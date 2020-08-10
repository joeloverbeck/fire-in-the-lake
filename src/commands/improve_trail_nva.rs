use board::track::Track;
use commands::manipulate_nva_resources::manipulate_nva_resources;

pub fn improve_trail_nva(track: &mut Track) -> Result<(), String> {
    manipulate_nva_resources(track, -2)?;

    track.set_trail(track.get_trail() + 1);

    Ok(())
}
