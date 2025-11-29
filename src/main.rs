use avian2d::prelude::*;
use bevy::{camera::ScalingMode, prelude::*};

const ARENA_WIDTH_MM: f32 = 3000.0;
const ARENA_HEIGHT_MM: f32 = 2000.0;
const ROBOT_SIZE_MM: f32 = 300.0;
const ROBOT_SPEED_MM_S: f32 = 2000.0;

const COLOR_ROBOT: Color = Color::srgb(0.2, 0.8, 0.2);

#[derive(Component)]
struct Robot;

#[derive(Component)]
struct Speed(f32);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Simulation Robot 2D".into(),
                resolution: (800, 600).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(PhysicsPlugins::default())
        .insert_resource(Gravity(Vec2::ZERO))
        .add_systems(Startup, setup)
        .add_systems(Update, move_robot)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Camera2d::default(),
        Transform::from_xyz(ARENA_WIDTH_MM / 2.0, ARENA_HEIGHT_MM / 2.0, 0.0),
    ));


    // Le fond d'après le fichier donné par le règlement
    commands.spawn((
        Sprite {
            image: asset_server.load("table.png"),
            custom_size: Some(Vec2::new(ARENA_WIDTH_MM, ARENA_HEIGHT_MM)),
            ..default()
        },
        Transform::from_xyz(ARENA_WIDTH_MM / 2.0, ARENA_HEIGHT_MM / 2.0, -1.0), // En arrière plan (Z = -1)
    ));

    // Les Murs autour de l'arène
    spawn_arena_boundaries(&mut commands);

    // Le Robot et sa physique
    commands.spawn((
        // En vert pour l'instant
        Sprite {
            color: COLOR_ROBOT,
            custom_size: Some(Vec2::new(ROBOT_SIZE_MM, ROBOT_SIZE_MM)),
            ..default()
        },
        Transform::from_xyz(ARENA_WIDTH_MM / 2.0, ARENA_HEIGHT_MM / 2.0, 1.0),
        Robot,
        Speed(ROBOT_SPEED_MM_S),
        // Composants Physiques (Avian)
        RigidBody::Dynamic,
        Collider::rectangle(ROBOT_SIZE_MM, ROBOT_SIZE_MM), // Il a une forme physique carré
        LockedAxes::ROTATION_LOCKED,                       // TODO : permettre au robot de tourner
    ));
}

fn spawn_arena_boundaries(commands: &mut Commands) {
    let wall_thickness = 50.0; // épaisseur des murs en mm

    // Mur du Bas
    spawn_wall(
        commands,
        Vec2::new(ARENA_WIDTH_MM / 2.0, -wall_thickness / 2.0),
        Vec2::new(ARENA_WIDTH_MM, wall_thickness),
    );

    // Mur du Haut
    spawn_wall(
        commands,
        Vec2::new(ARENA_WIDTH_MM / 2.0, ARENA_HEIGHT_MM + wall_thickness / 2.0),
        Vec2::new(ARENA_WIDTH_MM, wall_thickness),
    );

    // Mur Gauche
    spawn_wall(
        commands,
        Vec2::new(-wall_thickness / 2.0, ARENA_HEIGHT_MM / 2.0),
        Vec2::new(wall_thickness, ARENA_HEIGHT_MM),
    );

    // Mur Droit
    spawn_wall(
        commands,
        Vec2::new(ARENA_WIDTH_MM + wall_thickness / 2.0, ARENA_HEIGHT_MM / 2.0),
        Vec2::new(wall_thickness, ARENA_HEIGHT_MM),
    );
}

// La fonction utilitaire de tout à l'heure
fn spawn_wall(commands: &mut Commands, position: Vec2, size: Vec2) {
    commands.spawn((
        // Pas de texture pour l'instant
        Sprite {
            color: Color::srgb(1.0, 0.0, 0.0),
            custom_size: Some(size),
            ..default()
        },
        Transform::from_translation(position.extend(0.0)),
        RigidBody::Static,
        Collider::rectangle(size.x, size.y),
    ));
}

fn move_robot(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut LinearVelocity, &Speed), With<Robot>>,
) {
    if let Ok((mut velocity, speed)) = query.single_mut() {
        let mut direction = Vec2::ZERO;

        if keyboard_input.pressed(KeyCode::ArrowUp) {
            direction.y += 1.0;
        }
        if keyboard_input.pressed(KeyCode::ArrowDown) {
            direction.y -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::ArrowRight) {
            direction.x += 1.0;
        }
        if keyboard_input.pressed(KeyCode::ArrowLeft) {
            direction.x -= 1.0;
        }

        if direction.length() > 0.0 {
            direction = direction.normalize();
            velocity.0 = direction * speed.0;
        } else {
            // TODO : ajouter des charactéristiques de freinange du robot
            velocity.0 = Vec2::ZERO;
        }
    }
}
