use crate::gui::components::TILE_DIMENSION;
use crate::gui::utils::*;
use crate::RobotUI;
pub(in crate::gui) use crate::TickUpdate;
use crate::EVENT;
use crate::MAP_DIMENSION;
use crate::PLOT;
use crate::PLOTUPDATE;
use crate::ROBOT_COL;
use crate::ROBOT_ROW;
use crate::{ContentToDespawn, MapToDespawn};
use crate::{MapEvent, RobotResource};
pub use bevy::prelude::*;
use robotics_lib::world::tile::Content;
pub(crate) fn update_map(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    time: Res<Time>,
    mut tick_timer: Query<&mut TickUpdate>,
    mut map_tile_entities: Query<&mut MapToDespawn>,
    mut content_tile_entities: Query<&mut ContentToDespawn>,
    mut robot_ui: Query<(&mut Transform, &mut RobotUI)>,
    mut runner: NonSendMut<RobotResource>,
    mut query: Query<&mut Transform, (With<Camera>, Without<RobotUI>)>,
) {
    for mut timer in &mut tick_timer.iter_mut() {
        timer.tick(time.delta());
        if timer.just_finished() {
            let _ = runner.runner.game_tick();
            let mut events = EVENT.lock().unwrap();
            let map_dimension = MAP_DIMENSION.lock().unwrap();
            if events.contains(&MapEvent::UpdateMap) {
                tile_checker(
                    *map_dimension,
                    &mut texture_atlases,
                    &asset_server,
                    &mut commands,
                    &mut map_tile_entities,
                    &mut content_tile_entities,
                );

                remove_event(Box::new(&mut *events), MapEvent::UpdateMap)
            }
            if events.contains(&MapEvent::UpdateRobot) {
                robot_movement(&mut robot_ui, *map_dimension, &mut query);
                remove_event(Box::new(&mut *events), MapEvent::UpdateRobot);
            }
        }
    }
}

fn remove_event(events: Box<&mut Vec<MapEvent>>, event_to_remove: MapEvent) {
    let mut remove = 0;
    for (index, event) in events.iter().enumerate() {
        if *event == event_to_remove {
            remove = index;
            break;
        }
    }
    (*events).remove(remove);
}

fn tile_checker(
    map_dimension: usize,
    mut texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
    asset_server: &Res<AssetServer>,
    mut commands: &mut Commands,
    query2: &mut Query<&mut MapToDespawn>,
    query3: &mut Query<&mut ContentToDespawn>,
) {
    let mut map_to_update = PLOT.lock().unwrap();
    let map_updated = PLOTUPDATE.lock().unwrap();
    for i in 0..map_dimension {
        for j in 0..map_dimension {
            if map_to_update[j][i] != map_updated[j][i] {
                if map_to_update[j][i] == None {
                    map_to_update[j][i] = map_updated[j][i].clone();
                    commands.entity(query2.single().plot[i][j]).despawn();
                    query2.single_mut().plot[i][j] = map_tile_positioner(
                        &map_to_update[j][i],
                        &mut texture_atlases,
                        &asset_server,
                        i,
                        j,
                        &mut commands,
                        map_dimension,
                    );
                    query3.single_mut().plot[i][j] = content_positioner(
                        &map_to_update[j][i],
                        &mut texture_atlases,
                        &asset_server,
                        i,
                        j,
                        &mut commands,
                        map_dimension,
                    );
                } else {
                    if map_to_update[j][i].as_ref().unwrap().tile_type
                        != map_updated[j][i].as_ref().unwrap().tile_type
                    {
                        map_to_update[j][i].as_mut().unwrap().tile_type =
                            map_updated[j][i].clone().unwrap().tile_type;
                        query2.single_mut().plot[i][j] = map_tile_positioner(
                            &map_to_update[j][i],
                            &mut texture_atlases,
                            &asset_server,
                            i,
                            j,
                            &mut commands,
                            map_dimension,
                        );
                    }
                    if map_to_update[j][i].as_ref().unwrap().content
                        != map_updated[j][i].as_ref().unwrap().content
                    {
                        if map_to_update[j][i].as_ref().unwrap().content != Content::None {
                            commands
                                .entity(query3.single().plot[i][j].unwrap())
                                .despawn();
                        }
                        map_to_update[j][i].as_mut().unwrap().content =
                            map_updated[j][i].clone().unwrap().content;
                        query3.single_mut().plot[i][j] = content_positioner(
                            &map_to_update[j][i],
                            &mut texture_atlases,
                            &asset_server,
                            i,
                            j,
                            &mut commands,
                            map_dimension,
                        );
                    }
                }
            }
        }
    }
}

fn robot_movement(
    characters: &mut Query<(&mut Transform, &mut RobotUI)>,
    map_dimension: usize,
    mut query: &mut Query<&mut Transform, (With<Camera>, Without<RobotUI>)>,
) {
    let robot_col = ROBOT_COL.lock().unwrap();
    let robot_row = ROBOT_ROW.lock().unwrap();
    let x_trans = *robot_row as f32 * TILE_DIMENSION;
    let y_trans = (map_dimension - *robot_col) as f32 * TILE_DIMENSION;
    for (mut transform, _player) in characters {
        transform.translation.x = x_trans;
        transform.translation.y = y_trans;
    }
    for mut projection in query.iter_mut() {
        projection.translation.y = y_trans;
        projection.translation.x = x_trans;
    }
}
