use board::domain::board::Board;
use game_definitions::faction_stats::FactionStats;
use game_definitions::factions::Factions;
use players::domain::faction_stats_mutation::FactionStatsMutation;
use players::domain::mutation_types::MutationTypes;

pub fn produce_faction_stats_mutations_for_passing(
    faction: &Factions,
    board: &Board,
) -> Result<Vec<FactionStatsMutation>, String> {
    let mut faction_stats_mutations: Vec<FactionStatsMutation> = Vec::new();

    // Rules: if the passing faction is insurgent, +1 Resources for that faction (NVA or VC)
    // If the passing faction is COIN, +3 ARVN Resources (nothing for the US, even if it's the one passing)
    if faction == &Factions::VC {
        faction_stats_mutations.push(FactionStatsMutation::new(
            FactionStats::VcResources,
            MutationTypes::Increase,
            board.get_faction_stat(FactionStats::VcResources)?,
            1,
        ));
    } else if faction == &Factions::NVA {
        faction_stats_mutations.push(FactionStatsMutation::new(
            FactionStats::NvaResources,
            MutationTypes::Increase,
            board.get_faction_stat(FactionStats::NvaResources)?,
            1,
        ));
    } else if faction == &Factions::US || faction == &Factions::ARVN {
        faction_stats_mutations.push(FactionStatsMutation::new(
            FactionStats::ArvnResources,
            MutationTypes::Increase,
            board.get_faction_stat(FactionStats::ArvnResources)?,
            3,
        ));
    }

    Ok(faction_stats_mutations)
}
