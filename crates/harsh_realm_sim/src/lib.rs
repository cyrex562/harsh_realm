//! Harsh Realm – simulation core library.
//!
//! This crate contains the data model and turn-based simulation logic.  No
//! rendering or engine-specific code should live here.

pub mod buildings;
pub mod events;
pub mod faction;
pub mod game;
pub mod game_state;
pub mod maps;
pub mod population;
pub mod procedural_generation;
pub mod production;
pub mod resources;
pub mod simulation;
pub mod structures;
pub mod units;
pub mod universe;

pub mod prelude {
    pub use crate::game_state::GameState;
    pub use crate::simulation::simulation::Simulation;
}
