use env_logger::{Builder, Env};
use log::info;

mod buildings;
mod events;
mod faction;
mod game;
mod game_state;
mod maps;
mod population;
mod procedural_generation;
mod production;
mod resources;
mod simulation;
mod structures;
mod units;
mod universe;

fn main() {
    // Initialize the logger, This will allow info, warn, and error macros to print messages. By default, it logs messages at INFO level or higher or stderr. You can set RUST_LOG environment variable (e.g. RUST_LOG=info) to control logging.
    Builder::from_env(Env::default().default_filter_or("info")).init();
    info!("Starting Harsh Realm prototype simulation.");
    
    // Create a new game state
    let mut game_state = game_state::GameState::new();
    
    // define the number of turns to simulate for this basic loop
    let num_turns_to_simulate = 5;
    
    // main program loop
    for _ in 0..num_turns_to_simulate {
        game_state.process_turn();
        // in a real game, you would render the state, handle user input, etc.
    }
    
    info!("Simulation finished after {} turns.", num_turns_to_simulate);
    
    
}
