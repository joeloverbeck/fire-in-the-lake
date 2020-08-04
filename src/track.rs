pub struct Track {
    aid: u8

}

impl Track {
    pub fn new () -> Track {
        Track {
            aid: 0
        }
    }

    pub fn aid(&self) -> u8 {
        self.aid
    }

    pub fn set_aid(&mut self, new_aid: u8) {
        self.aid = new_aid
    }
}