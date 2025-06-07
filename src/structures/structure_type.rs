use crate::structures::{
    installation::Installation, settlement::Settlement, spacecraft::Spacecraft,
};

pub enum StructureType {
    Settlement(Settlement),
    Installation(Installation),
    Spacecraft(Spacecraft),
}
