use game_definitions::factions::Factions;
use players::domain::decision::Decision;

extern crate termcolor;

pub trait DisplayControllerTrait {
    fn write_announcement(&self, text: &str) -> Result<(), String>;
    fn write_information_for_decision(
        &self,
        decision: &Decision,
        faction: &Factions,
    ) -> Result<(), String>;
    fn write_instructions_for_decision(
        &self,
        decision: &Decision,
        faction: &Factions,
    ) -> Result<(), String>;
    fn write_instruction(&self, text: &str) -> Result<(), String>;
    fn write_information(&self, text: &str) -> Result<(), String>;
    fn write_section(&self, text: &str) -> Result<(), String>;
    fn write_alert(&self, text: &str) -> Result<(), String>;
    fn write_everything_necessary_for_decision(
        &self,
        decision: &Decision,
        faction: &Factions,
    ) -> Result<(), String>;
}
