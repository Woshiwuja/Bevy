use bevy::{
    prelude::{
        shape::{Cube, Plane},
        *,
    },
    transform,
};
use bevy_flycam::MovementSettings;
use bevy_flycam::PlayerPlugin;
use bevy_inspector_egui::WorldInspectorPlugin;
pub const HEIGHT: f32 = 720.0;
pub const WIDTH: f32 = 1280.0;

pub struct GameAssets {
    golem_scene: Handle<Scene>,
}
//model loading
fn asset_loading(mut commands: Commands, assets: Res<AssetServer>) {
    commands.insert_resource(GameAssets {
        golem_scene: assets.load("untitled.glb#Scene0"),
    });
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn_bundle(Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}
fn spawn_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    golem_asset: Res<GameAssets>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn_bundle(SceneBundle {
        scene: golem_asset.golem_scene.clone(),
        transform: Transform::from_xyz(1.0, 1.0, 1.0),
        ..Default::default()
    });

    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::rgb(1.0, 0.2, 1.0).into()),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
    });
    //plane

    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
        material: materials.add(Color::rgb(0.0, 1.0, 0.0).into()),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
    });

    //light
    commands.spawn_bundle(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 6.0),
        ..default()
    });
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.4, 0.4, 0.4)))
        .insert_resource(WindowDescriptor {
            width: WIDTH,
            height: HEIGHT,
            title: "bevy test".to_string(),
            resizable: false,
            ..Default::default()
        })
        .add_startup_system(spawn_scene)
        .add_startup_system(spawn_camera)
        .add_startup_system(asset_loading)
        .add_plugins(DefaultPlugins)
        .add_plugin(PlayerPlugin)
        .insert_resource(MovementSettings {
            sensitivity: 0.00015,
            speed: 12.0,
        })
        .add_plugin(WorldInspectorPlugin::new())
        .run();
}
