use chrono::NaiveDate;
use harsh_realm_sim::universe::solar_system_manager::SolarSystemManager;

#[test]
fn load_csv_and_step() {
    let start_date = NaiveDate::from_ymd_opt(2070, 1, 1).unwrap();
    let mut solar = SolarSystemManager::new(start_date);

    let csv_path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join("data")
        .join("solar_system_data.csv");

    solar
        .load_from_csv(&csv_path)
        .expect("should load CSV");

    // basic sanity: at least one body loaded
    assert!(!solar.get_all_bodies().is_empty());

    // advance 30 days (one turn) and confirm date progression
    solar.update_all_positions(30.0);
    assert_eq!(
        solar.get_game_date(),
        start_date + chrono::Duration::days(30)
    );
}