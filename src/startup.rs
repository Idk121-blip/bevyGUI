use crate::utils::{get_asset_of_content, get_asset_of_tile};
use crate::{
    ContentToDespawn, MapTickUpdate, MapToDespawn, RobotResource, RobotUI, TickUpdate, EVENT,
    MAP_DIMENSION, MAP_UPDATE_TICK_TIME, PLOT, PLOTUPDATE, PROCESS_TICK_TIME, ROBOT_COL, ROBOT_ROW,
    TILE_DIMENSION,
};

pub use bevy::core_pipeline::clear_color::ClearColorConfig;
pub use bevy::prelude::*;
use robotics_lib::world::tile::Content::Coin;
pub(crate) fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,

    mut runner: NonSendMut<RobotResource>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let _ = runner.runner.game_tick();
    commands.spawn(Camera2dBundle {
        camera_2d: Camera2d {
            clear_color: ClearColorConfig::Custom(Color::DARK_GRAY),
        },
        ..default()
    });
    //todo! rename
    let map_dimension = MAP_DIMENSION.lock().unwrap();
    let robot_col = ROBOT_COL.lock().unwrap();
    let robot_row = ROBOT_ROW.lock().unwrap();

    let mut map_to_update = PLOT.lock().unwrap();
    let map_updated = PLOTUPDATE.lock().unwrap();

    let mut map_entity_tile_ids = Vec::new();
    let mut map_entity_content_ids = Vec::new();

    for i in 0..*map_dimension {
        map_entity_tile_ids.push(vec![]);
        map_entity_content_ids.push(vec![]);
        for j in 0..*map_dimension {
            map_to_update[j][i] = map_updated[j][i].clone();
            let (texture_atlas_handle, index) =
                get_asset_of_tile(&map_to_update[j][i], &asset_server, &mut texture_atlases);
            //todo! rename
            let test =
                Transform::from_xyz((i * 32) as f32, ((*map_dimension - j) * 32) as f32, -5.0)
                    .with_scale(Vec3::new(1.0, 1.0, 0.0));
            let map_entity_id = commands
                .spawn(SpriteSheetBundle {
                    texture_atlas: texture_atlas_handle,
                    sprite: TextureAtlasSprite::new(index),
                    transform: test,
                    ..default()
                })
                .id();
            let mut map_entity_content_id = None;

            if map_to_update[j][i].is_some() {
                if let Some((texture_atlas_handle, index)) =
                    get_asset_of_content(&map_to_update[j][i], &asset_server, &mut texture_atlases)
                {
                    let test = Transform::from_xyz(
                        (i * 32) as f32,
                        ((*map_dimension - j) * 32) as f32,
                        -3.0,
                    )
                    .with_scale(Vec3::new(0.5, 0.5, 0.0));
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
            }
            map_entity_content_ids[i].push(map_entity_content_id);
            map_entity_tile_ids[i].push(map_entity_id);
        }
    }

    commands.spawn(MapToDespawn {
        plot: map_entity_tile_ids,
    });
    commands.spawn(ContentToDespawn {
        plot: map_entity_content_ids,
    });

    let texture = asset_server.load("penguin.png");
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(30.0, 30.0)),

                ..default()
            },
            transform: Transform::from_xyz(
                (*robot_row) as f32 * TILE_DIMENSION,
                (*map_dimension - *robot_col) as f32 * TILE_DIMENSION,
                0.0,
            ),
            texture,
            ..default()
        },
        RobotUI {},
    ));
    {
        let mut events = EVENT.lock().unwrap();
        events.clear();
    }
    commands.spawn(TickUpdate(Timer::from_seconds(
        PROCESS_TICK_TIME,
        TimerMode::Repeating,
    )));
    commands.spawn(MapTickUpdate(Timer::from_seconds(
        PROCESS_TICK_TIME - MAP_UPDATE_TICK_TIME,
        TimerMode::Repeating,
    )));
}
