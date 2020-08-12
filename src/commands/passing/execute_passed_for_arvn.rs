use board::track::Track;
use commands::atomic::manipulate_nva_resources::manipulate_nva_resources;

pub fn execute_passed_for_nva(track: &mut Track) -> Result<(), String> {
    // NVA passing increases their resources +1.
    manipulate_nva_resources(track, 1)?;

    Ok(())
}
