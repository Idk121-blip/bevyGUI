use crate::{PLOT, PLOTUPDATE, TILE_DIMENSION};
use bevy::asset::Assets;
use bevy::asset::Handle;
use bevy::ecs::system::Res;
use bevy::ecs::system::ResMut;
use bevy::math::Vec2;
use bevy::prelude::AssetServer;
use bevy::sprite::TextureAtlas;
use robotics_lib::world::tile::Content::{
    Bank, Bin, Coin, Crate, Fire, Fish, Garbage, Market, Rock, Tree,
};
use robotics_lib::world::tile::TileType::*;
use robotics_lib::world::tile::{Content, Tile};
pub fn make_map(map_dim: usize) {
    let mut map = PLOT.lock().unwrap();
    let mut update_map = PLOTUPDATE.lock().unwrap();
    for x in 0..map_dim {
        map.push(vec![]);
        update_map.push(vec![]);
        for _y in 0..map_dim {
            map[x].push(None);
            update_map[x].push(None);
        }
    }
}

pub fn get_asset_of_tile(
    tile_type: &Option<Tile>,
    asset_server: &Res<AssetServer>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
) -> (Handle<TextureAtlas>, usize) {
    const ROWS: usize = 4;
    const COLUMNS: usize = 3;
    let texture_handle = asset_server.load("tile_type.png");
    let texture_atlas = TextureAtlas::from_grid(
        texture_handle,
        Vec2::new(TILE_DIMENSION, TILE_DIMENSION),
        COLUMNS,
        ROWS,
        None,
        None,
    );
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    if tile_type.is_none() {
        return (texture_atlas_handle, 11);
    }
    return match &tile_type.as_ref().unwrap().tile_type {
        DeepWater => (texture_atlas_handle, 5),
        ShallowWater => (texture_atlas_handle, 4),
        Sand => (texture_atlas_handle, 0),
        Grass => (texture_atlas_handle, 2),
        Street => (texture_atlas_handle, 1),
        Hill => (texture_atlas_handle, 7),
        Mountain => (texture_atlas_handle, 8),
        Snow => (texture_atlas_handle, 6),
        Lava => (texture_atlas_handle, 3),
        Teleport(_) => (texture_atlas_handle, 10),
        Wall => (texture_atlas_handle, 9),
    };
}

pub fn get_asset_of_content(
    tile_type: &Option<Tile>,
    asset_server: &Res<AssetServer>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
) -> Option<(Handle<TextureAtlas>, usize)> {
    if tile_type.is_none() {
        return None;
    }
    let texture_handle = asset_server.load("tile_type.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(32.0, 32.0), 3, 5, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    match tile_type.as_ref().unwrap().content {
        Rock(_) => {
            return Some((texture_atlas_handle, 0));
        }
        Tree(_) => {
            return Some((texture_atlas_handle, 0));
        }
        Garbage(_) => {
            return Some((texture_atlas_handle, 0));
        }
        Fire => {
            return Some((texture_atlas_handle, 0));
        }
        Coin(_) => {
            return Some((texture_atlas_handle, 1));
        }
        Bin(_) => {
            return Some((texture_atlas_handle, 0));
        }
        Crate(_) => {
            return Some((texture_atlas_handle, 0));
        }
        Bank(_) => {
            return Some((texture_atlas_handle, 0));
        }
        Content::Water(_) => {
            return Some((texture_atlas_handle, 0));
        }
        Market(_) => {
            return Some((texture_atlas_handle, 0));
        }
        Fish(_) => {
            return Some((texture_atlas_handle, 0));
        }
        Content::Building => {
            return Some((texture_atlas_handle, 0));
        }
        Content::Bush(_) => {
            return Some((texture_atlas_handle, 0));
        }
        Content::JollyBlock(_) => {
            return Some((texture_atlas_handle, 0));
        }
        Content::Scarecrow => {
            return Some((texture_atlas_handle, 0));
        }
        Content::None => {
            return None;
        }
    }
}
