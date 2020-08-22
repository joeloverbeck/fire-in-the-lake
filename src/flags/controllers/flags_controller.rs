use game_definitions::event_types::EventTypes;
use game_definitions::flags::Flags;
use game_definitions::us_capabilities::UsCapabilities;
use std::collections::HashMap;

pub struct FlagsController {
    flags: HashMap<Flags, bool>,
    us_capabilities: HashMap<UsCapabilities, EventTypes>,
    is_monsoon: bool,
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
            us_capabilities: HashMap::new(),
            is_monsoon: false,
        }
    }

    pub fn is_monsoon(&self) -> Result<bool, String> {
        Ok(self.is_monsoon)
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

    pub fn has_unshaded_us_capabilities(&self) -> Result<bool, String> {
        // If there are no us capabilities in the hashmap, then obviously
        // this will return false
        if self.us_capabilities.is_empty() {
            return Ok(false);
        }

        Ok(self
            .us_capabilities
            .iter()
            .any(|(_, event_type)| event_type == &EventTypes::Unshaded))
    }
}
