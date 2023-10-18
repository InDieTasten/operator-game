mod components;

use bevy::prelude::*;

use crate::{loading::TextureAssets, GameState};

use self::components::{PlanetEvacuationVessel, SurfaceSceneRoot};

pub struct Scenario1Plugin;

impl Plugin for Scenario1Plugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), spawn_scenario);
        app.add_systems(OnExit(GameState::Playing), clear_scenario);
    }
}

fn spawn_scenario(mut commands: Commands, textures: Res<TextureAssets>) {
    commands
        .spawn(SpatialBundle::default())
        .insert(SurfaceSceneRoot)
        .with_children(|parent| {
            parent
                .spawn(SpriteBundle {
                    texture: textures.texture_pev.clone(),
                    transform: Transform::from_translation(Vec3::new(40., 1000., 0.)),
                    ..Default::default()
                })
                .insert(PlanetEvacuationVessel);

            parent
                .spawn(SpriteBundle {
                    texture: textures.texture_pev.clone(),
                    transform: Transform::from_translation(Vec3::new(30., 500., 0.)),
                    ..Default::default()
                })
                .insert(PlanetEvacuationVessel);

            parent
                .spawn(SpriteBundle {
                    texture: textures.texture_pev.clone(),
                    transform: Transform::from_translation(Vec3::new(80., 350., 0.)),
                    ..Default::default()
                })
                .insert(PlanetEvacuationVessel);
        });
}

fn clear_scenario(mut commands: Commands, scenes: Query<Entity, With<SurfaceSceneRoot>>) {
    for entity in scenes.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
