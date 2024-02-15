use crate::gui::components::{ROBOT_ASSET_FILE, ROBOT_DIMENSION, TILE_DIMENSION};
use crate::gui::utils::{content_positioner, map_tile_positioner};
use crate::{
    ContentToDespawn, MapTickUpdate, MapToDespawn, RobotResource, RobotUI, TextUpdate, TickUpdate,
    EVENT, MAP_DIMENSION, MAP_UPDATE_TICK_TIME, PLOT, PLOTUPDATE, PROCESS_TICK_TIME, ROBOT_COL,
    ROBOT_ROW,
};
pub use bevy::core_pipeline::clear_color::ClearColorConfig;
pub use bevy::prelude::*;

//todo! check if window_query may be useful

pub(crate) fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut runner: NonSendMut<RobotResource>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    // window_query: Query<&Window, With<PrimaryWindow>>,
) {
    // let window: &Window = window_query.get_single().unwrap();
    let _ = runner.runner.game_tick();
    commands.spawn(Camera2dBundle {
        // transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        camera_2d: Camera2d {
            clear_color: ClearColorConfig::Custom(Color::DARK_GRAY),
        },

        ..default()
    });

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

            let map_entity_id = map_tile_positioner(
                &map_to_update[j][i],
                &mut texture_atlases,
                &asset_server,
                i,
                j,
                &mut commands,
                *map_dimension,
            );
            let map_entity_content_id = content_positioner(
                &map_to_update[j][i],
                &mut texture_atlases,
                &asset_server,
                i,
                j,
                &mut commands,
                *map_dimension,
            );

            map_entity_content_ids[i].push(map_entity_content_id);
            map_entity_tile_ids[i].push(map_entity_id);
        }
    }
    robot_spawn(
        &mut commands,
        &asset_server,
        *robot_row,
        *robot_col,
        *map_dimension,
    );
    commands.spawn(MapToDespawn {
        plot: map_entity_tile_ids,
    });
    commands.spawn(ContentToDespawn {
        plot: map_entity_content_ids,
    });

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
    commands.spawn(TextUpdate(Timer::from_seconds(
        PROCESS_TICK_TIME,
        TimerMode::Repeating,
    )));
}

fn robot_spawn(
    commands: &mut Commands,
    asset_server: &AssetServer,
    robot_row: usize,
    robot_col: usize,
    map_dimension: usize,
) {
    let texture = asset_server.load(ROBOT_ASSET_FILE);
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(ROBOT_DIMENSION, ROBOT_DIMENSION)),
                ..default()
            },
            transform: Transform::from_xyz(
                robot_row as f32 * TILE_DIMENSION,
                (map_dimension - robot_col) as f32 * TILE_DIMENSION,
                0.0,
            ),
            texture,
            ..default()
        },
        RobotUI {},
    ));
}
