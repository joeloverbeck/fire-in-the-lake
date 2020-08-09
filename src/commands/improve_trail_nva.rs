use board::track::Track;
use commands::manipulate_nva_resources::ManipulateNvaResources;

pub struct ImproveTrailNva {}

impl Default for ImproveTrailNva {
    fn default() -> Self {
        Self::new()
    }
}

impl ImproveTrailNva {
    pub fn new() -> ImproveTrailNva {
        ImproveTrailNva {}
    }

    pub fn execute(&self, track: &mut Track) -> Result<(), String> {
        ManipulateNvaResources::new().execute(track, -2)?;

        track.set_trail(track.get_trail() + 1);

        Ok(())
    }
}
