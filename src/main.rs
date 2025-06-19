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
    // Initialize the logger
    Builder::from_env(Env::default().default_filter_or("info")).init();
    info!("Starting Harsh Realm prototype simulation.");
    
    // Create a new game state
    let mut game_state = game_state::GameState::new();
    
    // Load solar system data from CSV
    match game_state.load_solar_system_data("data/solar_system_data.csv") {
        Ok(()) => info!("Successfully loaded solar system data"),
        Err(e) => {
            eprintln!("Failed to load solar system data: {}", e);
            return;
        }
    }
    
    info!("Game state created successfully");
    info!("Initial game date: {}", game_state.get_formatted_date());
    
    // define the number of turns to simulate for this basic loop
    let num_turns_to_simulate = 5;
    
    info!("Starting simulation with {} turns (30 days each)", num_turns_to_simulate);
    
    // main program loop
    for turn in 1..=num_turns_to_simulate {
        info!("=== Turn {} ===", turn);
        game_state.process_turn();
        info!("Game date after turn {}: {}", turn, game_state.get_formatted_date());
        
        // Display some sample body positions
        let bodies = game_state.solar_system.get_all_bodies();
        let sample_bodies = ["Earth", "Mars", "Jupiter", "Saturn"];
        
        for body_name in sample_bodies.iter() {
            if let Some(body) = bodies.get(*body_name) {
                if let Some(ref orbital_state) = body.orbital_state {
                    let cartesian = orbital_state.to_cartesian();
                    info!("  {}: Position ({:.0}, {:.0}) km, Angle: {:.1}°", 
                          body_name, cartesian.x, cartesian.y, orbital_state.angle_degrees());
                }
            }
        }
    }
    
    info!("Simulation finished after {} turns.", num_turns_to_simulate);
}
