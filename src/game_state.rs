use serde::{Deserialize, Serialize};
use crate::simulation::simulation::Simulation;

#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct GameState {
    pub simulation: Simulation,
    // Add other game-specific state here that is not part of the core simulation.
}

impl GameState {
    /// Creates a new game state with a fresh simulation.
    pub fn new() -> Self {
        Self {
            simulation: Simulation::new(),
        }
    }
    
    /// advances the game state by processing and simulation turn.
    pub fn process_turn(&mut self) {
        self.simulation.process_turn();
    }
}

impl Default for GameState {
    fn default() -> Self {
        Self::new()
    }   
}