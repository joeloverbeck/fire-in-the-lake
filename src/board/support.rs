#[derive(Debug, Copy, Clone, PartialEq)]
pub enum SupportLevels {
    ActiveSupport,
    PassiveSupport,
    Neutral,
    PassiveOpposition,
    ActiveOpposition,
}

#[derive(Debug)]
pub struct Support {
    current_support_level: SupportLevels,
}

impl Default for Support {
    fn default() -> Self {
        Self::new()
    }
}

impl Support {
    pub fn new() -> Support {
        Support {
            current_support_level: SupportLevels::Neutral,
        }
    }

    pub fn get_current_support_level(&self) -> SupportLevels {
        self.current_support_level
    }

    pub fn set_support_level(&mut self, new_support_level: SupportLevels) {
        self.current_support_level = new_support_level;
    }

    pub fn shift_support_level_down(&mut self) {
        // It will try to shift the support level to the next level down. If it's already at the lowest level, it will stay there.
        match self.current_support_level {
            SupportLevels::PassiveSupport => self.current_support_level = SupportLevels::Neutral,
            SupportLevels::Neutral => self.current_support_level = SupportLevels::PassiveOpposition,
            _ => todo!(),
        }
    }

    pub fn shift_support_level_up(&mut self) {
        match self.current_support_level {
            SupportLevels::PassiveSupport => {
                self.current_support_level = SupportLevels::ActiveSupport
            }
            SupportLevels::Neutral => self.current_support_level = SupportLevels::PassiveSupport,
            _ => todo!(),
        }
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_should_be_able_to_create_support_object() -> Result<(), String> {
        let _ = Support::new();

        Ok(())
    }
}
