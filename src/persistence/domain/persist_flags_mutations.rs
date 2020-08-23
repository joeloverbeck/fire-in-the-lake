use flags::controllers::flags_controller::FlagsController;
use players::domain::flags_mutation::FlagsMutation;
use players::domain::mutation_types::MutationTypes;

pub fn persist_flags_mutations(
    flags_mutations: &[FlagsMutation],
    flags_controller: &mut FlagsController,
) -> Result<(), String> {
    for mutation in flags_mutations {
        if mutation.get_mutation_type() == &MutationTypes::Set {
            flags_controller.set_flag(*mutation.get_flag())?;
        } else {
            panic!("Case not handled for flag mutation {:?}", mutation);
        }
    }

    Ok(())
}
