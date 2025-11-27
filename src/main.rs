use avian2d::prelude::*;
use bevy::prelude::*;

const PIXELS_PER_METER: f32 = 200.0;
const ARENA_WIDTH_M: f32 = 3.0;
const ARENA_HEIGHT_M: f32 = 2.0;
const ROBOT_SIZE_M: f32 = 0.3 * PIXELS_PER_METER;
const ROBOT_SPEED_M: f32 = 2.0;

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
    commands.spawn(Camera2d::default());

    // Le fond d'après le fichier donné par le règlement
    commands.spawn((
        Sprite {
            image: asset_server.load("table.png"),
            custom_size: Some(Vec2::new(
                ARENA_WIDTH_M * PIXELS_PER_METER,
                ARENA_HEIGHT_M * PIXELS_PER_METER,
            )),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, -1.0), // En arrière plan (Z = -1)
    ));

    // Les Murs autour de l'arène
    spawn_arena_boundaries(&mut commands);

    // Le Robot et sa physique
    commands.spawn((
        // En vert pour l'instant
        Sprite {
            color: COLOR_ROBOT,
            custom_size: Some(Vec2::new(ROBOT_SIZE_M, ROBOT_SIZE_M)),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 1.0),
        Robot,
        Speed(ROBOT_SPEED_M),
        // Composants Physiques (Avian)
        RigidBody::Dynamic,
        Collider::rectangle(ROBOT_SIZE_M, ROBOT_SIZE_M), // Il a une forme physique carré
        LockedAxes::ROTATION_LOCKED, // TODO : permettre au robot de tourner
    ));
}

fn spawn_arena_boundaries(commands: &mut Commands) {
    let thickness = 0.5;

    // Conversion en pixels pour la taille visuelle/physique
    let w_pixels = ARENA_WIDTH_M * PIXELS_PER_METER;
    let h_pixels = ARENA_HEIGHT_M * PIXELS_PER_METER;
    let thick_pixels = thickness * PIXELS_PER_METER;

    // Les positions sont calculées par rapport au centre (0,0)
    // TODO: changer pour avoir la même grille que le règlement
    
    // Mur du Haut
    spawn_wall(
        commands,
        Vec2::new(0.0, h_pixels / 2.0 + thick_pixels / 2.0),
        Vec2::new(w_pixels + 2.0 * thick_pixels, thick_pixels),
    );

    // Mur du Bas
    spawn_wall(
        commands,
        Vec2::new(0.0, -(h_pixels / 2.0 + thick_pixels / 2.0)),
        Vec2::new(w_pixels + 2.0 * thick_pixels, thick_pixels),
    );

    // Mur Gauche
    spawn_wall(
        commands,
        Vec2::new(-(w_pixels / 2.0 + thick_pixels / 2.0), 0.0),
        Vec2::new(thick_pixels, h_pixels),
    );

    // Mur Droit
    spawn_wall(
        commands,
        Vec2::new(w_pixels / 2.0 + thick_pixels / 2.0, 0.0),
        Vec2::new(thick_pixels, h_pixels),
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
            velocity.0 = direction * speed.0 * PIXELS_PER_METER;
        } else {
            // TODO : ajouter des charactéristiques de freinange du robot
            velocity.0 = Vec2::ZERO;
        }
    }
}
