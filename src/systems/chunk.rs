use std::ops::Mul;

use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use crate::bundles::map::ChunkBundle;
use crate::components::flags::IsGameCamera;
use crate::components::mapping::ChunkList;
use crate::constants::mapping::*;
use crate::generation::noise::TiledNoise;


pub fn spawn_chunk(
    commands: &mut Commands,
    asset_server: &AssetServer,
    chunk_list: &mut ChunkList,
    chunk_pos: IVec2,
) {
    let chunk_entity = commands.spawn_empty().id();
    let mut tile_storage = TileStorage::empty(CHUNK_SIZE.into());

    let base_x = chunk_pos.x as i32 * CHUNK_SIZE.x as i32;
    let base_y = chunk_pos.y as i32 * CHUNK_SIZE.y as i32;
    let noise = TiledNoise::new(0);

    for x in 0..CHUNK_SIZE.x {
        for y in 0..CHUNK_SIZE.y {
            let tile_pos = TilePos { x, y };
            let value = noise.get_value(base_x + x as i32 , base_y + y as i32);
            let tile_entity = commands
                .spawn(TileBundle {
                    position: tile_pos,
                    texture_index: TileTextureIndex(value),
                    tilemap_id: TilemapId(chunk_entity),
                    ..Default::default()
                }).id();
            commands.entity(chunk_entity).add_child(tile_entity);
            tile_storage.set(&tile_pos, tile_entity);
        }
    }

    let transform = Transform::from_xyz(
        base_x as f32 * TILE_SIZE.x,
        base_y as f32 * TILE_SIZE.y,
        0.0,
    );

    let texture_handle: Handle<Image> = asset_server.load("tileset/environment/debug.png");

    let chunk = ChunkBundle::new(
        tile_storage,
        TilemapTexture::Single(texture_handle),
        transform
    );

    chunk_list.list.insert(chunk_pos, chunk_entity);

    commands.entity(chunk_entity).insert(chunk);
}

fn camera_pos_to_chunk_pos(camera_pos: &Vec2) -> IVec2 {
    let camera_pos = camera_pos.as_ivec2();
    let chunk_size: IVec2 = IVec2::new(CHUNK_SIZE.x as i32, CHUNK_SIZE.y as i32);
    let tile_size: IVec2 = IVec2::new(TILE_SIZE.x as i32, TILE_SIZE.y as i32);
    camera_pos / (chunk_size * tile_size)
}

pub fn handle_chunk_despawning(
    mut commands: Commands,
    camera_query: Query<&Transform, With<IsGameCamera>>,
    mut chunk_list_query: Query<&mut ChunkList>
) {
    for mut chunk_list in chunk_list_query.iter_mut() {
        let camera_transform = camera_query.single();
        chunk_list.list.retain(|chunk_ipos, entity| {
            let chunk_pos = chunk_ipos.as_vec2().mul(TILE);
            let distance = camera_transform.translation.xy()
                .distance_squared(chunk_pos);

            if distance < CHUNK_DESPAWN_RANGE_PX_SQUARED {
                true // Keep the chunk
            } else {
                commands.entity(*entity).despawn_recursive();
                false // Remove the chunk
            }
        });
    }
}

pub fn handle_chunk_spawning(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    camera_query: Query<&Transform, With<IsGameCamera>>,
    mut chunk_list_query: Query<&mut ChunkList>
) {
    for mut chunk_list in chunk_list_query.iter_mut() {
        let transform = camera_query.single();
        let camera_chunk_pos = camera_pos_to_chunk_pos(
            &transform.translation.xy()
        );

        for y in (camera_chunk_pos.y - CHUNK_SPAWN_RADIUS_Y)
                    ..(camera_chunk_pos.y + CHUNK_SPAWN_RADIUS_Y) {
            for x in (camera_chunk_pos.x - CHUNK_SPAWN_RADIUS_X)
                        ..(camera_chunk_pos.x + CHUNK_SPAWN_RADIUS_X) {
                if !chunk_list.list.contains_key(&IVec2::new(x, y)) {
                    spawn_chunk(
                        &mut commands,
                        &asset_server,
                        &mut chunk_list,
                        IVec2::new(x, y)
                    );
                }
            }
        }
    }
}