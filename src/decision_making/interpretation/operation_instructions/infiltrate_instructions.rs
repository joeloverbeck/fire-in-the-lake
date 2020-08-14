


use board::space_identifiers::SpaceIdentifiers;

#[derive(Debug, Clone)]
pub struct InfiltrateInstructions {
    type_of_infiltrate: String,
    space: SpaceIdentifiers,
    vc_piece: Option<String>,    
}

impl InfiltrateInstructions{
    pub fn new(type_of_infiltrate: String, space: SpaceIdentifiers, vc_piece: std::option::Option<String>) -> InfiltrateInstructions {
        InfiltrateInstructions{
            type_of_infiltrate,
            space,
            vc_piece
        }
    }

    pub fn get_type_of_infiltrate(&self) -> String {
        self.type_of_infiltrate.clone()
    }

    pub fn get_space(&self) -> SpaceIdentifiers {
        self.space
    }

    pub fn get_vc_piece(&self) -> &Option<String> {
        &self.vc_piece
    }
}