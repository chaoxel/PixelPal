use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

use crate::components::map::*;
use crate::constants::map::*;

/// Bundle for creating a data tile entity.
#[derive(Bundle)]
pub struct DataTileBundle {
    pub tile: TileBundle,
    pub level: ReliefLevel,
}

/// Bundle for creating a layer entity.
#[derive(Bundle, Clone)]
pub struct Layer {
    pub id: LayerId,
    pub tilemap: TilemapBundle,
}

impl Layer {
    pub fn new(
        id: u32,
        storage: TileStorage,
        texture: TilemapTexture,
        transform: Transform,
    ) -> Self {
        Self {
            id: LayerId(id),
            tilemap: TilemapBundle {
                grid_size: TILE_SIZE.into(),
                size: CHUNK_SIZE.into(),
                storage,
                texture,
                tile_size: TILE_SIZE,
                transform,
                ..Default::default()
            },
        }
    }
}
