use crate::{actions::Actions, GameCamera};
use bevy::prelude::*;

use crate::{loading::TextureAssets, GameState};

pub struct PlayerPlugin;

#[derive(Component)]
pub struct Player;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), spawn_player);
        app.add_systems(Update, move_player.run_if(in_state(GameState::Playing)));
        app.add_systems(
            Update,
            follow_player_with_camera.run_if(in_state(GameState::Playing)),
        );
    }
}

fn spawn_player(mut commands: Commands, textures: Res<TextureAssets>) {
    commands
        .spawn(SpriteBundle {
            texture: textures.texture_player.clone(),
            transform: Transform::from_translation(Vec3::new(0., 0., 1.)),
            ..Default::default()
        })
        .insert(Player);
}

fn move_player(
    time: Res<Time>,
    actions: Res<Actions>,
    mut player_query: Query<&mut Transform, With<Player>>,
) {
    if actions.player_movement.is_none() {
        return;
    }

    let speed = 300.;
    let movement = Vec3::new(
        actions.player_movement.unwrap().x * speed * time.delta_seconds(),
        actions.player_movement.unwrap().y * speed * time.delta_seconds(),
        0.,
    );
    for mut player_transform in &mut player_query {
        player_transform.translation += movement;
    }
}

type PlayerQuery<'w, 's> = Query<'w, 's, &'w Transform, With<Player>>;
type CameraQuery<'w, 's> = Query<'w, 's, &'w mut Transform, With<GameCamera>>;

fn follow_player_with_camera(mut set: ParamSet<(PlayerQuery, CameraQuery)>) {
    // retrieves translation of the player
    let target_translation = set.p0().single().translation;

    // sets translation on the camera
    set.p1().single_mut().translation = target_translation;
}
