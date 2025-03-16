//! A basic implementation of a character controller for a dynamic rigid body.
//!
//! This showcases the following:
//!
//! - Basic directional movement and jumping
//! - Support for both keyboard and gamepad input
//! - A configurable maximum slope angle for jumping
//!
//! The character controller logic is contained within the `plugin` module.
//!
//! For a kinematic character controller, see the `kinematic_character_2d` example.

mod maps;
mod plugin;

use avian2d::{math::*, prelude::*};
use bevy::prelude::*;
use plugin::*;

// A bevy state enum to represent the current level
#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
enum AppState {
    Level1
}

impl AppState {
    fn setup(&self, commands: &mut Commands, meshes: &mut ResMut<Assets<Mesh>>, materials: &mut ResMut<Assets<ColorMaterial>>) {
        match self {
            // match on the enum type and call the setup method of the corresponding map
            AppState::Level1 => maps::level1::setup(commands, meshes, materials),
            // by default render level1
        }
    }
}

#[derive(Event)]
struct SpawnWorld;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            // Add physics plugins and specify a units-per-meter scaling factor, 1 meter = 20 pixels.
            // The unit allows the engine to tune its parameters for the scale of the world, improving stability.
            PhysicsPlugins::default().with_length_unit(20.0),
            CharacterControllerPlugin,
        ))
        .add_event::<SpawnWorld>()
        .insert_state(AppState::Level1)
        .insert_resource(ClearColor(Color::srgb(0.05, 0.05, 0.1)))
        .insert_resource(Gravity(Vector::NEG_Y * 1000.0))
        .add_systems(Startup, send_startup_event)
        .add_systems(Update, handle_spawn_world_events)
        .run();
}

fn handle_spawn_world_events(
    mut commands: Commands,
    mut events: EventReader<SpawnWorld>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    state: Res<State<AppState>>,
) {
    for _event in events.read() {
        println!("Current state when handling SpawnWorld {:?}", state.get());
        setup(&mut commands, &mut meshes, &mut materials);
        state.get().setup(&mut commands, &mut meshes, &mut materials);
    }
}

fn send_startup_event(mut events: EventWriter<SpawnWorld>) {
    events.send(SpawnWorld);
}

fn setup(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) {
    // log the invocation of this method
    println!("Main Setup method invoked");
    // Player
    commands.spawn((
        Mesh2d(meshes.add(Capsule2d::new(12.5, 20.0))),
        MeshMaterial2d(materials.add(Color::srgb(0.2, 0.7, 0.9))),
        Transform::from_xyz(0.0, -100.0, 0.0),
        CharacterControllerBundle::new(Collider::capsule(12.5, 20.0)).with_movement(
            1250.0,
            0.92,
            400.0,
            (30.0 as Scalar).to_radians(),
        ),
        Friction::ZERO.with_combine_rule(CoefficientCombine::Min),
        Restitution::ZERO.with_combine_rule(CoefficientCombine::Min),
        ColliderDensity(2.0),
        GravityScale(1.5),
    ));
    // Camera
    commands.spawn(Camera2d);
}
