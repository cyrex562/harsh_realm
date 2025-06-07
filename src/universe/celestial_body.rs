use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug,Serialize,Deserialize)]
pub enum CelestialBodyType {
    Star
    
}

#[derive(Debug,Clone,Deserialize,Serialize)]
pub struct CelestialBody {
    name: String,
    id: Uuid,
    body_type: CelestialBodyType,
}