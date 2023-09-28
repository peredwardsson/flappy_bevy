mod assetmgmt;

use assetmgmt::{check_assets_ready, setup_assets, FlappyAssets, AssetLoading};
use bevy::{prelude::*, sprite::collide_aabb::collide};

const BIRD_SIZE: Vec3 = Vec3::new(34., 24., 0.);
const PIPE_SIZE: Vec3 = Vec3::new(52., 320., 0.);

const X_SPEED: f32 = 100.0;
const PIPE_GAP: f32 = 140.0;
const PIPE_SPAWN_X: u32 = 155;

#[derive(Clone, Resource, Copy, Default, Debug, Hash, States, PartialEq, Eq)]
pub enum GameState {
    #[default]
    Loading,
    Menu,
    Game,
}

#[derive(Component, Default)]
pub struct Bird;

#[derive(Component)]
pub struct Pipe;

#[derive(Component)]
struct Base;

#[derive(Component, Default)]
pub struct Collider;

#[derive(Bundle, Default)]
struct BirdCollider {
    bird: Bird,
    collider: Collider,
    sprite: SpriteBundle
}


#[derive(Component)]
struct IsGravity;

#[derive(Resource)]
struct Gravity(f32);

#[derive(Component)]
struct VerticalVelocity(f32);

#[derive(Resource)]
pub struct Score(u32);

pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut score: ResMut<Score>,
) {
    println!("Starting up!");

    let pc_handle: Handle<Image> = asset_server.load("./sprites/redbird-downflap.png");
    let bg_handle: Handle<Image> = asset_server.load("./sprites/background-day.png");
    commands.spawn(SpriteBundle{
        texture: bg_handle,
        transform: Transform::from_xyz(0., 0., -1.0),
        ..default()
    });
    commands.spawn(Camera2dBundle::default());
    commands.spawn(
        (BirdCollider {
            sprite: SpriteBundle {
                texture: pc_handle,
                ..default()
            },
            ..default()
        },
        VerticalVelocity(0.0),
        IsGravity
        )
    );
    commands.spawn(
        TextBundle::from_section(
            score.0.to_string(),
            TextStyle {
                font_size: 80.0,
                color: Color::Rgba { red: 1.0, green: 0.0, blue: 0.0, alpha: 1.0 },
                ..default()
            },
        )
    );

    score.0 = 0;

}

fn apply_gravity(
    // mut tf: Query<&mut Transform, With<Bird, IsGravity>>
    mut tf: Query<(&mut Transform, &mut VerticalVelocity, &IsGravity)>,
    gravity: Res<Gravity>,
    time: Res<Time>
) {
    for (mut r, mut vel, _) in &mut tf {
        vel.0 += gravity.0 * time.delta().as_secs_f32();
        r.translation.y -= vel.0;
    }
}

fn jump(
    keyboard_event: Res<Input<KeyCode>>,
    mut tf: Query<(&mut VerticalVelocity, With<IsGravity>)>,
    jump_amount: Res<JumpAmount>,
) {
    if keyboard_event.just_pressed(KeyCode::Space) {
        for (mut v, _) in &mut tf {
            v.0 = -jump_amount.0;
        }
    }
}

fn shift_pipes(
    mut query: Query<&mut Transform, With<Pipe>>,
    time: Res<Time>
) {
    let dt = time.delta_seconds();
    for mut tf in &mut query {
        tf.translation.x -= X_SPEED * dt;
    }
}

#[derive(Resource)]
struct PipeSpawnTimer {
    timer: Timer,
}

fn spawn_pipe_on_timer(
    mut commands: Commands,
    time: Res<Time>,
    asset_server: Res<AssetServer>,
    mut pipe_timer: ResMut<PipeSpawnTimer>
) {
    // this function is really slow.
    pipe_timer.timer.tick(time.delta());

    if pipe_timer.timer.just_finished() {
        println!("Spawning new pipe!");
        let pipe_handle: Handle<Image> = asset_server.load("sprites/pipe-green.png");
        let y_center = 30.0;

        let upper_offset = y_center - PIPE_GAP / 2.0;
        let lower_offset = y_center + PIPE_GAP / 2.0;
        commands.spawn(
            (
                SpriteBundle{
                    texture: pipe_handle.clone(),
                    transform: Transform::from_xyz(PIPE_SPAWN_X as f32, upper_offset - PIPE_SIZE[1], 0.0),
                    ..default()
                },
                Pipe,
                Collider
            )
        );
        commands.spawn(
            (
                SpriteBundle{
                    texture: pipe_handle,
                    transform: Transform::from_xyz(PIPE_SPAWN_X as f32, lower_offset, 0.0),
                    sprite: Sprite{
                        flip_y: true,
                        ..default()
                    },
                    ..default()
                },
                Pipe,
                Collider
            ),
        );
    }
}

pub fn check_pipe_update_score(
    pipe_query: Query<&Transform, With<Pipe>>,
    mut score: ResMut<Score>,
    mut score_text: Query<&mut Text>,
) {
    let mut pipes_passed = 0;
    for tf in &pipe_query {
        if tf.translation.x + PIPE_SIZE[0]/2. < 0.0 {
            // commands.entity(pipe_entity).despawn();
            pipes_passed += 1;
        }
    }
    score.0 = pipes_passed/2;
    score_text.single_mut().sections[0].value = score.0.to_string();
}

#[derive(Resource)]
struct JumpAmount(f32);

#[derive(Event, Default)]
pub struct GameOverEvent;

#[derive(Event, Default)]
pub struct NewGameEvent;


pub fn check_for_collisions(
    bird_query: Query<&Transform, With<Bird>>,
    colliders: Query<&Transform, (With<Collider>, With<Pipe>)>
) {
    let bird_tf = match bird_query.get_single() {
        Ok(t) => t,
        Err(e) => {
            println!("No bird, error {:?}", e);
            return;
        }
    };
    for tf in &colliders {
        let collision = collide(
            bird_tf.translation,
            BIRD_SIZE.truncate(),
            tf.translation,
            PIPE_SIZE.truncate(),
        );
        if let Some(t) = collision {
            println!("Collision! {:?}", t);
        }
    }
}

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Flappy Bird".into(),
                    resolution: (288., 512.).into(),
                    ..default()
                }),
                ..Default::default()
            }))
        .add_state::<GameState>()
        .add_plugins(
            (load_state::LoadScreen, game::Game)
        )
        .insert_resource(FlappyAssets::default())
        .insert_resource(AssetLoading::default())
        .insert_resource(JumpAmount(3.0))
        .insert_resource(Gravity(6.82))
        .run();

}

mod load_state {
    use bevy::prelude::*;
    use super::{GameState, despawn_screen, setup_assets, check_assets_ready};

    pub struct LoadScreen;

    #[derive(Component)]
    pub struct OnLoadScreen;

    impl Plugin for LoadScreen {
        fn build(&self, app: &mut App) {
            app
                .add_systems(OnEnter(GameState::Loading), setup_assets)
                .add_systems(Update, check_assets_ready.run_if(in_state(GameState::Loading)))
                .add_systems(OnExit(GameState::Loading), despawn_screen::<OnLoadScreen>);
        }
    }
}

mod game {
    use bevy::prelude::*;
    use super::*;

    pub struct Game;

    #[derive(Component)]
    pub struct OnGame;

    impl Plugin for Game {
        fn build(&self, app: &mut App) {
            app
                .insert_resource(
                    PipeSpawnTimer{
                        timer: Timer::from_seconds(
                            1.0,
                            TimerMode::Repeating
                        )
                    }
                )
                .add_systems(
                    OnEnter(GameState::Game), (setup)
                )
                .add_systems(
                    FixedUpdate,
                    (
                        apply_gravity,
                        shift_pipes,
                        spawn_pipe_on_timer.after((jump)),
                        check_for_collisions,
                    ).run_if(in_state(GameState::Game))
                );


        }
    }

}
// Generic system that takes a component as a parameter, and will despawn all entities with that component
fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}