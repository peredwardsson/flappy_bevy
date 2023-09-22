mod assetmgmt;
mod structs;
mod systems;

use bevy::{prelude::*, input::touch, ecs::component};

#[derive(Component, Default)]
struct Bird;

#[derive(Component)]
struct Pipe;

#[derive(Component)]
struct Base;

#[derive(Component, Default)]
struct Collider;

#[derive(Component)]
enum PipeDirection {
    Up,
    Down
}

#[derive(Bundle)]
struct PipeCollider {
    direction: PipeDirection,
    collider: Collider
}

#[derive(Component, Default)]
struct Gap(f32);

#[derive(Component, Default)]
struct YCenter(f32);

#[derive(Component)]
struct PipePair {
    gap: Gap,
    y_center: YCenter
}

impl PipePair {
    fn new(gap: f32, y_center: f32) -> Self {
        return PipePair {
            gap: Gap(gap),
            y_center: YCenter(y_center)
        }
    }
}

#[derive(Bundle, Default)]
struct BirdCollider {
    bird: Bird,
    collider: Collider,
    sprite: SpriteBundle
}


#[derive(PartialEq)]
pub enum LoadState {
    NotLoadded,
    Loaded
}

#[derive(Resource)]
pub struct GameLoadState {
    loaded: LoadState
}

#[derive(Component)]
struct IsGravity;

#[derive(Resource)]
struct Gravity(f32);

#[derive(Component)]
struct VerticalVelocity(f32);

impl Default for Gravity {
    fn default() -> Self {
        Self(0.82)
    }
}

pub fn setup(
    mut commands: Commands,
    assets: Res<AssetServer>
) {
    println!("Starting up!");

    match assets.load_folder(".") {
        Ok(t) => println!("Loaded folder!"),
        Err(e) => panic!("Folder doesn't exist!")
    }
    let bg_handle: Handle<Image> = assets.load("background-day.png");
    let pc_handle: Handle<Image> = assets.load("redbird-downflap.png");

    commands.spawn(Camera2dBundle::default());
    commands.spawn(SpriteBundle{
        texture: bg_handle,
        ..default()
    });

    commands.spawn(
        (BirdCollider {
                sprite: SpriteBundle {
                    texture: pc_handle,
                    ..default()
                },
                ..default()
            },
            VerticalVelocity(0.0),
            IsGravity)

    );
}

fn player_position(
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
    mut tf: Query<(&mut VerticalVelocity, &IsGravity)>,
) {
    if keyboard_event.just_pressed(KeyCode::Space) {
        for (mut v, _) in &mut tf {
            v.0 = -1.0;
        }
    }
}

fn shift_pipes(
    mut query: Query<(&PipeDirection, &mut Transform)>,
    time: Res<Time>
) {
    let dt = time.delta_seconds();
    for (_, mut tf) in &mut query {
        tf.translation.x -= 100.0 * dt;
    }
}

fn debug_pipe(
    keyboard_event: Res<Input<KeyCode>>,
    assets: Res<AssetServer>,
    a: Res<Assets<Image>>,
    mut commands: Commands,
) {
    let pipe_handle: Handle<Image> = assets.load("pipe-green.png");
    if keyboard_event.just_pressed(KeyCode::A) {
        match a.get(&pipe_handle) {
            Some(t) => println!("All good!"),
            None => {
                println!("Pipe not loaded yet!");
                return;
            }
        }
        let pipe_height = a.get(&pipe_handle).unwrap().texture_descriptor.size.height as f32;
        let gap = 140.0;
        let y_center = 30.0;
        let pipe_pair = PipePair::new(gap, y_center);
        let x_offset = 125.;

        let upper_offset = y_center - gap / 2.0;
        let lower_offset = y_center + gap / 2.0;
        commands.spawn(
            (
                SpriteBundle{
                    texture: pipe_handle.clone(),
                    transform: Transform::from_xyz(x_offset, upper_offset - pipe_height, 0.0),
                    ..default()
                },
                PipeCollider{
                    direction: PipeDirection::Up,
                    collider: Collider::default(),
                },
            )
        );
        commands.spawn(
            (
                SpriteBundle{
                    texture: pipe_handle,
                    transform: Transform::from_xyz(x_offset, lower_offset, 0.0),
                    sprite: Sprite{
                        flip_y: true,
                        ..default()
                    },
                    ..default()
                },
                PipeCollider{
                    direction: PipeDirection::Down,
                    collider: Collider::default(),
                },
            ),
        );
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
            })
        )
        .insert_resource(Gravity::default())
        .add_systems(
            Startup, (setup)
        )
        .add_systems(
            Update, (player_position, jump, debug_pipe, shift_pipes)
        )
        .run();
}
