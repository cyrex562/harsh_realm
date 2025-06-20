use bevy::prelude::*;
use bevy::prelude::Window;
use bevy::window::PrimaryWindow;
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use log::info;
use harsh_realm_sim::game_state;
use bevy::render::mesh::{Indices, PrimitiveTopology};
use bevy::sprite::MaterialMesh2dBundle;

#[derive(Component)]
struct PlanetLabel(String);

#[derive(Component)]
struct InnerOnly;

#[derive(Component)]
struct BeltOnly;

#[derive(Component)]
struct OuterOnly;

#[derive(Resource, PartialEq, Eq, Clone, Copy)]
enum ViewMode {
    Inner,
    Belt,
    Outer,
}

fn main() {
    // Initialize the logger
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    info!("Starting Harsh Realm prototype simulation.");

    let mut app = App::new();

    app.add_plugins(
        DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: (1920., 1080.).into(),
                title: "Harsh Realm".into(),
                ..Default::default()
            }),
            ..Default::default()
        }),
    )
    .add_plugins(EguiPlugin)
    .add_systems(Startup, setup_system)
    .add_systems(Update, ui_system)
    .add_systems(Update, solar_system_view_system)
    .add_systems(PostUpdate, planet_label_system)
    .add_systems(PostUpdate, update_visibility_system);

    app.run();
}

fn setup_system(
    mut commands: Commands,
    _asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    info!("Creating game state...");
    let mut game_state = game_state::GameState::new();

    info!("Loading solar system data...");
    if let Err(e) = game_state.load_solar_system_data("data/solar_system_data.csv") {
        eprintln!("Failed to load solar system data: {}", e);
        // Handle error, maybe by exiting the app
        return;
    }
    info!("Successfully loaded solar system data");

    // Spawn a 2D camera
    commands.spawn(Camera2dBundle::default());

    // Load default font for labels
    let font: Handle<Font> = Default::default();

    // Sun label now sits below the Sun so that all bodies use a consistent bottom-center labelling.
    const SUN_LABEL_OFFSET: f32 = 20.0; // > Sun radius (15) + small margin
    commands.spawn(Text2dBundle {
        text: Text::from_section(
            "Sun",
            TextStyle { font: font.clone(), font_size: 16.0, color: Color::WHITE },
        ),
        transform: Transform::from_translation(Vec3::new(0.0, -SUN_LABEL_OFFSET, 1.0)),
        ..Default::default()
    });

    // Spawn label entities for inner planets
    let planet_names = ["Mercury", "Venus", "Earth", "Mars"];
    for name in planet_names.iter() {
        commands.spawn((
            Text2dBundle {
                text: Text::from_section(
                    (*name).to_string(),
                    TextStyle {
                        font: font.clone(),
                        font_size: 14.0,
                        color: Color::WHITE,
                    },
                ),
                transform: Transform::from_translation(Vec3::new(0.0,0.0,1.0)),
                ..Default::default()
            },
            PlanetLabel((*name).to_string()),
            InnerOnly,
        ));
    }

    // Spawn annulus mesh for asteroid belt
    const SCALE: f32 = 200.0;
    const ASTEROID_CENTER_AU: f32 = 2.77;
    const ASTEROID_BAND_WIDTH_AU: f32 = 0.4;
    let inner_px = (ASTEROID_CENTER_AU - ASTEROID_BAND_WIDTH_AU * 0.5) * SCALE;
    let outer_px = (ASTEROID_CENTER_AU + ASTEROID_BAND_WIDTH_AU * 0.5) * SCALE;

    let annulus = generate_annulus_mesh(inner_px, outer_px, 256);
    let mesh_handle = meshes.add(annulus);
    let mat_handle = materials.add(ColorMaterial::from(Color::rgba(0.0, 1.0, 0.0, 0.15)));

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: mesh_handle.into(),
            material: mat_handle,
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, -0.5)),
            ..Default::default()
        },
        InnerOnly,
    ));

    // Asteroid Belt label – we spawn two identical labels so it is visible in both the
    // inner‐system and outer‐system views. The `planet_label_system` will reposition the
    // label every frame, so the initial translation value here is not important.
    let belt_font: Handle<Font> = font.clone();

    // Label used in the inner-system view
    commands.spawn( (
        Text2dBundle {
            text: Text::from_section(
                "Asteroid Belt",
                TextStyle { font: belt_font.clone(), font_size: 16.0, color: Color::WHITE },
            ),
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 1.0)),
            ..Default::default()
        },
        PlanetLabel("Asteroid Belt".to_string()),
        InnerOnly,
    ));

    // Label used in the outer-system view
    commands.spawn( (
        Text2dBundle {
            text: Text::from_section(
                "Asteroid Belt",
                TextStyle { font: belt_font.clone(), font_size: 16.0, color: Color::WHITE },
            ),
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 1.0)),
            ..Default::default()
        },
        PlanetLabel("Asteroid Belt".to_string()),
        OuterOnly,
    ));

    // Labels for each asteroid (displayed in belt view)
    for (name, body) in game_state.solar_system.get_all_bodies() {
        if body.region.to_lowercase().contains("asteroid belt") {
            commands.spawn((
                Text2dBundle {
                    text: Text::from_section(
                        name.clone(),
                        TextStyle { font: font.clone(), font_size: 12.0, color: Color::WHITE },
                    ),
                    transform: Transform::from_translation(Vec3::new(0.0, 0.0, 1.0)),
                    ..Default::default()
                },
                PlanetLabel(name.clone()),
                BeltOnly,
            ));
        }
    }

    // Jupiter label (shown in belt view)
    commands.spawn((
        Text2dBundle {
            text: Text::from_section(
                "Jupiter",
                TextStyle { font: font.clone(), font_size: 14.0, color: Color::WHITE },
            ),
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 1.0)),
            ..Default::default()
        },
        PlanetLabel("Jupiter".to_string()),
        BeltOnly,
    ));

    // Jupiter label for outer view
    commands.spawn((
        Text2dBundle {
            text: Text::from_section(
                "Jupiter",
                TextStyle { font: font.clone(), font_size: 14.0, color: Color::WHITE },
            ),
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 1.0)),
            ..Default::default()
        },
        PlanetLabel("Jupiter".to_string()),
        OuterOnly,
    ));

    // Labels for Saturn, Uranus, Neptune, Pluto for outer view
    let outer_planets = [
        ("Saturn", Color::WHITE),
        ("Uranus", Color::WHITE),
        ("Neptune", Color::WHITE),
        ("Pluto", Color::WHITE),
    ];
    for (name, _c) in outer_planets.iter() {
        commands.spawn((
            Text2dBundle {
                text: Text::from_section(
                    (*name).to_string(),
                    TextStyle { font: font.clone(), font_size: 14.0, color: Color::WHITE },
                ),
                transform: Transform::from_translation(Vec3::new(0.0, 0.0, 1.0)),
                ..Default::default()
            },
            PlanetLabel((*name).to_string()),
            OuterOnly,
        ));
    }

    // Store resources after spawning setup entities
    commands.insert_resource(game_state);
    // Insert initial view mode
    commands.insert_resource(ViewMode::Inner);
}

fn ui_system(mut contexts: EguiContexts, mut game_state: ResMut<game_state::GameState>, mut view_mode: ResMut<ViewMode>) {
    let ctx = contexts.ctx_mut();

    egui::Window::new("Controls").show(ctx, |ui| {
        ui.label(format!(
            "Turn: {} | Date: {}",
            game_state.simulation.current_turn,
            game_state.get_formatted_date()
        ));
        if ui.button("End Turn").clicked() {
            info!("=== Turn {} ===", game_state.simulation.current_turn + 1);
            game_state.process_turn();
            info!("Game date after turn: {}", game_state.get_formatted_date());
        }

        if ui.button("Exit").clicked() {
            std::process::exit(0);
        }

        ui.separator();

        // View switch buttons
        let inner_enabled = *view_mode != ViewMode::Inner;
        if ui
            .add_enabled(inner_enabled, egui::Button::new("Inner System"))
            .clicked()
        {
            *view_mode = ViewMode::Inner;
        }

        let belt_enabled = *view_mode != ViewMode::Belt;
        if ui
            .add_enabled(belt_enabled, egui::Button::new("Asteroid Belt"))
            .clicked()
        {
            *view_mode = ViewMode::Belt;
        }

        let outer_enabled = *view_mode != ViewMode::Outer;
        if ui
            .add_enabled(outer_enabled, egui::Button::new("Outer System"))
            .clicked()
        {
            *view_mode = ViewMode::Outer;
        }
    });
}

// Define colors
const SUN_COLOR: Color = Color::YELLOW;
const EARTH_COLOR: Color = Color::rgb(0.3, 0.3, 1.0);
const MARS_COLOR: Color = Color::rgb(1.0, 0.5, 0.3);
const VENUS_COLOR: Color = Color::rgb(0.9, 0.8, 0.7);
const MERCURY_COLOR: Color = Color::rgb(0.7, 0.7, 0.7);
const JUPITER_COLOR: Color = Color::rgb(1.0, 0.6, 0.3);
const ORBITAL_PATH_COLOR: Color = Color::rgba(0.5, 0.5, 0.5, 0.5);
// Offset applied to all dynamic labels so they appear just below (bottom-center) the body they
// describe.  A small negative Y shift moves the text downward; X shift is zero so the text is
// centred horizontally.
const LABEL_OFFSET: Vec2 = Vec2::new(0.0, -12.0);
const SATURN_COLOR: Color = Color::rgb(0.95, 0.9, 0.6);
const URANUS_COLOR: Color = Color::rgb(0.5, 0.8, 0.9);
const NEPTUNE_COLOR: Color = Color::rgb(0.3, 0.5, 1.0);
const PLUTO_COLOR: Color = Color::rgb(0.9, 0.8, 0.7);

fn solar_system_view_system(
    mut gizmos: Gizmos,
    game_state: Res<game_state::GameState>,
    view_mode: Res<ViewMode>,
    windows_q: Query<&Window, With<PrimaryWindow>>,
) {
    let scale_factor = 200.0; // 1 AU = 200 pixels

    // Center of the screen is (0,0) in Bevy's 2D world space
    let center = Vec2::ZERO;

    // Draw Sun
    gizmos.circle_2d(center, 15.0, SUN_COLOR).segments(32);

    match *view_mode {
        ViewMode::Inner => {
            draw_inner_system(&mut gizmos, &game_state, center, scale_factor);
        }
        ViewMode::Belt => {
            draw_belt_view(&mut gizmos, &game_state, center, scale_factor);
        }
        ViewMode::Outer => {
            let window = windows_q.single();
            let scale = compute_outer_scale(&game_state, window);
            draw_outer_system(&mut gizmos, &game_state, center, scale);
        }
    }
}

fn draw_inner_system(gizmos: &mut Gizmos, game_state: &game_state::GameState, center: Vec2, scale_factor: f32) {
    // Inner planets rendering (existing implementation simplified)
    let bodies = game_state.solar_system.get_all_bodies();
    let planets_to_draw = ["Mercury", "Venus", "Earth", "Mars"];
    let planet_colors = [MERCURY_COLOR, VENUS_COLOR, EARTH_COLOR, MARS_COLOR];

    use std::f32::consts::PI;
    for (i, planet_name) in planets_to_draw.iter().enumerate() {
        if let Some(body) = bodies.get(*planet_name) {
            if let Some(ref orbital_state) = body.orbital_state {
                let a_au = (orbital_state.parameters.semi_major_axis / 149_597_870.7) as f32;
                let e = orbital_state.parameters.eccentricity as f32;
                let b_au = a_au * (1.0 - e * e).sqrt();
                let c_au = a_au * e;

                let segments = 128;
                let mut prev = None::<Vec2>;
                for s in 0..=segments {
                    let theta = s as f32 / segments as f32 * 2.0 * PI;
                    let x = (a_au * theta.cos() - c_au) * scale_factor;
                    let y = (b_au * theta.sin()) * scale_factor;
                    let pos = center + Vec2::new(x, y);
                    if let Some(prev_pos) = prev {
                        gizmos.line_2d(prev_pos, pos, ORBITAL_PATH_COLOR);
                    }
                    prev = Some(pos);
                }

                let cart = orbital_state.to_cartesian();
                let x = center.x + ((cart.x as f32) / 149_597_870.7_f32) * scale_factor;
                let y = center.y + ((cart.y as f32) / 149_597_870.7_f32) * scale_factor;
                gizmos.circle_2d(Vec2::new(x, y), 5.0, planet_colors[i]).segments(16);
            }
        }
    }

    // edges already drawn earlier by caller
}

fn draw_belt_view(gizmos: &mut Gizmos, game_state: &game_state::GameState, center: Vec2, _scale_factor: f32) {
    let scale_factor = 120.0;

    use std::f32::consts::PI;
    // Jupiter
    let bodies = game_state.solar_system.get_all_bodies();
    if let Some(body) = bodies.get("Jupiter") {
        if let Some(ref orbital_state) = body.orbital_state {
            let a_au = (orbital_state.parameters.semi_major_axis / 149_597_870.7) as f32;
            let e = orbital_state.parameters.eccentricity as f32;
            let b_au = a_au * (1.0 - e * e).sqrt();
            let c_au = a_au * e;
            let segments = 256;
            let mut prev = None::<Vec2>;
            for s in 0..=segments {
                let theta = s as f32 / segments as f32 * 2.0 * PI;
                let x = (a_au * theta.cos() - c_au) * scale_factor;
                let y = (b_au * theta.sin()) * scale_factor;
                let pos = center + Vec2::new(x, y);
                if let Some(prev_pos) = prev {
                    gizmos.line_2d(prev_pos, pos, ORBITAL_PATH_COLOR);
                }
                prev = Some(pos);
            }

            let cart = orbital_state.to_cartesian();
            let x = center.x + ((cart.x as f32) / 149_597_870.7_f32) * scale_factor;
            let y = center.y + ((cart.y as f32) / 149_597_870.7_f32) * scale_factor;
            gizmos.circle_2d(Vec2::new(x, y), 7.0, JUPITER_COLOR).segments(20);
        }
    }

    // Asteroids (any body whose region is "asteroid belt")
    for (_name, body) in bodies.iter() {
        if body.region.to_lowercase().contains("asteroid belt") {
            if let Some(ref orbital_state) = body.orbital_state {
                // orbit
                let a_au = (orbital_state.parameters.semi_major_axis / 149_597_870.7) as f32;
                let e = orbital_state.parameters.eccentricity as f32;
                let b_au = a_au * (1.0 - e * e).sqrt();
                let c_au = a_au * e;
                let segments = 64;
                let mut prev = None::<Vec2>;
                for s in 0..=segments {
                    let theta = s as f32 / segments as f32 * 2.0 * PI;
                    let x = (a_au * theta.cos() - c_au) * scale_factor;
                    let y = (b_au * theta.sin()) * scale_factor;
                    let pos = center + Vec2::new(x, y);
                    if let Some(prev_pos) = prev {
                        gizmos.line_2d(prev_pos, pos, ORBITAL_PATH_COLOR);
                    }
                    prev = Some(pos);
                }

                let cart = orbital_state.to_cartesian();
                let x = center.x + ((cart.x as f32) / 149_597_870.7_f32) * scale_factor;
                let y = center.y + ((cart.y as f32) / 149_597_870.7_f32) * scale_factor;
                gizmos.circle_2d(Vec2::new(x, y), 2.0, Color::GREEN).segments(12);
            }
        }
    }
}

fn compute_outer_scale(gs: &game_state::GameState, window: &Window) -> f32 {
    // max aphelion for outer planets considered
    let planets = ["Jupiter","Saturn","Uranus","Neptune","Pluto"];
    let mut max = 0.0f32;
    for p in planets.iter() {
        if let Some(b) = gs.solar_system.get_body(p) {
            if let Some(os) = &b.orbital_state {
                let a = (os.parameters.semi_major_axis/149_597_870.7) as f32;
                let e = os.parameters.eccentricity as f32;
                let apo = a*(1.0+e);
                if apo>max {max=apo;}
            }
        }
    }
    let half = 0.5*window.height().min(window.width()) - 40.0; // margin
    half/max
}

fn draw_outer_system(
    gizmos: &mut Gizmos,
    game_state: &game_state::GameState,
    center: Vec2,
    scale_factor: f32,
) {
    use std::f32::consts::PI;

    // Draw single green circle for asteroid belt
    const AST_BELT_AU: f32 = 2.77;
    let radius = AST_BELT_AU * scale_factor;
    let segments = 256;
    let mut prev = None::<Vec2>;
    for s in 0..=segments {
        let theta = s as f32 / segments as f32 * 2.0 * PI;
        let x = radius * theta.cos();
        let y = radius * theta.sin();
        let pos = center + Vec2::new(x, y);
        if let Some(prev_pos) = prev { gizmos.line_2d(prev_pos, pos, Color::GREEN); }
        prev = Some(pos);
    }

    // Planets and colors
    let planets = [
        ("Jupiter", JUPITER_COLOR, 7.0),
        ("Saturn", SATURN_COLOR, 6.0),
        ("Uranus", URANUS_COLOR, 5.0),
        ("Neptune", NEPTUNE_COLOR, 5.0),
        ("Pluto", PLUTO_COLOR, 3.0),
    ];

    let bodies = game_state.solar_system.get_all_bodies();
    for (name, col, size) in planets.iter() {
        if let Some(body) = bodies.get(*name) {
            if let Some(ref orbital_state) = body.orbital_state {
                // orbit ellipse
                let a_au = (orbital_state.parameters.semi_major_axis / 149_597_870.7) as f32;
                let e = orbital_state.parameters.eccentricity as f32;
                let b_au = a_au * (1.0 - e * e).sqrt();
                let c_au = a_au * e;
                let segs = 256;
                let mut prev = None::<Vec2>;
                for s in 0..=segs {
                    let theta = s as f32 / segs as f32 * 2.0 * PI;
                    let x = (a_au * theta.cos() - c_au) * scale_factor;
                    let y = (b_au * theta.sin()) * scale_factor;
                    let pos = center + Vec2::new(x, y);
                    if let Some(prev_pos) = prev { gizmos.line_2d(prev_pos, pos, ORBITAL_PATH_COLOR); }
                    prev = Some(pos);
                }

                // planet position
                let cart = orbital_state.to_cartesian();
                let x = center.x + ((cart.x as f32) / 149_597_870.7_f32) * scale_factor;
                let y = center.y + ((cart.y as f32) / 149_597_870.7_f32) * scale_factor;
                gizmos.circle_2d(Vec2::new(x, y), *size, *col).segments(16);
            }
        }
    }
}

fn planet_label_system(
    mut query: Query<(&PlanetLabel, &mut Transform, Option<&InnerOnly>, Option<&BeltOnly>, Option<&OuterOnly>)>,
    game_state: Res<game_state::GameState>,
    view_mode: Res<ViewMode>,
    windows_q: Query<&Window, With<PrimaryWindow>>,
) {
    let center = Vec2::ZERO;
    let bodies = game_state.solar_system.get_all_bodies();

    let window = windows_q.single();
    let scale_factor = match *view_mode {
        ViewMode::Inner => 200.0,
        ViewMode::Belt => 120.0,
        ViewMode::Outer => compute_outer_scale(&game_state, window),
    };

    for (label, mut transform, inner_tag, belt_tag, outer_tag) in &mut query {
        // Skip labels not relevant for current view
        match *view_mode {
            ViewMode::Inner => {
                if belt_tag.is_some() || outer_tag.is_some() { continue; }
            }
            ViewMode::Belt => {
                if inner_tag.is_some() || outer_tag.is_some() { continue; }
            }
            ViewMode::Outer => {
                if inner_tag.is_some() || belt_tag.is_some() { continue; }
            }
        }

        // Special-case: Asteroid Belt label – place it just below the green belt ring so it stays
        // visually connected to the belt in *all* views (inner, belt, outer).  We compute the belt
        // radius in world units using the current `scale_factor` and offset the text downward by
        // the standard `LABEL_OFFSET` amount.
        if label.0 == "Asteroid Belt" {
            let radius_au = 2.77_f32;
            let x = center.x;
            let y = center.y - radius_au * scale_factor + LABEL_OFFSET.y;
            transform.translation = Vec3::new(x, y, 1.0);
            continue;
        }

        if let Some(body) = bodies.get(&label.0[..]) {
            if let Some(ref orbital_state) = body.orbital_state {
                let cart = orbital_state.to_cartesian();
                let x = center.x + ((cart.x as f32) / 149_597_870.7_f32) * scale_factor;
                let y = center.y + ((cart.y as f32) / 149_597_870.7_f32) * scale_factor;
                transform.translation = Vec3::new(x + LABEL_OFFSET.x, y + LABEL_OFFSET.y, 1.0);
            }
        }
    }
}

/// Generate a 2-D annulus mesh (triangle strip) between inner and outer radius.
fn generate_annulus_mesh(inner_radius: f32, outer_radius: f32, segments: usize) -> Mesh {
    let mut positions: Vec<[f32; 3]> = Vec::with_capacity((segments + 1) * 2);
    let mut uvs: Vec<[f32; 2]> = Vec::with_capacity((segments + 1) * 2);
    let mut normals: Vec<[f32; 3]> = Vec::with_capacity((segments + 1) * 2);

    use std::f32::consts::TAU;
    for i in 0..=segments {
        let theta = i as f32 / segments as f32 * TAU;
        let (sin_t, cos_t) = theta.sin_cos();

        // outer vertex
        positions.push([outer_radius * cos_t, outer_radius * sin_t, 0.0]);
        uvs.push([1.0, 0.0]);
        normals.push([0.0, 0.0, 1.0]);

        // inner vertex
        positions.push([inner_radius * cos_t, inner_radius * sin_t, 0.0]);
        uvs.push([0.0, 1.0]);
        normals.push([0.0, 0.0, 1.0]);
    }

    let mut indices: Vec<u32> = Vec::with_capacity((segments) * 6);
    for i in 0..segments as u32 {
        let start = i * 2;
        indices.extend_from_slice(&[start, start + 1, start + 2, start + 2, start + 1, start + 3]);
    }

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh.set_indices(Some(Indices::U32(indices)));
    mesh
}

fn update_visibility_system(
    view_mode: Res<ViewMode>,
    mut inner_q: Query<&mut Visibility, (With<InnerOnly>, Without<BeltOnly>, Without<OuterOnly>)>,
    mut belt_q: Query<&mut Visibility, (With<BeltOnly>, Without<InnerOnly>, Without<OuterOnly>)>,
    mut outer_q: Query<&mut Visibility, (With<OuterOnly>, Without<InnerOnly>, Without<BeltOnly>)>,
) {
    let inner_visible = *view_mode == ViewMode::Inner;
    let belt_visible = *view_mode == ViewMode::Belt;
    let outer_visible = *view_mode == ViewMode::Outer;

    for mut vis in &mut inner_q { *vis = if inner_visible { Visibility::Visible } else { Visibility::Hidden }; }
    for mut vis in &mut belt_q { *vis = if belt_visible { Visibility::Visible } else { Visibility::Hidden }; }
    for mut vis in &mut outer_q { *vis = if outer_visible { Visibility::Visible } else { Visibility::Hidden }; }
}
