use bevy::app::App;
use bevy::prelude::*;
use scenario1::Scenario1Plugin;

mod actions;
mod loading;
mod player;
mod scenario1;

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
            .add_plugins(Scenario1Plugin)
            .add_systems(OnEnter(GameState::Playing), spawn_camera);

        #[cfg(debug_assertions)]
        {
            app.add_plugins((FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin::default()));
        }
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default()).insert(GameCamera);
}
