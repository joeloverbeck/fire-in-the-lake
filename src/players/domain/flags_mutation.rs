use game_definitions::flags::Flags;

#[derive(Debug)]
pub struct FlagsMutation {
    flag: Flags,
    value: bool,
}

impl FlagsMutation {
    pub fn new(flag: Flags, value: bool) -> FlagsMutation {
        FlagsMutation { flag, value }
    }

    pub fn get_flag(&self) -> &Flags {
        &self.flag
    }

    pub fn get_value(&self) -> bool {
        self.value
    }
}
