use board::space_identifiers::SpaceIdentifiers;
use decision_making::interpretation::event_instructions::deploy_from_out_of_play_data::DeployFromOutOfPlayData;

#[derive(Debug, Clone)]
pub struct InterpretedIntentions {
    wants_to_do_an_operation: bool,
    wants_to_pass: bool,
    wants_to_activate_the_event: bool,
    spaces_for_event: Vec<SpaceIdentifiers>,
    wants_to_do_an_operation_only: bool,
    wants_to_train: bool,
    wants_to_rally: bool,
    wants_to_sweep: bool,
    wants_to_pacify: bool,
    spaces_for_operation: Vec<SpaceIdentifiers>,
    digits_for_operation: Vec<u8>,
    wants_to_improve_the_trail: bool,
    wants_to_govern: bool,
    wants_to_tax: bool,
    spaces_for_special_activity: Vec<SpaceIdentifiers>,
    digits_for_special_activity: Vec<u8>,
    deploy_from_out_of_play_data: Vec<DeployFromOutOfPlayData>,
}

impl Default for InterpretedIntentions {
    fn default() -> Self {
        Self::new()
    }
}

impl InterpretedIntentions {
    pub fn new() -> InterpretedIntentions {
        InterpretedIntentions {
            wants_to_do_an_operation: false,
            wants_to_pass: false,
            wants_to_activate_the_event: false,
            spaces_for_event: Vec::new(),
            wants_to_do_an_operation_only: false,
            wants_to_train: false,
            wants_to_rally: false,
            wants_to_sweep: false,
            wants_to_pacify: false,
            spaces_for_operation: Vec::new(),
            digits_for_operation: Vec::new(),
            wants_to_improve_the_trail: false,
            wants_to_govern: false,
            wants_to_tax: false,
            spaces_for_special_activity: Vec::new(),
            digits_for_special_activity: Vec::new(),
            deploy_from_out_of_play_data: Vec::<DeployFromOutOfPlayData>::new(),
        }
    }

    pub fn wants_to_do_an_operation(&mut self) {
        self.wants_to_do_an_operation = true;
    }

    pub fn does_it_want_to_do_an_operation(&self) -> bool {
        self.wants_to_do_an_operation
    }

    pub fn wants_to_activate_the_event(&mut self) {
        self.wants_to_activate_the_event = true;
    }

    pub fn does_it_want_to_activate_the_event(&self) -> bool {
        self.wants_to_activate_the_event
    }

    pub fn wants_to_govern(&mut self) {
        self.wants_to_govern = true;
    }

    pub fn does_it_want_to_govern(&self) -> bool {
        self.wants_to_govern
    }

    pub fn wants_to_tax(&mut self) {
        self.wants_to_tax = true;
    }

    pub fn does_it_want_to_tax(&self) -> bool {
        self.wants_to_tax
    }

    pub fn does_it_want_to_pass(&self) -> bool {
        self.wants_to_pass
    }

    pub fn wants_to_train(&mut self) {
        self.wants_to_train = true;
    }

    pub fn wants_to_rally(&mut self) {
        self.wants_to_rally = true;
    }

    pub fn does_it_want_to_rally(&self) -> bool {
        self.wants_to_rally
    }

    pub fn wants_to_sweep(&mut self) {
        self.wants_to_sweep = true;
    }

    pub fn does_it_want_to_sweep(&self) -> bool {
        self.wants_to_sweep
    }

    pub fn wants_to_pacify(&mut self) {
        self.wants_to_pacify = true;
    }

    pub fn does_it_want_to_train(&self) -> bool {
        self.wants_to_train
    }

    pub fn does_it_want_to_pacify(&self) -> bool {
        self.wants_to_pacify
    }

    pub fn add_space_for_event(&mut self, space_to_add: SpaceIdentifiers) {
        self.spaces_for_event.push(space_to_add);
    }

    pub fn get_spaces_for_event(&self) -> Vec<SpaceIdentifiers> {
        self.spaces_for_event.clone()
    }

    pub fn add_space_for_operation(&mut self, space_to_add: SpaceIdentifiers) {
        self.spaces_for_operation.push(space_to_add);
    }

    pub fn get_spaces_for_operation(&self) -> Vec<SpaceIdentifiers> {
        self.spaces_for_operation.clone()
    }

    pub fn add_digit_for_operation(&mut self, digit_to_add: u8) {
        self.digits_for_operation.push(digit_to_add);
    }

    pub fn get_digits_for_operation(&self) -> Vec<u8> {
        self.digits_for_operation.clone()
    }

    pub fn add_space_for_special_activity(&mut self, space_to_add: SpaceIdentifiers) {
        self.spaces_for_special_activity.push(space_to_add);
    }

    pub fn get_spaces_for_special_activity(&self) -> Vec<SpaceIdentifiers> {
        self.spaces_for_special_activity.clone()
    }

    pub fn add_digit_for_special_activity(&mut self, digit_to_add: u8) {
        self.digits_for_special_activity.push(digit_to_add);
    }

    pub fn has_it_decided_on_an_operation(&self) -> bool {
        self.wants_to_train || self.wants_to_rally || self.wants_to_sweep
    }

    pub fn wants_to_do_an_operation_only(&mut self) {
        self.wants_to_do_an_operation_only = true;
    }

    pub fn does_it_want_to_do_an_operation_only(&self) -> bool {
        self.wants_to_do_an_operation_only
    }

    pub fn has_it_chosen_a_special_activity(&self) -> bool {
        self.wants_to_govern || self.wants_to_tax
    }

    pub fn wants_to_pass(&mut self) {
        self.wants_to_pass = true;
    }

    pub fn wants_to_improve_the_trail(&mut self) {
        self.wants_to_improve_the_trail = true;
    }

    pub fn does_it_want_to_improve_the_trail(&self) -> bool {
        self.wants_to_improve_the_trail
    }

    pub fn add_to_deploy_from_out_of_play_data(
        &mut self,
        deploy_from_out_of_play_data: DeployFromOutOfPlayData,
    ) {
        self.deploy_from_out_of_play_data
            .push(deploy_from_out_of_play_data);
    }

    pub fn get_deploy_from_out_of_play_data(&self) -> &Vec<DeployFromOutOfPlayData> {
        &self.deploy_from_out_of_play_data
    }
}
