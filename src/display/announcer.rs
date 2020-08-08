use factions::Factions;

pub struct Announcer {}

impl Announcer {
    pub fn new() -> Announcer {
        Announcer {}
    }

    pub fn instruct_to_move_faction_cylinder_from_eligible_to_first_eligible_event_box(
        &self,
        faction: Factions,
    ) {
        self.print_instruction(format!(
            "Move the cylinder for the {} faction from Eligible to the First Eligible Event box.",
            faction
        ));
    }

    pub fn instruct_to_move_faction_cylinder_from_eligible_to_passed_box(&self, faction: Factions) {
        self.print_instruction(format!(
            "Move the cylinder for the {} faction from Eligible to the Pass box.",
            faction
        ));
    }

    pub fn instruct_to_move_faction_cylinder_from_eligible_to_operation_and_special_activity_box(
        &self,
        faction: Factions,
    ) {
        self.print_instruction(format!(
            "Move the cylinder for the {} faction from Eligible to the Operation and Special Activity box.",
            faction
        ));
    }

    fn print_instruction(&self, instruction: String) {
        let full_string = [String::from("==> "), instruction].concat();

        println!("{}", full_string);
    }
}

impl Default for Announcer {
    fn default() -> Self {
        Self::new()
    }
}
