use bevy::{
    diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin},
    prelude::*,
};
use bevy_editor_pls::EditorPlugin;
use bevy_rapier3d::prelude::*;

const X_RANGE: isize = 2;
const Z_RANGE: isize = 2;
const HEIGHT: usize = 1000;
const FLOOR_SIZE: f32 = 50.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(EditorPlugin)
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_startup_system(setup_graphics)
        .add_startup_system(setup_physics)
        .add_system(counter_system)
        .run();
}

#[derive(Component)]
struct StatsText;

fn setup_graphics(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-12.0, 12.0, 40.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });

    let text_section = move |color, value: &str| {
        TextSection::new(
            value,
            TextStyle {
                font: asset_server.load("fonts/Roboto-Regular.ttf"),
                font_size: 40.0,
                color,
            },
        )
    };

    commands.spawn((
        TextBundle::from_sections([
            text_section(Color::GREEN, "Cubes Count: "),
            text_section(Color::CYAN, ""),
            text_section(Color::GREEN, "\nFPS (raw): "),
            text_section(Color::CYAN, ""),
            text_section(Color::GREEN, "\nFPS (SMA): "),
            text_section(Color::CYAN, ""),
            text_section(Color::GREEN, "\nFPS (EMA): "),
            text_section(Color::CYAN, ""),
        ])
        .with_style(Style {
            position_type: PositionType::Absolute,
            position: UiRect {
                top: Val::Px(25.0),
                left: Val::Px(5.0),
                ..default()
            },
            ..default()
        }),
        StatsText,
    ));
}

fn setup_physics(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    spawn_floor(&mut commands, &mut meshes, &mut materials);
    spawn_cubes(&mut commands, &mut meshes, &mut materials);
}

fn spawn_floor(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(shape::Plane::from_size(FLOOR_SIZE).into()),
            material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
            transform: Transform::from_xyz(0.0, -1.0, 0.0),
            ..Default::default()
        },
        Collider::cuboid(FLOOR_SIZE / 2.0, 0.1, FLOOR_SIZE / 2.0),
    ));
}

fn spawn_cubes(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    let cube_mesh = meshes.add(shape::Cube { size: 1.0 }.into());
    let cube_material = materials.add(Color::rgb(0.8, 0.7, 0.6).into());

    let cubes = (-X_RANGE..X_RANGE)
        .flat_map(|x| (-Z_RANGE..Z_RANGE).flat_map(move |z| (0..HEIGHT).map(move |y| (x, y, z))))
        .map(move |(x, y, z)| {
            (
                PbrBundle {
                    mesh: cube_mesh.clone(),
                    material: cube_material.clone(),
                    transform: Transform::from_xyz(x as f32, y as f32, z as f32),
                    ..Default::default()
                },
                Collider::cuboid(0.5, 0.5, 0.5),
                RigidBody::Dynamic,
            )
        });

    commands.spawn_batch(cubes)
}

fn counter_system(
    diagnostics: Res<Diagnostics>,
    cubes: Query<&RigidBody>,
    mut query: Query<&mut Text, With<StatsText>>,
) {
    let mut text = query.single_mut();

    let cubes_count = cubes.iter().len();
    text.sections[1].value = cubes_count.to_string();

    if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
        if let Some(raw) = fps.value() {
            text.sections[3].value = format!("{raw:.2}");
        }
        if let Some(sma) = fps.average() {
            text.sections[5].value = format!("{sma:.2}");
        }
        if let Some(ema) = fps.smoothed() {
            text.sections[7].value = format!("{ema:.2}");
        }
    };
}
