mod components;
mod systems;
use systems::*;

use bevy::prelude::*;

pub const BOID_SIZE: bevy::prelude::Vec2 = Vec2::new(0.15, 0.075); // height, radius
pub const BOID_COUNT: i32 = 40;

pub struct BoidsPlugin;

impl Plugin for BoidsPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_boids).add_system(update_boids);
    }
}
