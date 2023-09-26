use bevy::prelude::*;

use crate::{GameLoadState, LoadState};

#[derive(Resource)]
pub struct AssetLoading(pub Vec<HandleUntyped>);

#[derive(Resource, Default)]
pub struct FlappyAssets {
    pub base: Option<Handle<Image>>,
    pub background: Option<Handle<Image>>,
    pub pipe: Option<Handle<Image>>,
    pub pc: Option<Handle<Image>>,
}

pub fn setup_assets(
    asset_server: Res<AssetServer>,
    mut loading: ResMut<AssetLoading>,
    mut flappy_assets: ResMut<FlappyAssets>,
) {
    let base_handle: Handle<Image> = asset_server.load("sprites/base.png");
    let bg_handle: Handle<Image> = asset_server.load("sprites/background-day.png");
    let pipe_handle: Handle<Image> = asset_server.load("sprites/pipe-green.png");
    let pc_handle: Handle<Image> = asset_server.load("sprites/redbird-upflap.png");
    flappy_assets.base = Some(base_handle.clone());
    flappy_assets.background = Some(bg_handle.clone());
    flappy_assets.pipe = Some(pipe_handle.clone());
    flappy_assets.pc = Some(pc_handle.clone());
    loading.0.push(base_handle.clone_untyped());
    loading.0.push(bg_handle.clone_untyped());
    loading.0.push(pipe_handle.clone_untyped());
    loading.0.push(pc_handle.clone_untyped());
}

pub fn render(
    mut commands: Commands,
    gamestate: Res<GameLoadState>,
    flappy_assets: Res<FlappyAssets>
) {
    match gamestate.loaded {
        LoadState::Loaded => {
            if let Some(bg_sprite) = flappy_assets.background.clone() {
                commands.spawn(
                    SpriteBundle{
                        texture: bg_sprite,
                        ..default()
                    }
                );
            }

            if let Some(pc_sprite) = flappy_assets.pc.clone() {
                commands.spawn(
                    SpriteBundle{
                        texture: pc_sprite,
                        ..default()
                    }
                );
            }
        },
        _ => {
        }
    }
}

pub fn check_assets_ready(
    mut commands: Commands,
    server: Res<AssetServer>,
    loading: Res<AssetLoading>,
    mut gamestate: ResMut<GameLoadState>
) {
    if gamestate.loaded == LoadState::Loaded {
        return;
    }

    match server.get_group_load_state(loading.0.iter().map(|h| h.id())) {
        bevy::asset::LoadState::Failed => {
            println!("Failed to load one of the assets!!");
        }
        bevy::asset::LoadState::Loaded => {
            println!("All loaded!!");
            *gamestate = GameLoadState{loaded: LoadState::Loaded};
            commands.spawn(Camera2dBundle::default());
        }
        _ => {
            println!("Loading...");
        }
    }
}