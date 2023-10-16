use bevy::app::App;
use bevy::prelude::*;
use loading::TextureAssets;

mod actions;
mod loading;
mod player;

use crate::actions::ActionsPlugin;
use crate::loading::LoadingPlugin;
use crate::player::PlayerPlugin;

#[cfg(debug_assertions)]
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};

#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    #[default]
    Loading,
    Playing,
}

pub struct GamePlugin;

#[derive(Component)]
pub struct GameCamera;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<GameState>()
            .add_plugins((LoadingPlugin, PlayerPlugin, ActionsPlugin))
            .add_systems(OnEnter(GameState::Playing), (spawn_camera, spawn_pev));

        #[cfg(debug_assertions)]
        {
            app.add_plugins((FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin::default()));
        }
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default()).insert(GameCamera);
}

#[derive(Component)]
pub struct PlanetEvacuationVessel;

fn spawn_pev(mut commands: Commands, textures: Res<TextureAssets>) {
    commands
        .spawn(SpriteBundle {
            texture: textures.texture_pev.clone(),
            transform: Transform::from_translation(Vec3::new(40., 1000., 0.)),
            ..Default::default()
        })
        .insert(PlanetEvacuationVessel);
}
