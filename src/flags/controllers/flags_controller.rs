use game_definitions::flags::Flags;
use std::collections::HashMap;

pub struct FlagsController {
    flags: HashMap<Flags, bool>,
}

impl Default for FlagsController {
    fn default() -> Self {
        Self::new()
    }
}

impl FlagsController {
    pub fn new() -> FlagsController {
        FlagsController {
            flags: [(Flags::BlowtorchComer, false)].iter().cloned().collect(),
        }
    }

    fn crash_if_the_flag_wasnt_initialized(&self, flag: Flags) -> Result<(), String> {
        if !self.flags.contains_key(&flag) {
            return Err(format!(
                "Couldn't find the flag {:?} in the collection! Contents: {:?}",
                flag, self.flags
            ));
        }

        Ok(())
    }

    pub fn set_flag(&mut self, flag: Flags, value: bool) -> Result<(), String> {
        self.crash_if_the_flag_wasnt_initialized(flag)?;

        *self.flags.get_mut(&flag).unwrap() = value;

        Ok(())
    }

    pub fn get_flag(&self, flag: Flags) -> Result<bool, String> {
        self.crash_if_the_flag_wasnt_initialized(flag)?;

        Ok(*self.flags.get(&flag).unwrap())
    }
}
