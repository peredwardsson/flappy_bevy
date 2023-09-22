use bevy::prelude::*;

#[derive(Default, Component)]
pub struct Player;

#[derive(Component, Default)]
struct VerticalPosition(f32);

pub fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    let pc_handle: Handle<Image> = asset_server.load("redbird-upflap.png");

    commands.spawn(
        (
            Player,
            SpriteBundle {
                texture: pc_handle,
                transform: Transform::from_xyz(0., 0., 0.),
                ..default()
            }
        )
    );
}