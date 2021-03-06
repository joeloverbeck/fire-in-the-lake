use game_definitions::event_types::EventTypes;
use game_definitions::flags::Flags;
use game_definitions::nva_capabilities::NvaCapabilities;
use game_definitions::us_capabilities::UsCapabilities;
use std::collections::HashMap;

pub struct FlagsController {
    flags: Vec<Flags>,
    us_capabilities: HashMap<UsCapabilities, EventTypes>,
    nva_capabilities: HashMap<NvaCapabilities, EventTypes>,
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
            nva_capabilities: HashMap::new(),
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

    pub fn has_nva_capability(&self, nva_capability: NvaCapabilities) -> Result<bool, String> {
        if self.nva_capabilities.is_empty() {
            return Ok(false);
        }

        Ok(self.nva_capabilities.contains_key(&nva_capability))
    }

    pub fn get_event_type_of_nva_capability(
        &self,
        nva_capability: NvaCapabilities,
    ) -> Result<&EventTypes, String> {
        if self.nva_capabilities.is_empty() {
            panic!("Attempted to get the event type of nva capability {:?}, but there were no NVA Capabilities.", nva_capability);
        }

        if !self.nva_capabilities.contains_key(&nva_capability) {
            panic!("Attempted to get the event type of nva capability {:?}, but there was no entry for that capability.", nva_capability);
        }

        Ok(self.nva_capabilities.get(&nva_capability).as_ref().unwrap())
    }

    fn remove_from_flags(&mut self, flag_to_remove: Flags) -> Result<(), String> {
        for i in (0..self.flags.len()).rev() {
            if self.flags[i] == flag_to_remove {
                self.flags.swap_remove(i);
            }
        }

        Ok(())
    }

    pub fn perform_end_of_turn(&mut self) -> Result<(), String> {
        // Wipe some of the flags that should be present for a single turn
        if self
            .flags
            .iter()
            .any(|flag| flag == &Flags::NvaAttemptedToMarchButWereUnable)
        {
            self.remove_from_flags(Flags::NvaAttemptedToMarchButWereUnable)?;
        }
        if self
            .flags
            .iter()
            .any(|flag| flag == &Flags::NvaAttemptedToRallyButWereUnable)
        {
            self.remove_from_flags(Flags::NvaAttemptedToRallyButWereUnable)?;
        }

        Ok(())
    }
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]
    fn test_after_performing_end_of_turn_those_flags_that_are_single_turn_only_should_have_been_wiped(
    ) -> Result<(), String> {
        let mut sut = FlagsController::new();

        sut.set_flag(Flags::NvaAttemptedToMarchButWereUnable)?;
        sut.set_flag(Flags::NvaAttemptedToRallyButWereUnable)?;

        sut.perform_end_of_turn()?;

        assert_eq!(
            sut.has_flag(Flags::NvaAttemptedToMarchButWereUnable)?,
            false
        );
        assert_eq!(
            sut.has_flag(Flags::NvaAttemptedToRallyButWereUnable)?,
            false
        );

        Ok(())
    }
}
