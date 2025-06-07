use uuid::Uuid;

use crate::units::unit_type::UnitType;

pub enum InstallationPurpose {
    Mine,
    Refinery,
    Factory,
    Military,
    Research,
}

pub struct Installation {
    id: Uuid,
    name: String,
    purpose: InstallationPurpose,
    crew: Option<UnitType>,
}
