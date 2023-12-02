use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

use bevy_xpbd_2d::prelude::*;

// wall constants
const WALL_COLOR: Color = Color::rgb(0.8, 0.8, 0.8);
const WALL_THICKNESS: f32 = 10.0;
// x coordinates
const LEFT_WALL: f32 = -180.;
const RIGHT_WALL: f32 = 180.;
// y coordinates
const BOTTOM_WALL: f32 = -250.;
const TOP_WALL: f32 = 250.;
// Ball constants
const BALL_STARTING_POSITION: Vec3 = Vec3::new(0.0, -50.0, 1.0);
const BALL_SIZE: Vec3 = Vec3::new(30.0, 30.0, 0.0);
const BALL_COLOR: Color = Color::rgb(1.0, 0.5, 0.5);

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            PhysicsPlugins::default(),
            PhysicsDebugPlugin::default(),
        ))
        .add_event::<CollisionEvent>()
        .add_systems(Startup, setup)
        .add_systems(FixedUpdate, (spawn_ball).chain())
        .insert_resource(PhysicsDebugConfig {
            aabb_color: Some(Color::RED),
            ..default()
        })
        .run();
}

#[derive(Component)]
struct Ball;

#[derive(Event, Default)]
struct CollisionEvent;

// This bundle is a collection of the components that define a "wall" in our game
#[derive(Bundle)]
struct WallBundle {
    // You can nest bundles inside of other bundles like this
    // Allowing you to compose their functionality
    sprite_bundle: SpriteBundle,
    collider: Collider,
    rigid_body: RigidBody,
}

/// Which side of the arena is this wall located on?
enum WallLocation {
    Left,
    Right,
    Bottom,
    Top,
}

impl WallLocation {
    fn position(&self) -> Vec2 {
        match self {
            WallLocation::Left => Vec2::new(LEFT_WALL, 0.),
            WallLocation::Right => Vec2::new(RIGHT_WALL, 0.),
            WallLocation::Bottom => Vec2::new(0., BOTTOM_WALL),
            WallLocation::Top => Vec2::new(0., TOP_WALL),
        }
    }

    fn size(&self) -> Vec2 {
        let arena_height = TOP_WALL - BOTTOM_WALL;
        let arena_width = RIGHT_WALL - LEFT_WALL;
        // Make sure we haven't messed up our constants
        assert!(arena_height > 0.0);
        assert!(arena_width > 0.0);

        match self {
            WallLocation::Left | WallLocation::Right => {
                Vec2::new(WALL_THICKNESS, arena_height + WALL_THICKNESS)
            }
            WallLocation::Bottom | WallLocation::Top => {
                Vec2::new(arena_width + WALL_THICKNESS, WALL_THICKNESS)
            }
        }
    }
}

impl WallBundle {
    // This "builder method" allows us to reuse logic across our wall entities,
    // making our code easier to read and less prone to bugs when we change the logic
    fn new(location: WallLocation) -> WallBundle {
        WallBundle {
            sprite_bundle: SpriteBundle {
                transform: Transform {
                    translation: location.position().extend(0.0),
                    scale: location.size().extend(1.0),
                    ..default()
                },
                sprite: Sprite {
                    color: WALL_COLOR,
                    ..default()
                },

                ..default()
            },
            collider: Collider::cuboid(1.0, 1.0),
            rigid_body: RigidBody::Static,
        }
    }
}

// Add the game's entities to our world
fn setup(
    mut commands: Commands,
    //mut meshes: ResMut<Assets<Mesh>>,
    //mut materials: ResMut<Assets<ColorMaterial>>,
    //asset_server: Res<AssetServer>,
) {
    // Camera
    commands.spawn(Camera2dBundle::default());

    // Walls
    commands.spawn(WallBundle::new(WallLocation::Left));
    commands.spawn(WallBundle::new(WallLocation::Right));
    commands.spawn(WallBundle::new(WallLocation::Bottom));

    commands.spawn((
        RigidBody::Dynamic,
        Collider::ball(0.5),
        Transform::from_xyz(0.0, 2.0, 0.0),
    ));
}

fn spawn_ball(
    keyboard_input: Res<Input<KeyCode>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    if !keyboard_input.pressed(KeyCode::Space) {
        return;
    }
    // Ball
    // This ball is not colliding with the walls, but the one in the startup is, adding the materialmesh2dbundle make it not work
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::default().into()).into(),
            material: materials.add(ColorMaterial::from(BALL_COLOR)),
            transform: Transform::from_translation(BALL_STARTING_POSITION).with_scale(BALL_SIZE),
            ..default()
        },
        RigidBody::Dynamic,
        Collider::ball(0.5),
        Ball,
    ));
}

/*fn check_for_collisions(
    mut ball_query: Query<(&mut Velocity, &Transform), With<Ball>>,
    collider_query: Query<(Entity, &Transform), With<Collider>>,
    mut collision_events: EventWriter<CollisionEvent>,
) {
    // check for all balls, not single one
    ball_query.for_each_mut(|(mut ball_velocity, ball_transform)| {
        let ball_size = ball_transform.scale.truncate();

        // check collision with walls
    });
}
*/
