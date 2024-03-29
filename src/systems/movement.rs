use std::sync::atomic::Ordering;

use bevy::prelude::*;

use crate::components::action::*;
use crate::components::character::*;
use crate::constants::action::ACTION_TICK_FREQUENCY;

/// Updates the position of characters based on their actions.
///
/// This system moves characters each frame based on their current actions
/// and updates their busy status when actions are completed.
///
/// # Parameters
/// - `query`: Query for accessing and modifying character transforms, busy status, and actions.
pub fn move_characters(mut query: Query<(&mut Transform, &mut Busy, &Action, &mut ActionTimer)>) {
    for (mut transform, busy, action, mut timer) in query.iter_mut() {
        if !busy.load(Ordering::Acquire) {
            continue;
        }

        let transformation = action.get_transformation();

        // Don't move if the animation is not a moving animation
        // Shouldn't impact performances, but can be replaced by a component in the action bundle
        if transformation == Vec3::new(0., 0., 0.) {
            continue;
        }

        timer.tick(ACTION_TICK_FREQUENCY);

        if timer.finished() {
            busy.store(false, Ordering::Release);
            transform.translation = transform.translation.round();
            return;
        }

        let movement = transformation
            * Vec3::splat(ACTION_TICK_FREQUENCY.as_secs_f32() / timer.duration().as_secs_f32());

        transform.translation += movement;
    }
}

/// Adjusts the camera's position to follow the player.
///
/// This system sets the camera's position to match the player's position,
/// ensuring that the camera always centers on the player.
///
/// # Parameters
/// - `player_query`: Query to access the player's transform.
/// - `camera_query`: Query to access and modify the camera's transform.
pub fn camera_follow_player(
    player_query: Query<&Transform, With<IsUser>>,
    mut camera_query: Query<&mut Transform, (With<Camera>, Without<IsUser>)>,
) {
    let player_transform = player_query.single();
    let mut camera_transform = camera_query.single_mut();
    camera_transform.translation = player_transform.translation;
}
