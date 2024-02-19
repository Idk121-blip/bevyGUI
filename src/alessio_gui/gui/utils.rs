use crate::gui::components::{
    CONTENT_ASSET_FILE, CONTENT_Z, TILE_ASSET_FILE, TILE_DIMENSION, TILE_Z,
};
use bevy::prelude::*;
use robotics_lib::world::tile::Content::{
    Bank, Bin, Coin, Crate, Fire, Fish, Garbage, Market, Rock, Tree,
};
use robotics_lib::world::tile::TileType::{
    DeepWater, Grass, Hill, Lava, Mountain, Sand, ShallowWater, Snow, Street, Teleport, Wall,
};
use robotics_lib::world::tile::{Content, Tile};

pub(in crate::alessio_gui::gui) fn content_positioner(
    tile: &Option<Tile>,
    mut texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
    asset_server: &Res<AssetServer>,
    i: usize,
    j: usize,
    commands: &mut Commands,
    map_dimension: usize,
) -> Option<Entity> {
    let mut map_entity_content_id = None;
    if let Some((texture_atlas_handle, index)) =
        get_asset_of_content_from_tile(&tile, &asset_server, &mut texture_atlases)
    {
        let test = Transform::from_xyz(
            i as f32 * TILE_DIMENSION,
            (map_dimension - j) as f32 * TILE_DIMENSION,
            CONTENT_Z,
        )
        .with_scale(Vec3::new(0.7, 0.7, 0.0));
        map_entity_content_id = Some(
            commands
                .spawn(SpriteSheetBundle {
                    texture_atlas: texture_atlas_handle,
                    sprite: TextureAtlasSprite::new(index),
                    transform: test,
                    ..default()
                })
                .id(),
        );
    }
    map_entity_content_id
}

pub(in crate::alessio_gui::gui) fn map_tile_positioner(
    tile: &Option<Tile>,
    mut texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
    asset_server: &Res<AssetServer>,
    i: usize,
    j: usize,
    commands: &mut Commands,
    map_dimension: usize,
) -> Entity {
    let (texture_atlas, index) = get_asset_of_tile(&tile, &asset_server, &mut texture_atlases);
    let transform = Transform::from_xyz(
        i as f32 * TILE_DIMENSION,
        (map_dimension - j) as f32 * TILE_DIMENSION,
        TILE_Z,
    )
    .with_scale(Vec3::new(1.0, 1.0, 0.0));

    let map_entity_id = commands
        .spawn(SpriteSheetBundle {
            texture_atlas,
            transform,
            sprite: TextureAtlasSprite::new(index),
            ..default()
        })
        .id();
    map_entity_id
}
fn get_asset_of_tile(
    tile_type: &Option<Tile>,
    asset_server: &Res<AssetServer>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
) -> (Handle<TextureAtlas>, usize) {
    const ROWS: usize = 4;
    const COLUMNS: usize = 3;
    let texture_handle = asset_server.load(TILE_ASSET_FILE);
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

fn get_asset_of_content_from_tile(
    tile_type: &Option<Tile>,
    asset_server: &Res<AssetServer>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
) -> Option<(Handle<TextureAtlas>, usize)> {
    if tile_type.is_none() {
        return None;
    }

    return get_asset_of_content(
        &tile_type.as_ref().unwrap().content,
        asset_server,
        texture_atlases,
    );
}

pub(crate) fn get_asset_of_content(
    content: &Content,
    asset_server: &Res<AssetServer>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
) -> Option<(Handle<TextureAtlas>, usize)> {
    const COLUMNS: usize = 3;
    const ROWS: usize = 5;

    let texture_handle = asset_server.load(CONTENT_ASSET_FILE);
    let texture_atlas = TextureAtlas::from_grid(
        texture_handle,
        Vec2::new(TILE_DIMENSION, TILE_DIMENSION),
        COLUMNS,
        ROWS,
        None,
        None,
    );
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    return match content {
        Rock(_) => Some((texture_atlas_handle, 4)),
        Tree(_) => Some((texture_atlas_handle, 5)),
        Garbage(_) => Some((texture_atlas_handle, 2)),
        Fire => Some((texture_atlas_handle, 0)),
        Coin(_) => Some((texture_atlas_handle, 3)),
        Bin(_) => Some((texture_atlas_handle, 1)),
        Crate(_) => Some((texture_atlas_handle, 8)),
        Bank(_) => Some((texture_atlas_handle, 6)),
        Content::Water(_) => Some((texture_atlas_handle, 7)),
        Market(_) => Some((texture_atlas_handle, 9)),
        Fish(_) => Some((texture_atlas_handle, 11)),
        Content::Building => Some((texture_atlas_handle, 10)),
        Content::Bush(_) => Some((texture_atlas_handle, 12)),
        Content::JollyBlock(_) => None,
        Content::Scarecrow => None,
        Content::None => None,
    };
}
