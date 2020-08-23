use game_definitions::event_types::EventTypes;
use game_definitions::flags::Flags;
use game_definitions::us_capabilities::UsCapabilities;
use std::collections::HashMap;

pub struct FlagsController {
    flags: Vec<Flags>,
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
            flags: [Flags::BlowtorchComer].to_vec(),
            us_capabilities: HashMap::new(),
            is_monsoon: false,
        }
    }

    pub fn is_monsoon(&self) -> Result<bool, String> {
        Ok(self.is_monsoon)
    }

    pub fn set_flag(&mut self, flag: Flags) -> Result<(), String> {
        // Sanity check
        if self
            .flags
            .iter()
            .any(|contained_flag| contained_flag == &flag)
        {
            // It was already there. Something is wrong with the calling code,
            // but it wanted to set the flag, and it's already set.
            return Ok(());
        }

        self.flags.push(flag);

        Ok(())
    }

    pub fn has_flag(&self, flag: Flags) -> Result<bool, String> {
        Ok(self
            .flags
            .iter()
            .any(|contained_flag| contained_flag == &flag))
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
