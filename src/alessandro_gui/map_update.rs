use crate::alessandro_gui::generator::{MapContent, MapTile};
use crate::alessandro_gui::main::{
    EXACT_TIME, FUTURE_WEATHER, GLOBAL_3D_CONTENT, GLOBAL_3D_TILES, GLOBAL_CONTEXT, GLOBAL_SCORE,
    PAST_WORLD, ROBOT_POS, TIME, WEATHER,
};
use crate::alessandro_gui::robot_utils::adjust_robot_height;
use robotics_lib::world::environmental_conditions::{DayTime, WeatherType};
use robotics_lib::world::tile::TileType::{
    DeepWater, Grass, Hill, Lava, Mountain, Sand, ShallowWater, Snow, Street, Teleport, Wall,
};
use robotics_lib::world::tile::{Content, Tile, TileType};
use std::collections::HashMap;
use strum::IntoEnumIterator;
use three_d::{vec3, Matrix4};
use three_d_asset::Mat4;
pub fn robot_around_tile(map: Vec<Vec<Option<Tile>>>) {
    let tile_size = vec3(12.0, 0.0, 12.0);
    unsafe {
        // generate the past world if is the first time the function is called
        if PAST_WORLD.is_empty() {
            for (_, rows) in map.iter().enumerate() {
                let mut row: Vec<Option<Tile>> = Vec::new();
                for (_, _) in rows.iter().enumerate() {
                    row.push(Some(Tile {
                        tile_type: Grass,
                        content: Content::JollyBlock(0),
                        elevation: 0,
                    }));
                }
                PAST_WORLD.push(row);
            }
        }
    }
    // create the data struct to know if need to update
    let mut equal_tile = Vec::new();
    for _ in TileType::iter() {
        equal_tile.push(true);
    }
    equal_tile.push(true);
    for (row_index, row) in map.iter().enumerate() {
        for (col_index, value) in row.iter().enumerate() {
            // test -------
            unsafe {
                if &PAST_WORLD[row_index][col_index] == value {
                    // if nothing change we don't need to update
                    continue;
                } else {
                    match &PAST_WORLD[row_index][col_index] {
                        None => {
                            GLOBAL_3D_TILES[11].pop_pos(Mat4::from_translation(vec3(
                                0.0 + row_index as f32 * (tile_size.x),
                                0.0,
                                0.0 + col_index as f32 * (tile_size.z),
                            )));
                            equal_tile[11] = false;
                        }
                        Some(ex_tile) => match ex_tile.tile_type {
                            DeepWater => {
                                GLOBAL_3D_TILES[0].pop_pos(Mat4::from_translation(vec3(
                                    0.0 + row_index as f32 * (tile_size.x),
                                    0.0,
                                    0.0 + col_index as f32 * (tile_size.z),
                                )));
                                equal_tile[0] = false;
                            }
                            ShallowWater => {
                                GLOBAL_3D_TILES[1].pop_pos(Mat4::from_translation(vec3(
                                    0.0 + row_index as f32 * (tile_size.x),
                                    0.0,
                                    0.0 + col_index as f32 * (tile_size.z),
                                )));
                                equal_tile[1] = false;
                            }
                            Sand => {
                                GLOBAL_3D_TILES[2].pop_pos(Mat4::from_translation(vec3(
                                    0.0 + row_index as f32 * (tile_size.x),
                                    0.0,
                                    0.0 + col_index as f32 * (tile_size.z),
                                )));
                                equal_tile[2] = false;
                            }
                            Grass => {
                                GLOBAL_3D_TILES[3].pop_pos(Mat4::from_translation(vec3(
                                    0.0 + row_index as f32 * (tile_size.x),
                                    0.0,
                                    0.0 + col_index as f32 * (tile_size.z),
                                )));
                                equal_tile[3] = false;
                            }
                            Street => {
                                GLOBAL_3D_TILES[4].pop_pos(Mat4::from_translation(vec3(
                                    0.0 + row_index as f32 * (tile_size.x),
                                    0.0,
                                    0.0 + col_index as f32 * (tile_size.z),
                                )));
                                equal_tile[4] = false;
                            }
                            Hill => {
                                GLOBAL_3D_TILES[5].pop_pos(Mat4::from_translation(vec3(
                                    0.0 + row_index as f32 * (tile_size.x),
                                    0.0,
                                    0.0 + col_index as f32 * (tile_size.z),
                                )));
                                equal_tile[5] = false;
                            }
                            Mountain => {
                                GLOBAL_3D_TILES[6].pop_pos(Mat4::from_translation(vec3(
                                    0.0 + row_index as f32 * (tile_size.x),
                                    0.0,
                                    0.0 + col_index as f32 * (tile_size.z),
                                )));
                                equal_tile[6] = false;
                            }
                            Snow => {
                                GLOBAL_3D_TILES[7].pop_pos(Mat4::from_translation(vec3(
                                    0.0 + row_index as f32 * (tile_size.x),
                                    0.0,
                                    0.0 + col_index as f32 * (tile_size.z),
                                )));
                                equal_tile[7] = false;
                            }
                            Lava => {
                                GLOBAL_3D_TILES[8].pop_pos(Mat4::from_translation(vec3(
                                    0.0 + row_index as f32 * (tile_size.x),
                                    0.0,
                                    0.0 + col_index as f32 * (tile_size.z),
                                )));
                                equal_tile[8] = false;
                            }
                            Teleport(_) => {
                                GLOBAL_3D_TILES[9].pop_pos(Mat4::from_translation(vec3(
                                    0.0 + row_index as f32 * (tile_size.x),
                                    0.0,
                                    0.0 + col_index as f32 * (tile_size.z),
                                )));
                                equal_tile[9] = false;
                            }
                            Wall => {
                                GLOBAL_3D_TILES[10].pop_pos(Mat4::from_translation(vec3(
                                    0.0 + row_index as f32 * (tile_size.x),
                                    0.0,
                                    0.0 + col_index as f32 * (tile_size.z),
                                )));
                                equal_tile[10] = false;
                            }
                        },
                    }
                    match value {
                        Some(tile) => {
                            if PAST_WORLD[row_index][col_index].as_ref().is_some() {
                                if PAST_WORLD[row_index][col_index].as_ref().unwrap().content
                                    != tile.content
                                {
                                    update_content(tile.clone(), (row_index, col_index));
                                }
                            } else {
                                update_content(tile.clone(), (row_index, col_index));
                            }
                            match tile.tile_type {
                                DeepWater => {
                                    GLOBAL_3D_TILES[0].push_pos(Mat4::from_translation(vec3(
                                        0.0 + row_index as f32 * (tile_size.x),
                                        0.0,
                                        0.0 + col_index as f32 * (tile_size.z),
                                    )));
                                    PAST_WORLD[row_index][col_index] = Some(tile.clone());
                                    equal_tile[0] = false;
                                }
                                ShallowWater => {
                                    GLOBAL_3D_TILES[1].push_pos(Mat4::from_translation(vec3(
                                        0.0 + row_index as f32 * (tile_size.x),
                                        0.0,
                                        0.0 + col_index as f32 * (tile_size.z),
                                    )));
                                    PAST_WORLD[row_index][col_index] = Some(tile.clone());
                                    equal_tile[1] = false;
                                }
                                Sand => {
                                    GLOBAL_3D_TILES[2].push_pos(Mat4::from_translation(vec3(
                                        0.0 + row_index as f32 * (tile_size.x),
                                        0.0,
                                        0.0 + col_index as f32 * (tile_size.z),
                                    )));
                                    PAST_WORLD[row_index][col_index] = Some(tile.clone());
                                    equal_tile[2] = false;
                                }
                                Grass => {
                                    GLOBAL_3D_TILES[3].push_pos(Mat4::from_translation(vec3(
                                        0.0 + row_index as f32 * (tile_size.x),
                                        0.0,
                                        0.0 + col_index as f32 * (tile_size.z),
                                    )));
                                    PAST_WORLD[row_index][col_index] = Some(tile.clone());
                                    equal_tile[3] = false;
                                }
                                Street => {
                                    GLOBAL_3D_TILES[4].push_pos(Mat4::from_translation(vec3(
                                        0.0 + row_index as f32 * (tile_size.x),
                                        0.0,
                                        0.0 + col_index as f32 * (tile_size.z),
                                    )));
                                    PAST_WORLD[row_index][col_index] = Some(tile.clone());
                                    equal_tile[4] = false;
                                }
                                Hill => {
                                    GLOBAL_3D_TILES[5].push_pos(Mat4::from_translation(vec3(
                                        0.0 + row_index as f32 * (tile_size.x),
                                        0.0,
                                        0.0 + col_index as f32 * (tile_size.z),
                                    )));
                                    PAST_WORLD[row_index][col_index] = Some(tile.clone());
                                    equal_tile[5] = false;
                                }
                                Mountain => {
                                    GLOBAL_3D_TILES[6].push_pos(Mat4::from_translation(vec3(
                                        0.0 + row_index as f32 * (tile_size.x),
                                        0.0,
                                        0.0 + col_index as f32 * (tile_size.z),
                                    )));
                                    PAST_WORLD[row_index][col_index] = Some(tile.clone());
                                    equal_tile[6] = false;
                                }
                                Snow => {
                                    GLOBAL_3D_TILES[7].push_pos(Mat4::from_translation(vec3(
                                        0.0 + row_index as f32 * (tile_size.x),
                                        0.0,
                                        0.0 + col_index as f32 * (tile_size.z),
                                    )));
                                    PAST_WORLD[row_index][col_index] = Some(tile.clone());
                                    equal_tile[7] = false;
                                }
                                Lava => {
                                    GLOBAL_3D_TILES[8].push_pos(Mat4::from_translation(vec3(
                                        0.0 + row_index as f32 * (tile_size.x),
                                        0.0,
                                        0.0 + col_index as f32 * (tile_size.z),
                                    )));
                                    PAST_WORLD[row_index][col_index] = Some(tile.clone());
                                    equal_tile[8] = false;
                                }
                                Teleport(_) => {
                                    GLOBAL_3D_TILES[9].push_pos(Mat4::from_translation(vec3(
                                        0.0 + row_index as f32 * (tile_size.x),
                                        0.0,
                                        0.0 + col_index as f32 * (tile_size.z),
                                    )));
                                    PAST_WORLD[row_index][col_index] = Some(tile.clone());
                                    equal_tile[9] = false;
                                }
                                Wall => {
                                    GLOBAL_3D_TILES[10].push_pos(Mat4::from_translation(vec3(
                                        0.0 + row_index as f32 * (tile_size.x),
                                        0.0,
                                        0.0 + col_index as f32 * (tile_size.z),
                                    )));
                                    PAST_WORLD[row_index][col_index] = Some(tile.clone());
                                    equal_tile[10] = false;
                                }
                            }
                        }
                        None => {
                            GLOBAL_3D_TILES[11].push_pos(Mat4::from_translation(vec3(
                                0.0 + row_index as f32 * (tile_size.x),
                                0.0,
                                0.0 + col_index as f32 * (tile_size.z),
                            )));
                            PAST_WORLD[row_index][col_index] = None;
                            equal_tile[11] = false;
                        }
                    }
                }
            }
        }
    }
    unsafe {
        // adjust the height of the robot if the movement change elevation (e.g. from hill to ground)
        if PAST_WORLD[ROBOT_POS.0][ROBOT_POS.1].as_ref().is_some()
            && (PAST_WORLD[ROBOT_POS.0][ROBOT_POS.1]
                .as_ref()
                .unwrap()
                .tile_type
                == Mountain)
        {
            adjust_robot_height(
                PAST_WORLD[ROBOT_POS.0][ROBOT_POS.1]
                    .as_ref()
                    .unwrap()
                    .tile_type
                    .clone(),
            );
            ROBOT_POS.2 = 2;
        } else if PAST_WORLD[ROBOT_POS.0][ROBOT_POS.1].as_ref().is_some()
            && PAST_WORLD[ROBOT_POS.0][ROBOT_POS.1]
                .as_ref()
                .unwrap()
                .tile_type
                == Hill
        {
            adjust_robot_height(
                PAST_WORLD[ROBOT_POS.0][ROBOT_POS.1]
                    .as_ref()
                    .unwrap()
                    .tile_type
                    .clone(),
            );
            ROBOT_POS.2 = 1;
        }
        // check if need to update istances
        for (i, tile) in GLOBAL_3D_TILES.iter_mut().enumerate() {
            if equal_tile[i] {
                continue;
            } else {
                if let Some(context) = GLOBAL_CONTEXT.as_ref() {
                    tile.update_istances(context);
                } else {
                    println!("Option is None");
                }
            }
        }
    }
}

pub fn remove_up_content(
    tile: Tile,
    (x, y): (usize, usize),
    content_vec: &mut HashMap<Content, MapContent>,
) {
    // function used to remove content form hil or mountain (because their position is not standard)
    let tile_size = vec3(12.0, 0.0, 12.0);
    match tile.tile_type {
        Hill => {
            if let Some(map_content) = content_vec.get_mut(&tile.content.clone().to_default()) {
                match tile.content {
                    Content::Rock(_) => {
                        map_content.pop_pos(
                            Matrix4::from_translation(vec3(
                                x as f32 * (tile_size.x) + 1.5,
                                0.0 + 6.3,
                                y as f32 * (tile_size.z) - 2.0,
                            )) * Mat4::from_scale(0.5),
                        );
                    }
                    Content::Tree(_) => {
                        map_content.pop_pos(
                            Matrix4::from_translation(vec3(
                                x as f32 * (tile_size.x) + 2.3,
                                0.0 + 6.15,
                                y as f32 * (tile_size.z) - 2.0,
                            )) * Mat4::from_scale(0.6),
                        );
                    }
                    Content::Garbage(_) => {
                        map_content.pop_pos(Matrix4::from_translation(vec3(
                            x as f32 * (tile_size.x) + 1.0,
                            0.0 + 5.45,
                            y as f32 * (tile_size.z) - 1.5,
                        )));
                    }
                    Content::Fire => {
                        map_content.pop_pos(Matrix4::from_translation(vec3(
                            x as f32 * (tile_size.x),
                            0.0 + 5.45,
                            y as f32 * (tile_size.z),
                        )));
                    }
                    Content::Coin(_) => {
                        map_content.pop_pos(Matrix4::from_translation(vec3(
                            x as f32 * (tile_size.x),
                            0.0 + 5.45,
                            y as f32 * (tile_size.z) + 3.0,
                        )));
                    }
                    Content::Bin(_) => {
                        map_content.pop_pos(Matrix4::from_translation(vec3(
                            x as f32 * (tile_size.x) + 0.5,
                            0.0 + 5.45,
                            y as f32 * (tile_size.z) + 0.5,
                        )));
                    }
                    Content::Crate(_) => {
                        map_content.pop_pos(Matrix4::from_translation(vec3(
                            x as f32 * (tile_size.x) + 1.0,
                            0.0 + 5.45,
                            y as f32 * (tile_size.z),
                        )));
                    }
                    Content::Building => {
                        map_content.pop_pos(Matrix4::from_translation(vec3(
                            x as f32 * (tile_size.x),
                            0.0 + 5.45,
                            y as f32 * (tile_size.z),
                        )));
                    }
                    Content::Bush(_) => {
                        map_content.pop_pos(Matrix4::from_translation(vec3(
                            x as f32 * (tile_size.x),
                            0.0 + 5.45,
                            y as f32 * (tile_size.z),
                        )));
                    }
                    _ => {
                        map_content.pop_pos(Matrix4::from_translation(vec3(
                            x as f32 * (tile_size.x),
                            0.0,
                            y as f32 * (tile_size.z),
                        )));
                    }
                }
            }
        }
        Mountain => {
            if let Some(map_content) = content_vec.get_mut(&tile.content.clone().to_default()) {
                match tile.content {
                    Content::Rock(_) => {
                        map_content.pop_pos(
                            Matrix4::from_translation(vec3(
                                x as f32 * (tile_size.x) + 2.5,
                                0.0 + 9.9,
                                y as f32 * (tile_size.z) - 3.0,
                            )) * Mat4::from_scale(0.5),
                        );
                    }
                    Content::Tree(_) => {
                        map_content.pop_pos(
                            Matrix4::from_translation(vec3(
                                x as f32 * (tile_size.x) + 2.3,
                                0.0 + 9.7,
                                y as f32 * (tile_size.z) - 2.0,
                            )) * Mat4::from_scale(0.6),
                        );
                    }
                    Content::Garbage(_) => {
                        map_content.pop_pos(Matrix4::from_translation(vec3(
                            x as f32 * (tile_size.x) + 1.5,
                            0.0 + 9.0,
                            y as f32 * (tile_size.z) - 2.5,
                        )));
                    }
                    Content::Fire => {
                        map_content.pop_pos(Matrix4::from_translation(vec3(
                            x as f32 * (tile_size.x) + 0.5,
                            0.0 + 9.0,
                            y as f32 * (tile_size.z) - 1.0,
                        )));
                    }
                    Content::Coin(_) => {
                        map_content.pop_pos(Matrix4::from_translation(vec3(
                            x as f32 * (tile_size.x) + 0.5,
                            0.0 + 9.0,
                            y as f32 * (tile_size.z) + 3.0,
                        )));
                    }
                    Content::Bin(_) => {
                        map_content.pop_pos(Matrix4::from_translation(vec3(
                            x as f32 * (tile_size.x) + 1.5,
                            0.0 + 9.0,
                            y as f32 * (tile_size.z) + 1.5,
                        )));
                    }
                    Content::Crate(_) => {
                        map_content.pop_pos(Matrix4::from_translation(vec3(
                            x as f32 * (tile_size.x) + 2.0,
                            0.0 + 9.0,
                            y as f32 * (tile_size.z) + 1.0,
                        )));
                    }
                    Content::Building => {
                        map_content.pop_pos(Matrix4::from_translation(vec3(
                            x as f32 * (tile_size.x),
                            0.0 + 9.0,
                            y as f32 * (tile_size.z),
                        )));
                    }
                    Content::Bush(_) => {
                        map_content.pop_pos(Matrix4::from_translation(vec3(
                            x as f32 * (tile_size.x),
                            0.0 + 9.0,
                            y as f32 * (tile_size.z),
                        )));
                    }
                    _ => {
                        map_content.pop_pos(Matrix4::from_translation(vec3(
                            x as f32 * (tile_size.x),
                            0.0,
                            y as f32 * (tile_size.z),
                        )));
                    }
                }
            }
        }
        _ => {}
    }
}

pub fn update_content(tile: Tile, (x, y): (usize, usize)) {
    let tile_size = vec3(12.0, 0.0, 12.0);
    let mut equal_content: HashMap<Content, bool> = HashMap::new();
    // initialize the equal hashmap used to be sure if need to update
    for c in Content::iter() {
        equal_content.insert(c.to_default(), true);
    }
    match tile.tile_type {
        Hill => {
            unsafe {
                // check if content is different
                if let Some(t) = equal_content.get_mut(
                    &PAST_WORLD[x][y]
                        .as_ref()
                        .unwrap_or(&Tile {
                            tile_type: Grass,
                            content: Content::None,
                            elevation: 0,
                        })
                        .content
                        .clone()
                        .to_default(),
                ) {
                    *t = false;
                }
                if let Some(map_content) = GLOBAL_3D_CONTENT.as_mut().unwrap().get_mut(
                    &PAST_WORLD[x][y]
                        .as_ref()
                        .unwrap_or(&Tile {
                            tile_type: Grass,
                            content: Content::None,
                            elevation: 0,
                        })
                        .content
                        .clone()
                        .to_default(),
                ) {
                    // pop the content position, need modification because hill is not equal to other tile
                    match &PAST_WORLD[x][y]
                        .as_ref()
                        .unwrap_or(&Tile {
                            tile_type: Grass,
                            content: Content::None,
                            elevation: 0,
                        })
                        .content
                        .clone()
                        .to_default()
                    {
                        Content::Rock(_) => {
                            map_content.pop_pos(
                                Matrix4::from_translation(vec3(
                                    x as f32 * (tile_size.x) + 1.5,
                                    0.0 + 6.3,
                                    y as f32 * (tile_size.z) - 2.0,
                                )) * Mat4::from_scale(0.5),
                            );
                        }
                        Content::Tree(_) => {
                            map_content.pop_pos(
                                Matrix4::from_translation(vec3(
                                    x as f32 * (tile_size.x) + 2.3,
                                    0.0 + 6.15,
                                    y as f32 * (tile_size.z) - 2.0,
                                )) * Mat4::from_scale(0.6),
                            );
                        }
                        Content::Garbage(_) => {
                            map_content.pop_pos(Matrix4::from_translation(vec3(
                                x as f32 * (tile_size.x) + 1.0,
                                0.0 + 5.45,
                                y as f32 * (tile_size.z) - 1.5,
                            )));
                        }
                        Content::Fire => {
                            map_content.pop_pos(Matrix4::from_translation(vec3(
                                x as f32 * (tile_size.x),
                                0.0 + 5.45,
                                y as f32 * (tile_size.z),
                            )));
                        }
                        Content::Coin(_) => {
                            map_content.pop_pos(Matrix4::from_translation(vec3(
                                x as f32 * (tile_size.x),
                                0.0 + 5.45,
                                y as f32 * (tile_size.z) + 3.0,
                            )));
                        }
                        Content::Bin(_) => {
                            map_content.pop_pos(Matrix4::from_translation(vec3(
                                x as f32 * (tile_size.x) + 0.5,
                                0.0 + 5.45,
                                y as f32 * (tile_size.z) + 0.5,
                            )));
                        }
                        Content::Crate(_) => {
                            map_content.pop_pos(Matrix4::from_translation(vec3(
                                x as f32 * (tile_size.x) + 1.0,
                                0.0 + 5.45,
                                y as f32 * (tile_size.z),
                            )));
                        }
                        Content::Building => {
                            map_content.pop_pos(Matrix4::from_translation(vec3(
                                x as f32 * (tile_size.x),
                                0.0 + 5.45,
                                y as f32 * (tile_size.z),
                            )));
                        }
                        Content::Bush(_) => {
                            map_content.pop_pos(Matrix4::from_translation(vec3(
                                x as f32 * (tile_size.x),
                                0.0 + 5.45,
                                y as f32 * (tile_size.z),
                            )));
                        }
                        _ => {
                            map_content.pop_pos(Matrix4::from_translation(vec3(
                                x as f32 * (tile_size.x),
                                0.0,
                                y as f32 * (tile_size.z),
                            )));
                        }
                    }
                }

                // check if content change
                if let Some(map_content) = GLOBAL_3D_CONTENT
                    .as_mut()
                    .unwrap()
                    .get_mut(&tile.content.clone().to_default())
                {
                    if let Some(t) = equal_content.get_mut(&tile.content.clone().to_default()) {
                        *t = false;
                    }
                    // push the new content position, need modification because hill is not equal to other tile
                    match &tile.content.clone().to_default() {
                        Content::Rock(_) => {
                            map_content.push_pos(
                                Matrix4::from_translation(vec3(
                                    x as f32 * (tile_size.x) + 1.5,
                                    0.0 + 6.3,
                                    y as f32 * (tile_size.z) - 2.0,
                                )) * Mat4::from_scale(0.5),
                            );
                        }
                        Content::Tree(_) => {
                            map_content.push_pos(
                                Matrix4::from_translation(vec3(
                                    x as f32 * (tile_size.x) + 2.3,
                                    0.0 + 6.15,
                                    y as f32 * (tile_size.z) - 2.0,
                                )) * Mat4::from_scale(0.6),
                            );
                        }
                        Content::Garbage(_) => {
                            map_content.push_pos(Matrix4::from_translation(vec3(
                                x as f32 * (tile_size.x) + 1.0,
                                0.0 + 5.45,
                                y as f32 * (tile_size.z) - 1.5,
                            )));
                        }
                        Content::Fire => {
                            map_content.push_pos(Matrix4::from_translation(vec3(
                                x as f32 * (tile_size.x),
                                0.0 + 5.45,
                                y as f32 * (tile_size.z),
                            )));
                        }
                        Content::Coin(_) => {
                            map_content.push_pos(Matrix4::from_translation(vec3(
                                x as f32 * (tile_size.x),
                                0.0 + 5.45,
                                y as f32 * (tile_size.z) + 3.0,
                            )));
                        }
                        Content::Bin(_) => {
                            map_content.push_pos(Matrix4::from_translation(vec3(
                                x as f32 * (tile_size.x) + 0.5,
                                0.0 + 5.45,
                                y as f32 * (tile_size.z) + 0.5,
                            )));
                        }
                        Content::Crate(_) => {
                            map_content.push_pos(Matrix4::from_translation(vec3(
                                x as f32 * (tile_size.x) + 1.0,
                                0.0 + 5.45,
                                y as f32 * (tile_size.z),
                            )));
                        }
                        Content::Building => {
                            map_content.push_pos(Matrix4::from_translation(vec3(
                                x as f32 * (tile_size.x),
                                0.0 + 5.45,
                                y as f32 * (tile_size.z),
                            )));
                        }
                        Content::Bush(_) => {
                            map_content.push_pos(Matrix4::from_translation(vec3(
                                x as f32 * (tile_size.x),
                                0.0 + 5.45,
                                y as f32 * (tile_size.z),
                            )));
                        }
                        _ => {
                            map_content.push_pos(Matrix4::from_translation(vec3(
                                x as f32 * (tile_size.x),
                                0.0,
                                y as f32 * (tile_size.z),
                            )));
                        }
                    }
                }
            }
            unsafe {
                // update only if something change to avoid useless resource usage and lag
                for c in Content::iter() {
                    if equal_content[&c.to_default()] {
                        continue;
                    } else {
                        if let Some(context) = GLOBAL_CONTEXT.as_ref() {
                            if let Some(mc) =
                                GLOBAL_3D_CONTENT.as_mut().unwrap().get_mut(&c.to_default())
                            {
                                mc.update_istances(context);
                            }
                        } else {
                            println!("Option is None");
                        }
                    }
                }
            }
        }
        Mountain => {
            unsafe {
                // check if content change
                if let Some(t) = equal_content.get_mut(
                    &PAST_WORLD[x][y]
                        .as_ref()
                        .unwrap_or(&Tile {
                            tile_type: Grass,
                            content: Content::None,
                            elevation: 0,
                        })
                        .content
                        .clone()
                        .to_default(),
                ) {
                    *t = false;
                }
                if let Some(map_content) = GLOBAL_3D_CONTENT.as_mut().unwrap().get_mut(
                    &PAST_WORLD[x][y]
                        .as_ref()
                        .unwrap_or(&Tile {
                            tile_type: Grass,
                            content: Content::None,
                            elevation: 0,
                        })
                        .content
                        .clone()
                        .to_default(),
                ) {
                    // pop the content position, need modification because mountain is not equal to other tile
                    match &PAST_WORLD[x][y]
                        .as_ref()
                        .unwrap_or(&Tile {
                            tile_type: Grass,
                            content: Content::None,
                            elevation: 0,
                        })
                        .content
                        .clone()
                        .to_default()
                    {
                        Content::Rock(_) => {
                            map_content.pop_pos(
                                Matrix4::from_translation(vec3(
                                    x as f32 * (tile_size.x) + 2.5,
                                    0.0 + 9.9,
                                    y as f32 * (tile_size.z) - 3.0,
                                )) * Mat4::from_scale(0.5),
                            );
                        }
                        Content::Tree(_) => {
                            map_content.pop_pos(
                                Matrix4::from_translation(vec3(
                                    x as f32 * (tile_size.x) + 2.3,
                                    0.0 + 9.7,
                                    y as f32 * (tile_size.z) - 2.0,
                                )) * Mat4::from_scale(0.6),
                            );
                        }
                        Content::Garbage(_) => {
                            map_content.pop_pos(Matrix4::from_translation(vec3(
                                x as f32 * (tile_size.x) + 1.5,
                                0.0 + 9.0,
                                y as f32 * (tile_size.z) - 2.5,
                            )));
                        }
                        Content::Fire => {
                            map_content.pop_pos(Matrix4::from_translation(vec3(
                                x as f32 * (tile_size.x) + 0.5,
                                0.0 + 9.0,
                                y as f32 * (tile_size.z) - 1.0,
                            )));
                        }
                        Content::Coin(_) => {
                            map_content.pop_pos(Matrix4::from_translation(vec3(
                                x as f32 * (tile_size.x) + 0.5,
                                0.0 + 9.0,
                                y as f32 * (tile_size.z) + 3.0,
                            )));
                        }
                        Content::Bin(_) => {
                            map_content.pop_pos(Matrix4::from_translation(vec3(
                                x as f32 * (tile_size.x) + 1.5,
                                0.0 + 9.0,
                                y as f32 * (tile_size.z) + 1.5,
                            )));
                        }
                        Content::Crate(_) => {
                            map_content.pop_pos(Matrix4::from_translation(vec3(
                                x as f32 * (tile_size.x) + 2.0,
                                0.0 + 9.0,
                                y as f32 * (tile_size.z) + 1.0,
                            )));
                        }
                        Content::Building => {
                            map_content.pop_pos(Matrix4::from_translation(vec3(
                                x as f32 * (tile_size.x),
                                0.0 + 9.0,
                                y as f32 * (tile_size.z),
                            )));
                        }
                        Content::Bush(_) => {
                            map_content.pop_pos(Matrix4::from_translation(vec3(
                                x as f32 * (tile_size.x),
                                0.0 + 9.0,
                                y as f32 * (tile_size.z),
                            )));
                        }
                        _ => {
                            map_content.pop_pos(Matrix4::from_translation(vec3(
                                x as f32 * (tile_size.x),
                                0.0,
                                y as f32 * (tile_size.z),
                            )));
                        }
                    }
                }
                if let Some(map_content) = GLOBAL_3D_CONTENT
                    .as_mut()
                    .unwrap()
                    .get_mut(&tile.content.clone().to_default())
                {
                    if let Some(t) = equal_content.get_mut(&tile.content.clone().to_default()) {
                        *t = false;
                    }
                    // push the new content position, need modification because mountain is not equal to other tile
                    match &tile.content.clone().to_default() {
                        Content::Rock(_) => {
                            map_content.push_pos(
                                Matrix4::from_translation(vec3(
                                    x as f32 * (tile_size.x) + 2.5,
                                    0.0 + 9.9,
                                    y as f32 * (tile_size.z) - 3.0,
                                )) * Mat4::from_scale(0.5),
                            );
                        }
                        Content::Tree(_) => {
                            map_content.push_pos(
                                Matrix4::from_translation(vec3(
                                    x as f32 * (tile_size.x) + 2.3,
                                    0.0 + 9.7,
                                    y as f32 * (tile_size.z) - 2.0,
                                )) * Mat4::from_scale(0.6),
                            );
                        }
                        Content::Garbage(_) => {
                            map_content.push_pos(Matrix4::from_translation(vec3(
                                x as f32 * (tile_size.x) + 1.5,
                                0.0 + 9.0,
                                y as f32 * (tile_size.z) - 2.5,
                            )));
                        }
                        Content::Fire => {
                            map_content.push_pos(Matrix4::from_translation(vec3(
                                x as f32 * (tile_size.x),
                                0.5 + 9.0,
                                y as f32 * (tile_size.z) - 1.0,
                            )));
                        }
                        Content::Coin(_) => {
                            map_content.push_pos(Matrix4::from_translation(vec3(
                                x as f32 * (tile_size.x) + 0.5,
                                0.0 + 9.0,
                                y as f32 * (tile_size.z) + 3.0,
                            )));
                        }
                        Content::Bin(_) => {
                            map_content.push_pos(Matrix4::from_translation(vec3(
                                x as f32 * (tile_size.x) + 1.5,
                                0.0 + 9.0,
                                y as f32 * (tile_size.z) + 1.5,
                            )));
                        }
                        Content::Crate(_) => {
                            map_content.push_pos(Matrix4::from_translation(vec3(
                                x as f32 * (tile_size.x) + 2.0,
                                0.0 + 9.0,
                                y as f32 * (tile_size.z) + 1.0,
                            )));
                        }
                        Content::Building => {
                            map_content.push_pos(Matrix4::from_translation(vec3(
                                x as f32 * (tile_size.x),
                                0.0 + 9.0,
                                y as f32 * (tile_size.z),
                            )));
                        }
                        Content::Bush(_) => {
                            map_content.push_pos(Matrix4::from_translation(vec3(
                                x as f32 * (tile_size.x),
                                0.0 + 9.0,
                                y as f32 * (tile_size.z),
                            )));
                        }
                        _ => {
                            map_content.push_pos(Matrix4::from_translation(vec3(
                                x as f32 * (tile_size.x),
                                0.0,
                                y as f32 * (tile_size.z),
                            )));
                        }
                    }
                }
            }
            unsafe {
                // update only if something change to avoid useless resource usage and lag
                for c in Content::iter() {
                    if equal_content[&c.to_default()] {
                        continue;
                    } else {
                        if let Some(context) = GLOBAL_CONTEXT.as_ref() {
                            if let Some(mc) =
                                GLOBAL_3D_CONTENT.as_mut().unwrap().get_mut(&c.to_default())
                            {
                                mc.update_istances(context);
                            }
                        } else {
                            println!("Option is None");
                        }
                    }
                }
            }
        }
        // Teleport(_) => {}
        // Wall => {}
        _ => unsafe {
            // if the tile is update to a Street, I can't know it
            // but I can delete the possible object to be safe
            if tile.tile_type == Street {
                for c in Content::iter() {
                    remove_up_content(
                        Tile {
                            tile_type: Mountain,
                            content: c.clone().to_default(),
                            elevation: 0,
                        },
                        (x, y),
                        GLOBAL_3D_CONTENT.as_mut().unwrap(),
                    );
                    remove_up_content(
                        Tile {
                            tile_type: Hill,
                            content: c.to_default(),
                            elevation: 0,
                        },
                        (x, y),
                        GLOBAL_3D_CONTENT.as_mut().unwrap(),
                    );
                }
            }
            unsafe {
                // check if content change
                if let Some(t) = equal_content.get_mut(
                    &PAST_WORLD[x][y]
                        .as_ref()
                        .unwrap_or(&Tile {
                            tile_type: Grass,
                            content: Content::None,
                            elevation: 0,
                        })
                        .content
                        .clone()
                        .to_default(),
                ) {
                    *t = false;
                }
                // remove the content
                if let Some(map_content) = GLOBAL_3D_CONTENT.as_mut().unwrap().get_mut(
                    &PAST_WORLD[x][y]
                        .as_ref()
                        .unwrap_or(&Tile {
                            tile_type: Grass,
                            content: Content::None,
                            elevation: 0,
                        })
                        .content
                        .clone()
                        .to_default(),
                ) {
                    map_content.pop_pos(Matrix4::from_translation(vec3(
                        x as f32 * (tile_size.x),
                        0.0,
                        y as f32 * (tile_size.z),
                    )));
                }
            }
            // push the new content (it could be the same of before)
            if let Some(map_content) = GLOBAL_3D_CONTENT
                .as_mut()
                .unwrap()
                .get_mut(&tile.content.clone().to_default())
            {
                if let Some(t) = equal_content.get_mut(&tile.content.clone().to_default()) {
                    *t = false;
                }
                map_content.push_pos(Matrix4::from_translation(vec3(
                    x as f32 * (tile_size.x),
                    0.0,
                    y as f32 * (tile_size.z),
                )));
            }
            // update only if something change to avoid useless resource usage and lag
            unsafe {
                for c in Content::iter() {
                    if *equal_content.get(&c.to_default()).unwrap() {
                        continue;
                    } else {
                        if let Some(context) = GLOBAL_CONTEXT.as_ref() {
                            if let Some(mc) =
                                GLOBAL_3D_CONTENT.as_mut().unwrap().get_mut(&c.to_default())
                            {
                                mc.update_istances(context);
                            }
                        } else {
                            println!("Option is None");
                        }
                    }
                }
            }
        },
    }
}

pub fn change_time(t: DayTime, tt: String) {
    unsafe {
        TIME = t;
        EXACT_TIME = tt;
    }
}

pub fn change_weather(w: WeatherType) {
    unsafe {
        WEATHER = w;
    }
}

pub fn future_weather(w: WeatherType) {
    unsafe {
        FUTURE_WEATHER = w;
    }
}

pub fn score_update(s: f32) {
    unsafe {
        GLOBAL_SCORE = s;
    }
}
