use crate::alessandro_gui::generator::create_robot_model;
use crate::alessandro_gui::main::{ENERGY, INVENTORY, ROBOT_MODEL, ROBOT_POS};
use robotics_lib::world::tile::TileType::{Hill, Mountain};
use robotics_lib::world::tile::{Content, TileType};
use std::collections::HashMap;
use strum::IntoEnumIterator;
use three_d::{vec3, Deg, Rad, SquareMatrix};
use three_d_asset::Mat4;

pub unsafe fn set_robot_start_pos() {
    // set the pos in the map, so if you don't move
    // but only render the robot is in the correct position and not in (0,0)
    ROBOT_MODEL.as_mut().unwrap().iter_mut().for_each(|m| {
        m.geometry.set_transformation(Mat4::from_translation(vec3(
            12.0 * ROBOT_POS.0 as f32,
            0.0,
            12.0 * ROBOT_POS.1 as f32,
        )));
    });
}

pub fn adjust_robot_height(tile_type: TileType) {
    // adjust the height of the model (for example moving from mountain to hill or ground
    match tile_type {
        Hill => unsafe {
            if ROBOT_POS.2 == 1 {
                return; // already at the correct height
            } else if ROBOT_POS.2 == 2 {
                // at mountain level
                ROBOT_MODEL.as_mut().unwrap().iter_mut().for_each(|m| {
                    let t = m.geometry.transformation()
                        * Mat4::from_translation(vec3(0.0, 9.0, 0.0))
                            .invert()
                            .unwrap();
                    m.geometry
                        .set_transformation(t * Mat4::from_translation(vec3(0.0, 5.45, 0.0)));
                })
            } else {
                // at ground level
                ROBOT_MODEL.as_mut().unwrap().iter_mut().for_each(|m| {
                    let t = m.geometry.transformation();
                    m.geometry
                        .set_transformation(t * Mat4::from_translation(vec3(0.0, 5.45, 0.0)));
                })
            }
        },
        Mountain => {
            unsafe {
                if ROBOT_POS.2 == 2 {
                    return; // already at the correct height
                } else if ROBOT_POS.2 == 0 {
                    // at ground level
                    ROBOT_MODEL.as_mut().unwrap().iter_mut().for_each(|m| {
                        let t = m.geometry.transformation();
                        m.geometry
                            .set_transformation(t * Mat4::from_translation(vec3(0.0, 9.0, 0.0)));
                    })
                } else {
                    // at hill level
                    ROBOT_MODEL.as_mut().unwrap().iter_mut().for_each(|m| {
                        let t = m.geometry.transformation()
                            * Mat4::from_translation(vec3(0.0, 5.45, 0.0))
                                .invert()
                                .unwrap();
                        m.geometry
                            .set_transformation(t * Mat4::from_translation(vec3(0.0, 9.0, 0.0)));
                    })
                }
            }
        }
        _ => {}
    }
}

pub fn move_robot(x: usize, y: usize) {
    // update the pos of the robot model and rotate according to his direction of movement
    // use the ROBOT_POS to know where the robot was
    unsafe {
        ROBOT_MODEL.as_mut().unwrap().iter_mut().for_each(|m| {
            if (ROBOT_POS.1 as i32 - y as i32) == 0 && x as i32 - (ROBOT_POS.0 as i32) < 0 {
                m.geometry.set_transformation(Mat4::from_translation(vec3(
                    12.0 * x as f32,
                    0.0,
                    12.0 * y as f32,
                )));
            } else if (ROBOT_POS.1 as i32 - y as i32) == 0 && x as i32 - ROBOT_POS.0 as i32 >= 1 {
                m.geometry.set_transformation(
                    Mat4::from_angle_y(Rad::from(Deg(180.0)))
                        * Mat4::from_translation(vec3(-12.0, 0.0, 12.0))
                        * Mat4::from_translation(vec3(-12.0 * x as f32, 0.0, -12.0 * y as f32)),
                );
            } else {
                if y as f32 - ROBOT_POS.1 as f32 >= 1.0 {
                    m.geometry.set_transformation(
                        Mat4::from_angle_y(Rad::from(Deg(90.0)))
                            * Mat4::from_translation(vec3(0.0, 0.0, 12.0))
                            * Mat4::from_translation(vec3(-12.0 * y as f32, 0.0, 12.0 * x as f32)),
                    );
                } else {
                    m.geometry.set_transformation(
                        Mat4::from_angle_y(Rad::from(Deg(-90.0)))
                            * Mat4::from_translation(vec3(-12.0, 0.0, 0.0))
                            * Mat4::from_translation(vec3(12.0 * y as f32, 0.0, -12.0 * x as f32)),
                    );
                }
            }
        });
        ROBOT_POS = (x, y, 0);
    }
}

pub fn set_initial_config(x: usize, y: usize) {
    unsafe {
        ROBOT_POS = (x, y, 0);
        // set_robot_start_pos();
        INVENTORY = Some(HashMap::new());
        for c in Content::iter() {
            INVENTORY.as_mut().unwrap().insert(c.to_default(), 0);
        }
    }
}

pub fn add_energy(e: usize) {
    unsafe {
        if ENERGY < 1000 {
            ENERGY += e;
            if ENERGY > 1000 {
                ENERGY = 1000;
            }
        }
    }
}

pub fn sub_energy(e: usize) {
    unsafe {
        if !ENERGY.overflowing_sub(e).1 {
            ENERGY -= e;
        }
    }
}

pub fn add_to_backpack(content: &Content, amount: usize) {
    unsafe {
        if let Some(q) = INVENTORY.as_mut().unwrap().get_mut(content) {
            *q += amount;
        }
    }
}

pub fn sub_to_backpack(content: &Content, amount: usize) {
    unsafe {
        if let Some(q) = INVENTORY.as_mut().unwrap().get_mut(content) {
            *q -= amount;
        }
    }
}
