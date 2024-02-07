use crate::utils::{get_asset_of_content, get_asset_of_tile};
use bevy::ecs::query::QuerySingleError;

use crate::RobotUI;
use crate::{ContentToDespawn, MapToDespawn};

use crate::TickUpdate;
use crate::EVENT;
use crate::MAP_DIMENSION;
use crate::PLOT;
use crate::PLOTUPDATE;
use crate::ROBOT_COL;
use crate::ROBOT_ROW;
use crate::TILE_DIMENSION;

use crate::MapEvent;
pub use bevy::prelude::*;
use robotics_lib::world::tile::Content;

pub(crate) fn update_map(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    time: Res<Time>,
    mut query: Query<&mut TickUpdate>,
    mut query2: Query<&mut MapToDespawn>,
    mut query3: Query<&mut ContentToDespawn>,
    mut characters: Query<(&mut Transform, &mut RobotUI)>,
) {
    //todo! rename
    for mut timer in &mut query.iter_mut() {
        timer.tick(time.delta());
        if timer.just_finished() {
            let map_dimension = MAP_DIMENSION.lock().unwrap();

            let mut map_to_update = PLOT.lock().unwrap();
            let map_updated = PLOTUPDATE.lock().unwrap();
            let mut events = EVENT.lock().unwrap();

            if events.contains(&MapEvent::UpdateMap) {
                for i in 0..*map_dimension {
                    for j in 0..*map_dimension {
                        if map_to_update[j][i] != map_updated[j][i] {
                            if map_to_update[j][i] == None {
                                map_to_update[j][i] = map_updated[j][i].clone();
                                let (texture_atlas_handle, index) = get_asset_of_tile(
                                    &map_to_update[j][i],
                                    &asset_server,
                                    &mut texture_atlases,
                                );
                                //todo! rename
                                let test = Transform::from_xyz(
                                    (i * 32) as f32,
                                    ((*map_dimension - j) * 32) as f32,
                                    -5.0,
                                )
                                .with_scale(Vec3::new(1.0, 1.0, 0.0));
                                commands.entity(query2.single_mut().plot[i][j]).despawn();
                                let map_entity_id = commands
                                    .spawn(SpriteSheetBundle {
                                        texture_atlas: texture_atlas_handle,
                                        sprite: TextureAtlasSprite::new(index),
                                        transform: test,
                                        ..default()
                                    })
                                    .id();
                                let mut map_entity_content_id = None;
                                if let Some((texture_atlas_handle, index)) = get_asset_of_content(
                                    &map_to_update[j][i],
                                    &asset_server,
                                    &mut texture_atlases,
                                ) {
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
                                query2.single_mut().plot[i][j] = map_entity_id;
                                query3.single_mut().plot[i][j] = map_entity_content_id;
                            } else {
                                if map_to_update[j][i].as_ref().unwrap().tile_type
                                    != map_updated[j][i].as_ref().unwrap().tile_type
                                {
                                    map_to_update[j][i].as_mut().unwrap().tile_type =
                                        map_updated[j][i].clone().unwrap().tile_type;
                                    let (texture_atlas_handle, index) = get_asset_of_tile(
                                        &map_to_update[j][i],
                                        &asset_server,
                                        &mut texture_atlases,
                                    );
                                    //todo! rename
                                    let test = Transform::from_xyz(
                                        (i * 32) as f32,
                                        ((*map_dimension - j) * 32) as f32,
                                        -5.0,
                                    )
                                    .with_scale(Vec3::new(1.0, 1.0, 0.0));
                                    commands.entity(query2.single_mut().plot[i][j]).despawn();
                                    let map_entity_id = commands
                                        .spawn(SpriteSheetBundle {
                                            texture_atlas: texture_atlas_handle,
                                            sprite: TextureAtlasSprite::new(index),
                                            transform: test,
                                            ..default()
                                        })
                                        .id();

                                    query2.single_mut().plot[i][j] = map_entity_id;
                                }
                                if map_to_update[j][i].as_ref().unwrap().content
                                    != map_updated[j][i].as_ref().unwrap().content
                                {
                                    if map_to_update[j][i].as_ref().unwrap().content
                                        != Content::None
                                    {
                                        commands
                                            .entity(query3.single_mut().plot[i][j].unwrap())
                                            .despawn();
                                    }
                                    map_to_update[j][i].as_mut().unwrap().content =
                                        map_updated[j][i].clone().unwrap().content;
                                    let mut map_entity_content_id = None;
                                    if let Some((texture_atlas_handle, index)) =
                                        get_asset_of_content(
                                            &map_to_update[j][i],
                                            &asset_server,
                                            &mut texture_atlases,
                                        )
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
                                    query3.single_mut().plot[i][j] = map_entity_content_id;
                                }
                            }
                        }
                        //todo! cambiare nome in mapUI
                    }
                }
                let mut remove = 0;
                for (index, event) in events.iter().enumerate() {
                    if event == &MapEvent::UpdateMap {
                        remove = index;
                        break;
                    }
                }
                events.remove(remove);
            }
            if events.contains(&MapEvent::UpdateRobot) {
                let mut remove = 0;
                let robot_col = ROBOT_COL.lock().unwrap();
                let robot_row = ROBOT_ROW.lock().unwrap();
                for (mut transform, _player) in &mut characters {
                    transform.translation.x = *robot_row as f32 * TILE_DIMENSION;
                    transform.translation.y = (*map_dimension - *robot_col) as f32 * TILE_DIMENSION;
                }

                for (index, event) in events.iter().enumerate() {
                    if event == &MapEvent::UpdateRobot {
                        remove = index;
                        break;
                    }
                }
                events.remove(remove);
            }
        }

        // if  {  }
    }
}
//todo! fix this
// pub(crate) fn camera_movement(
//     mut query: Query<&mut Transform, (With<Camera>, Without<RobotUI>)>,
//     time: Res<Time>,
//
//     mut query2: Query<&mut TickUpdate>,
//     player: Query<&Transform, With<RobotUI>>,
// ) {
//     for mut projection in query.iter_mut() {
//         let tranform_player = player.single();
//         let player_x = tranform_player.translation.x;
//         let player_y = tranform_player.translation.y;
//         projection.translation.y = player_y;
//         projection.translation.x = player_x;
//     }
// }

/*

*/

// match query2.get_single_mut() {
//     Ok(mut x) => {
//         if map_to_update[j][i].as_ref().unwrap().content
//             != Content::None
//             && map_to_update[j][i].as_ref().unwrap().content
//                 != map_updated[j][i].as_ref().unwrap().content
//         {
//             commands
//                 .entity(query3.single_mut().plot[i][j].unwrap())
//                 .despawn();
//             if let Some((texture_atlas_handle, index)) =
//                 get_asset_of_content(
//                     &map_to_update[j][i],
//                     &asset_server,
//                     &mut texture_atlases,
//                 )
//             {
//                 let test = Transform::from_xyz(
//                     (i * 32) as f32,
//                     ((*t - j) * 32) as f32,
//                     -3.0,
//                 )
//                 .with_scale(Vec3::new(0.5, 0.5, 0.0));
//                 query3.single_mut().plot[i][j] = Some(
//                     commands
//                         .spawn(SpriteSheetBundle {
//                             texture_atlas: texture_atlas_handle,
//                             sprite: TextureAtlasSprite::new(index),
//                             transform: test,
//                             ..default()
//                         })
//                         .id(),
//                 );
//             }
//         } //else if  {  }
//         commands.entity(x.plot[j][i]).despawn();
//         map_to_update[j][i] = map_updated[j][i].clone();
//         let (texture_atlas_handle, index) = get_asset_of_tile(
//             &map_to_update[j][i],
//             &asset_server,
//             &mut texture_atlases,
//         );
//
//         let test = Transform::from_xyz(
//             (i * 32) as f32,
//             ((*t - j) * 32) as f32,
//             -5.0,
//         )
//         .with_scale(Vec3::new(1.0, 1.0, 0.0));
//         let map_entity_id = commands
//             .spawn(SpriteSheetBundle {
//                 texture_atlas: texture_atlas_handle,
//                 sprite: TextureAtlasSprite::new(index),
//                 transform: test,
//                 ..default()
//             })
//             .id();
//         x.plot[i][j] = map_entity_id;
//     }
//     Err(_) => {}
