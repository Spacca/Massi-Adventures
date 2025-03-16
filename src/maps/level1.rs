
//Create a Bevy plugin to represent the levle1 map
use bevy::prelude::*;
use avian2d::{math::*, prelude::*};
use bevy::render::{render_asset::RenderAssetUsages, render_resource::PrimitiveTopology};

//create a bevy system that can be added to the app to instantiate the map
pub fn setup(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) {
        // A cube to move around
        commands.spawn((
            Sprite {
                color: Color::srgb(0.0, 0.4, 0.7),
                custom_size: Some(Vec2::new(30.0, 30.0)),
                ..default()
            },
            Transform::from_xyz(50.0, -100.0, 0.0),
            RigidBody::Dynamic,
            Collider::rectangle(30.0, 30.0),
        ));
    
        // Platforms
        commands.spawn((
            Sprite {
                color: Color::srgb(0.7, 0.7, 0.8),
                custom_size: Some(Vec2::new(1100.0, 50.0)),
                ..default()
            },
            Transform::from_xyz(0.0, -175.0, 0.0),
            RigidBody::Static,
            Collider::rectangle(1100.0, 50.0),
        ));
        commands.spawn((
            Sprite {
                color: Color::srgb(0.7, 0.7, 0.8),
                custom_size: Some(Vec2::new(300.0, 25.0)),
                ..default()
            },
            Transform::from_xyz(175.0, -35.0, 0.0),
            RigidBody::Static,
            Collider::rectangle(300.0, 25.0),
        ));
        commands.spawn((
            Sprite {
                color: Color::srgb(0.7, 0.7, 0.8),
                custom_size: Some(Vec2::new(300.0, 25.0)),
                ..default()
            },
            Transform::from_xyz(-175.0, 0.0, 0.0),
            RigidBody::Static,
            Collider::rectangle(300.0, 25.0),
        ));
        commands.spawn((
            Sprite {
                color: Color::srgb(0.7, 0.7, 0.8),
                custom_size: Some(Vec2::new(150.0, 80.0)),
                ..default()
            },
            Transform::from_xyz(475.0, -110.0, 0.0),
            RigidBody::Static,
            Collider::rectangle(150.0, 80.0),
        ));
        commands.spawn((
            Sprite {
                color: Color::srgb(0.7, 0.7, 0.8),
                custom_size: Some(Vec2::new(150.0, 80.0)),
                ..default()
            },
            Transform::from_xyz(-475.0, -110.0, 0.0),
            RigidBody::Static,
            Collider::rectangle(150.0, 80.0),
        ));
    
        // Ramps
    
        let mut ramp_mesh = Mesh::new(
            PrimitiveTopology::TriangleList,
            RenderAssetUsages::default(),
        );
    
        ramp_mesh.insert_attribute(
            Mesh::ATTRIBUTE_POSITION,
            vec![[-125.0, 80.0, 0.0], [-125.0, 0.0, 0.0], [125.0, 0.0, 0.0]],
        );
    
        let ramp_collider = Collider::triangle(
            Vector::new(-125.0, 80.0),
            Vector::NEG_X * 125.0,
            Vector::X * 125.0,
        );
    
        commands.spawn((
            Mesh2d(meshes.add(ramp_mesh)),
            MeshMaterial2d(materials.add(Color::srgb(0.4, 0.4, 0.5))),
            Transform::from_xyz(-275.0, -150.0, 0.0),
            RigidBody::Static,
            ramp_collider,
        ));
    
        let mut ramp_mesh = Mesh::new(
            PrimitiveTopology::TriangleList,
            RenderAssetUsages::default(),
        );
    
        ramp_mesh.insert_attribute(
            Mesh::ATTRIBUTE_POSITION,
            vec![[20.0, -40.0, 0.0], [20.0, 40.0, 0.0], [-20.0, -40.0, 0.0]],
        );
    
        let ramp_collider = Collider::triangle(
            Vector::new(20.0, -40.0),
            Vector::new(20.0, 40.0),
            Vector::new(-20.0, -40.0),
        );
    
        commands.spawn((
            Mesh2d(meshes.add(ramp_mesh)),
            MeshMaterial2d(materials.add(Color::srgb(0.4, 0.4, 0.5))),
            Transform::from_xyz(380.0, -110.0, 0.0),
            RigidBody::Static,
            ramp_collider,
        ));
}