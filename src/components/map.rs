use std::fmt;
use std::sync::Arc;

use bevy::ecs::system::CommandQueue;
use bevy::prelude::*;
use bevy::tasks::Task;
use bevy::utils::HashMap;
use bevy_ecs_tilemap::map::TilemapTexture;
use tokio::sync::mpsc;
use tokio::sync::mpsc::Receiver;
use tokio::sync::mpsc::Sender;

use crate::constants::map::CHUNK_SPAWNING_CHANNEL_BUFFER_SIZE;

/// Resource representing the main tilemap texture.
#[derive(Resource, Deref, DerefMut)]
pub struct MainTilemapTexture(
    /// The handle to the tilemap texture.
    Option<Arc<TilemapTexture>>,
);

/// Communication channel for sending CommandQueues
#[derive(Resource)]
pub struct ChunkSpawningChannel {
    pub sender: Sender<CommandQueue>,
    pub receiver: Receiver<CommandQueue>,
}

/// Component representing the relief level of a tile.
#[derive(Component, Deref)]
pub struct ReliefLevel(
    /// The relief level value.
    pub u32,
);

/// Component representing the name used for saving.
#[derive(Component)]
pub struct SavingName(
    /// The name used for saving.
    pub String,
);

/// Component representing the chunk map.
#[derive(Component, Resource, Default, Deref, DerefMut)]
pub struct ChunkMap(
    /// A HashMap that maps chunk positions to their respective entities.
    pub HashMap<IVec2, (Entity, Entity)>,
);

/// Component representing the ID of a layer.
#[derive(Component, Clone, Deref)]
pub struct LayerId(
    /// The ID of the layer.
    pub u32,
);

/// Component representing a chunk task with a command queue.
#[derive(Component, Deref)]
pub struct ChunkTask(pub Task<CommandQueue>);

impl MainTilemapTexture {
    /// Creates a new `MainTilemapTexture` with no texture handle.
    pub fn default() -> Self {
        Self(None)
    }

    /// Sets the tilemap texture handle.
    ///
    /// # Arguments
    ///
    /// * `image` - The handle to the image resource.
    pub fn set_handle(&mut self, image: Handle<Image>) {
        self.0 = Some(Arc::new(TilemapTexture::Single(image)));
    }

    /// Retrieves the tilemap texture.
    pub fn clone_arc(&self) -> Arc<TilemapTexture> {
        self.0
            .as_ref()
            .expect("Main tilemap texture not set!")
            .clone()
    }
}

impl ChunkSpawningChannel {
    pub fn new() -> Self {
        let (sender, receiver) = mpsc::channel(CHUNK_SPAWNING_CHANNEL_BUFFER_SIZE);
        Self { sender, receiver }
    }
}

impl fmt::Display for ReliefLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl ChunkMap {
    /// Creates a new `ChunkMap`.
    pub fn new() -> Self {
        Self(HashMap::new())
    }
}
