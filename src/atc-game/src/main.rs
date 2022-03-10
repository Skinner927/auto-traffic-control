use bevy::prelude::*;
use tokio::sync::broadcast::channel;

use crate::api::Api;
use crate::event::{Event, EventBus};
use crate::systems::*;

mod api;
mod components;
mod event;
mod map;
mod systems;

/// The height of the game's window
const SCREEN_HEIGHT: f32 = 768.0;

/// The width of the game's window
const SCREEN_WIDTH: f32 = 1024.0;

/// The dimension of a tile
///
/// Tiles must have the same size as the textures that are used to render them. This game uses
/// textures with a size of 32 by 32 pixels, and thus tiles must be 32 pixels high and wide as well.
const TILE_SIZE: i32 = 32;

#[tokio::main]
async fn main() {
    let (event_sender, _event_receiver) = channel::<Event>(1024);

    let _api_join_handle = tokio::spawn(Api::serve(event_sender.clone()));

    App::new()
        // Must be added before the DefaultPlugins
        .insert_resource(WindowDescriptor {
            title: "Auto Traffic Control".to_string(),
            width: SCREEN_WIDTH,
            height: SCREEN_HEIGHT,
            resizable: false,
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(DefaultPlugins)
        .insert_resource(event_sender)
        .insert_resource(SpawnTimer::new(Timer::from_seconds(1.0, true)))
        .add_startup_system(setup_camera)
        .add_startup_system(setup_airport)
        .add_startup_system(setup_grid)
        .add_system(despawn_airplane)
        .add_system(follow_flight_plan)
        .add_system(spawn_airplane)
        .run();
}