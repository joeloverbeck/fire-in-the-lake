use players::domain::faction_stats_mutation::FactionStatsMutation;
use players::domain::flags_mutation::FlagsMutation;
use players::domain::forces_mutation::ForcesMutation;
use players::domain::sequence_of_play_mutation::SequenceOfPlayMutation;
use players::domain::space_mutation::SpaceMutation;

#[derive(Debug)]
pub struct Mutations {
    sequence_of_play_mutations: Option<Vec<SequenceOfPlayMutation>>,
    space_mutations: Option<Vec<SpaceMutation>>,
    faction_stats_mutations: Option<Vec<FactionStatsMutation>>,
    forces_mutations: Option<Vec<ForcesMutation>>,
    flags_mutations: Option<Vec<FlagsMutation>>,
}

impl Default for Mutations {
    fn default() -> Self {
        Self::new()
    }
}

impl Mutations {
    pub fn new() -> Mutations {
        Mutations {
            sequence_of_play_mutations: None,
            space_mutations: None,
            faction_stats_mutations: None,
            forces_mutations: None,
            flags_mutations: None,
        }
    }

    pub fn get_sequence_of_play_mutations(&self) -> Result<&Vec<SequenceOfPlayMutation>, String> {
        if self.sequence_of_play_mutations.is_none() {
            panic!("Attempted to get the sequence of play mutations of a decision, but there were none!");
        }

        Ok(self.sequence_of_play_mutations.as_ref().unwrap())
    }

    pub fn set_sequence_of_play_mutations(
        &mut self,
        sequence_of_play_mutations: Vec<SequenceOfPlayMutation>,
    ) -> Result<(), String> {
        self.sequence_of_play_mutations = Some(sequence_of_play_mutations);

        Ok(())
    }

    pub fn has_sequence_of_play_mutations(&self) -> Result<bool, String> {
        Ok(self.sequence_of_play_mutations.is_some())
    }

    pub fn get_space_mutations(&self) -> Result<&Vec<SpaceMutation>, String> {
        if self.space_mutations.is_none() {
            panic!("Attempted to get the space mutations of a decision, but there were none!");
        }

        Ok(self.space_mutations.as_ref().unwrap())
    }

    pub fn set_space_mutations(
        &mut self,
        space_mutations: Vec<SpaceMutation>,
    ) -> Result<(), String> {
        self.space_mutations = Some(space_mutations);

        Ok(())
    }

    pub fn has_space_mutations(&self) -> Result<bool, String> {
        Ok(self.space_mutations.is_some())
    }

    pub fn get_faction_stats_mutations(&self) -> Result<&Vec<FactionStatsMutation>, String> {
        if self.faction_stats_mutations.is_none() {
            panic!(
                "Attempted to get the faction stats mutations of a decision, but there were none!"
            );
        }

        Ok(self.faction_stats_mutations.as_ref().unwrap())
    }

    pub fn set_faction_stats_mutations(
        &mut self,
        faction_stats_mutations: Vec<FactionStatsMutation>,
    ) -> Result<(), String> {
        self.faction_stats_mutations = Some(faction_stats_mutations);

        Ok(())
    }

    pub fn has_faction_stats_mutations(&self) -> Result<bool, String> {
        Ok(self.faction_stats_mutations.is_some())
    }

    pub fn get_forces_mutations(&self) -> Result<&Vec<ForcesMutation>, String> {
        if self.forces_mutations.is_none() {
            panic!("Attempted to get the forces mutations of a decision, but there were none!");
        }

        Ok(self.forces_mutations.as_ref().unwrap())
    }

    pub fn set_forces_mutations(
        &mut self,
        forces_mutations: Vec<ForcesMutation>,
    ) -> Result<(), String> {
        self.forces_mutations = Some(forces_mutations);

        Ok(())
    }

    pub fn has_forces_mutations(&self) -> Result<bool, String> {
        Ok(self.forces_mutations.is_some())
    }

    pub fn get_flags_mutations(&self) -> Result<&Vec<FlagsMutation>, String> {
        if self.flags_mutations.is_none() {
            panic!("Attempted to get the flags mutations of a decision, but there were none!");
        }

        Ok(self.flags_mutations.as_ref().unwrap())
    }

    pub fn set_flags_mutations(
        &mut self,
        flags_mutations: Vec<FlagsMutation>,
    ) -> Result<(), String> {
        self.flags_mutations = Some(flags_mutations);

        Ok(())
    }

    pub fn has_flags_mutations(&self) -> Result<bool, String> {
        Ok(self.flags_mutations.is_some())
    }
}
