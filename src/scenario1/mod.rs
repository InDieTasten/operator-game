mod components;

use bevy::prelude::*;

use crate::{loading::TextureAssets, GameState};

use self::components::PlanetEvacuationVessel;

pub struct Scenario1Plugin;

impl Plugin for Scenario1Plugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), spawn_pev);
    }
}

fn spawn_pev(mut commands: Commands, textures: Res<TextureAssets>) {
    commands
        .spawn(SpriteBundle {
            texture: textures.texture_pev.clone(),
            transform: Transform::from_translation(Vec3::new(40., 1000., 0.)),
            ..Default::default()
        })
        .insert(PlanetEvacuationVessel);
}
