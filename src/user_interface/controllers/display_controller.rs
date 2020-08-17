use game_definitions::factions::Factions;
use players::domain::decision::Decision;
use user_interface::domain::alert_writer::AlertWriter;
use user_interface::domain::announcements_composer::AnnouncementsComposer;
use user_interface::domain::instructions_composer::InstructionsComposer;
use user_interface::domain::produce_instructions_for_mutations::produce_instructions_for_mutations;
use user_interface::domain::section_writer::SectionWriter;

extern crate termcolor;
use self::termcolor::BufferWriter;

pub struct DisplayController {
    buffer_writer: BufferWriter,
    announcements_composer: AnnouncementsComposer,
    section_writer: SectionWriter,
    instructions_composer: InstructionsComposer,
    alert_writer: AlertWriter,
}

impl DisplayController {
    pub fn new(buffer_writer: BufferWriter) -> DisplayController {
        DisplayController {
            buffer_writer,
            announcements_composer: AnnouncementsComposer::new(),
            section_writer: SectionWriter::new(),
            instructions_composer: InstructionsComposer::new(),
            alert_writer: AlertWriter::new(),
        }
    }

    pub fn write_announcement(&self, text: &str) -> Result<(), String> {
        let buffer = &mut self.buffer_writer.buffer();

        let buffer_writer_result = self
            .buffer_writer
            .print(self.announcements_composer.compose(text, buffer)?);

        if let Err(error) = buffer_writer_result {
            return Err(error.to_string());
        }

        Ok(())
    }

    pub fn write_instructions_for_decision(
        &self,
        decision: &Decision,
        faction: &Factions,
    ) -> Result<(), String> {
        let instructions = produce_instructions_for_mutations(decision, faction)?;

        for instruction in instructions.iter() {
            self.write_instruction(instruction)?;
        }

        Ok(())
    }

    pub fn write_instruction(&self, text: &str) -> Result<(), String> {
        let buffer = &mut self.buffer_writer.buffer();

        let buffer_writer_result = self
            .buffer_writer
            .print(self.instructions_composer.compose(text, buffer)?);

        if let Err(error) = buffer_writer_result {
            return Err(error.to_string());
        }

        Ok(())
    }

    pub fn write_section(&self, text: &str) -> Result<(), String> {
        let buffer = &mut self.buffer_writer.buffer();

        let buffer_writer_result = self
            .buffer_writer
            .print(self.section_writer.write(text, buffer)?);

        if let Err(error) = buffer_writer_result {
            return Err(error.to_string());
        }

        Ok(())
    }

    pub fn write_alert(&self, text: &str) -> Result<(), String> {
        let buffer = &mut self.buffer_writer.buffer();

        let buffer_writer_result = self
            .buffer_writer
            .print(self.alert_writer.write(text, buffer)?);

        if let Err(error) = buffer_writer_result {
            return Err(error.to_string());
        }

        Ok(())
    }
}
