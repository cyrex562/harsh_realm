use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct Unit {
    id: Uuid,
    name: String,
}
