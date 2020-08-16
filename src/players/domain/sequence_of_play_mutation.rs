use game_definitions::factions::Factions;
use sequence_of_play::domain::sequence_of_play_slots::SequenceOfPlaySlots;
use sequence_of_play::domain::slot_occupancy::SlotOccupancy;

#[derive(Debug)]
pub struct SequenceOfPlayMutation {
    sequence_of_play_slot: SequenceOfPlaySlots,
    slot_occupancy: SlotOccupancy,
    faction: Factions,
}

impl SequenceOfPlayMutation {
    pub fn new(
        sequence_of_play_slot: SequenceOfPlaySlots,
        slot_occupancy: SlotOccupancy,
        faction: Factions,
    ) -> SequenceOfPlayMutation {
        SequenceOfPlayMutation {
            sequence_of_play_slot,
            slot_occupancy,
            faction,
        }
    }

    pub fn get_sequence_of_play_slot(&self) -> &SequenceOfPlaySlots {
        &self.sequence_of_play_slot
    }

    pub fn get_slot_occupancy(&self) -> &SlotOccupancy {
        &self.slot_occupancy
    }

    pub fn get_faction(&self) -> &Factions {
        &self.faction
    }
}
