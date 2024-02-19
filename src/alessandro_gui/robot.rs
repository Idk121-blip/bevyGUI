use rand::Rng;
use robotics_lib;
use robotics_lib::energy::Energy;
use robotics_lib::event::events::Event;
use robotics_lib::interface::{
    destroy, discover_tiles, get_score, go, put, robot_map, where_am_i, Direction, Tools,
};
use robotics_lib::runner::backpack::BackPack;
use robotics_lib::runner::{Robot, Runnable, Runner};
use robotics_lib::utils::LibError::*;
use robotics_lib::utils::{in_bounds, LibError};
use robotics_lib::world::coordinates::Coordinate;
use robotics_lib::world::environmental_conditions::WeatherType::{
    Foggy, Rainy, Sunny, TrentinoSnow, TropicalMonsoon,
};
use robotics_lib::world::environmental_conditions::{DayTime, EnvironmentalConditions};
use robotics_lib::world::tile::Content::{
    Bank, Bin, Coin, Crate, Fire, Fish, Garbage, Market, Rock, Tree,
};
use robotics_lib::world::tile::TileType::*;
use robotics_lib::world::tile::{Content, Tile, TileType};
use robotics_lib::world::world_generator::Generator;
use robotics_lib::world::World;

// ------------ needed --------
use crate::map_update::{change_time, change_weather, robot_around_tile, update_content};
use crate::robot_utils::{move_robot, set_robot_start_pos};
use crate::{
    ENERGY, FUTURE_WEATHER, GLOBAL_3D_CONTENT, GLOBAL_3D_TILES, GLOBAL_SCORE, INVENTORY, ROBOT_POS,
};
use ohcrab_weather::weather_tool::WeatherPredictionTool;
use oxagaudiotool::OxAgAudioTool;
use std::collections::HashMap;
use strum::IntoEnumIterator;
// ------------------------------

pub(crate) struct Tool {}
pub enum State {
    GET_STONES,
    MAKE_ROAD,
}
#[derive(Debug)]
pub enum RpmError {
    NotEnoughEnergy,
    CannotPlaceHere,
    OutOfBounds,
    NotEnoughMaterial,
    NoRockHere,
    MustDestroyContentFirst,
    UndefinedError,
}
impl Tool {
    fn error_handing(error: Result<usize, LibError>) -> Result<(), RpmError> {
        match error {
            Ok(_) => Ok(()),
            Err(e) => match e {
                NotEnoughEnergy => Err(RpmError::NotEnoughEnergy),
                OutOfBounds => Err(RpmError::OutOfBounds),
                NotEnoughContentInBackPack => Err(RpmError::NotEnoughMaterial),
                ContentNotAllowedOnTile => Err(RpmError::CannotPlaceHere),
                MustDestroyContentFirst => Err(RpmError::MustDestroyContentFirst),
                NoContent => Err(RpmError::NoRockHere),
                NotEnoughContentProvided => Err(RpmError::NotEnoughMaterial),
                _ => Err(RpmError::UndefinedError),
            },
        }
    }
    pub fn road_paving_machine(
        robot: &mut impl Runnable,
        world: &mut World,
        direction: Direction,
        state: State,
    ) -> Result<(), RpmError> {
        where_am_i(robot, world);
        if robot
            .get_backpack()
            .get_contents()
            .get(&Content::Rock(0))
            .is_none()
        {
            return Err(RpmError::NotEnoughMaterial);
        }

        let tile: Tile;
        match in_bounds(robot, world, &direction) {
            Ok(_) => {}
            Err(_) => return Err(RpmError::OutOfBounds),
        }

        let mut col = robot.get_coordinate().get_col();
        let mut row = robot.get_coordinate().get_row();
        match direction {
            Direction::Up => {
                row -= 1;
            }
            Direction::Down => {
                row += 1;
            }
            Direction::Left => {
                col -= 1;
            }
            Direction::Right => {
                col += 1;
            }
        }

        match robot_map(world) {
            None => {
                return Err(RpmError::OutOfBounds);
            }
            Some(map) => {
                tile = map[row][col].clone().unwrap();
            }
        }

        match state {
            State::GET_STONES => {
                let error = destroy(robot, world, direction);
                return Self::error_handing(error);
            }
            State::MAKE_ROAD => match tile.tile_type {
                DeepWater => {
                    let error = put(robot, world, Content::Rock(0), 3, direction);
                    return Self::error_handing(error);
                }
                ShallowWater => {
                    let error = put(robot, world, Content::Rock(0), 2, direction);
                    return Self::error_handing(error);
                }
                Sand => {
                    let error = put(robot, world, Content::Rock(0), 1, direction);
                    return Self::error_handing(error);
                }
                Grass => {
                    let error = put(robot, world, Content::Rock(0), 1, direction);
                    return Self::error_handing(error);
                }
                Street => {
                    return Err(RpmError::CannotPlaceHere);
                }
                Hill => {
                    let error = put(robot, world, Content::Rock(0), 1, direction);
                    return Self::error_handing(error);
                }
                Mountain => {
                    let error = put(robot, world, Content::None, 0, direction);
                    return Self::error_handing(error);
                }
                Snow => {
                    let error = put(robot, world, Content::Rock(0), 1, direction);
                    return Self::error_handing(error);
                }
                Lava => {
                    let error = put(robot, world, Content::Rock(0), 3, direction);
                    return Self::error_handing(error);
                }
                Teleport(_) => {
                    return Err(RpmError::CannotPlaceHere);
                }
                Wall => {
                    return Err(RpmError::CannotPlaceHere);
                }
            },
        }
    }
}

impl Tools for Tool {}

pub struct MyRobot {
    pub(crate) robot: Robot,
    pub(crate) audio: OxAgAudioTool,
    pub(crate) weather: WeatherPredictionTool,
}

impl Runnable for MyRobot {
    fn process_tick(&mut self, world: &mut World) {
        // ---------------------------------
        unsafe {
            robot_around_tile(
                robot_map(world).unwrap_or(vec![vec![None]]),
                &mut GLOBAL_3D_TILES,
            )
        };
        // ---------------------------------
        //     let q = where_am_i(self, world);
        //     if q.1 == (2, 2) {
        //         go(self, world, Direction::Left);
        //         // d = 2;
        //     } else if q.1 == (2, 1) {
        //         go(self, world, Direction::Up);
        //         // Tool::road_paving_machine(self, world, Direction::Left,State::GET_STONES);
        //         // d= 3;
        //     } else if q.1 == (1, 1) {
        //         go(self, world, Direction::Right);
        //     } else if q.1 == (1, 2) {
        //         go(self, world, Direction::Down);
        //     }
        // go(self, world, Direction::Down);
        // go(self, world, Direction::Right);
        // println!(
        //     "CCP {:?}",
        //     crate::robot::Tool::road_paving_machine(self, world, Direction::Right, State::GET_STONES)
        // );
        // println!(
        //     "CUP {:?}",
        //     crate::robot::Tool::road_paving_machine(self, world, Direction::Down, State::MAKE_ROAD)
        // );
        if go(self, world, Direction::Down).is_err() {
            if go(self, world, Direction::Left).is_err() {
                if go(self, world, Direction::Right).is_err() {
                    if go(self, world, Direction::Up).is_err() {}
                }
            }
        }
        if go(self, world, Direction::Left).is_err() {
            if go(self, world, Direction::Down).is_err() {
                if go(self, world, Direction::Right).is_err() {
                    if go(self, world, Direction::Up).is_err() {}
                }
            }
        }
        // println!("{:?}",world);
        // discover_tiles(self, world, &[(10usize, 10usize)]);
        // ---------------------------------
        unsafe {
            let t = self.weather.ticks_until_weather_change(1000);
            if let Ok(tt) = t {
                future_weather()
                //   FUTURE_WEATHER = self.weather.predict(tt).unwrap_or(Sunny);
            }
            robot_around_tile(robot_map(world).unwrap_or(vec![vec![None]]));
            score_update()
            //GLOBAL_SCORE = get_score(&world);
        }
        // --------------------------------
    }

    fn handle_event(&mut self, event: robotics_lib::event::events::Event) {
        self.weather.process_event(&event);
        let _ = self.audio.play_audio_based_on_event(&event);
        match event {
            robotics_lib::event::events::Event::Ready => {
                set_initial_config(
                    self.get_coordinate().get_row(),
                    self.get_coordinate().get_col(),
                )
                /*unsafe {
                    ROBOT_POS = (
                        self.get_coordinate().get_row(),
                        self.get_coordinate().get_col(),
                        0,
                    );
                    // set_robot_start_pos();
                    INVENTORY = Some(HashMap::new());
                    for c in Content::iter() {
                        INVENTORY.as_mut().unwrap().insert(c.to_default(), 0);
                    }
                }*/
            }
            robotics_lib::event::events::Event::Terminated => {}
            robotics_lib::event::events::Event::TimeChanged(x) => {
                change_time(x.get_time_of_day(), x.get_time_of_day_string());
                change_weather(x.get_weather_condition());
            }
            robotics_lib::event::events::Event::DayChanged(x) => {
                change_time(x.get_time_of_day(), x.get_time_of_day_string());
                change_weather(x.get_weather_condition());
            }
            robotics_lib::event::events::Event::EnergyRecharged(x) => unsafe {
                if ENERGY < 1000 {
                    ENERGY += x;
                    if ENERGY > 1000 {
                        ENERGY = 1000;
                    }
                }
            },
            robotics_lib::event::events::Event::EnergyConsumed(x) => unsafe {
                if !ENERGY.overflowing_sub(x).1 {
                    ENERGY -= x;
                }
            },
            robotics_lib::event::events::Event::Moved(tile, (x, y)) => {
                move_robot(x, y);
            }
            robotics_lib::event::events::Event::TileContentUpdated(tile, c) => unsafe {
                update_content(tile, c, &mut GLOBAL_3D_CONTENT.as_mut().unwrap())
            },
            robotics_lib::event::events::Event::AddedToBackpack(content, amount) => unsafe {
                add_to_backpack(&content, amount)
            },
            robotics_lib::event::events::Event::RemovedFromBackpack(content, amount) => unsafe {
                sub_to_backpack(&content, amount)
            },
        }
    }

    fn get_energy(&self) -> &Energy {
        &self.robot.energy
    }

    fn get_energy_mut(&mut self) -> &mut Energy {
        &mut self.robot.energy
    }

    fn get_coordinate(&self) -> &Coordinate {
        &self.robot.coordinate
    }

    fn get_coordinate_mut(&mut self) -> &mut Coordinate {
        &mut self.robot.coordinate
    }

    fn get_backpack(&self) -> &BackPack {
        &self.robot.backpack
    }

    fn get_backpack_mut(&mut self) -> &mut BackPack {
        &mut self.robot.backpack
    }
}

// pub(crate) struct WorldGenerator {
//     size: usize,
// }
//
// impl WorldGenerator {
//     pub(crate) fn init(size: usize) -> Self {
//         WorldGenerator { size }
//     }
// }
//
// impl Generator for WorldGenerator {
//     fn gen(
//         &mut self,
//     ) -> (
//         Vec<Vec<Tile>>,
//         (usize, usize),
//         EnvironmentalConditions,
//         f32,
//         Option<HashMap<Content, f32>>,
//     ) {
//         let mut rng = rand::thread_rng();
//         let mut map: Vec<Vec<Tile>> = Vec::new();
//         // Initialize the map with default tiles
//         for _ in 0..self.size {
//             let mut row: Vec<Tile> = Vec::new();
//             for _ in 0..self.size {
//                 let i_tiletype = rng.gen_range(0..2);
//                 let i_content = rng.gen_range(0..Content::iter().len());
//                 // let i_content = rng.gen_range(0..3);
//                 let tile_type = match i_tiletype {
//                     // 0 => DeepWater,
//                     // 1 => ShallowWater,
//                     // 2 => Sand,
//                     // 3 => Grass,
//                     // 4 => Street,
//                     0 => Hill,
//                     1 => Mountain,
//                     // 7 => Snow,
//                     // 8 => Lava,
//                     // 9 => Teleport(false),
//                     _ => Grass,
//                 };
//                 let content = match i_content {
//                     0 => Rock(0),
//                     1 => Tree(2),
//                     2 => Garbage(2),
//                     // 3 => Fire,
//                     4 => Coin(2),
//                     5 => Bin(2..3),
//                     6 => Crate(2..3),
//                     // 7 => Bank(3..54),
//                     // 8 => Content::Water(20),
//                     9 => Content::None,
//                     // 10 => Fish(3),
//                     // 11 => Market(20),
//                     12 => Content::Building,
//                     13 => Content::Bush(2),
//                     // 14 => Content::JollyBlock(2),
//                     // 15 => Content::Scarecrow,
//                     _ => Content::None,
//                 };
//                 // row.push(Tile {
//                 //     tile_type: TileType::Grass,
//                 //     content: Content::Rock(10),
//                 //     elevation: 0,
//                 // });
//                 row.push(Tile {
//                     tile_type:TileType::Mountain,
//                     content,
//                     elevation: 0,
//                 })
//             }
//             // row[1] = Tile {
//             //     tile_type: TileType::Lava,
//             //     content: Content::None,
//             //     elevation: 0,
//             // };
//             map.push(row);
//         }
//         map[5][4] = Tile {
//             tile_type:TileType::Wall,
//             content:Content::Fire,
//             elevation: 0,
//         };
//
//
//         let environmental_conditions =
//             EnvironmentalConditions::new(&[Sunny, Rainy, TrentinoSnow, TropicalMonsoon, Foggy], 15, 12).unwrap();
//
//         let max_score = rand::random::<f32>();
// //(1,2)
//         (map, (2, 2), environmental_conditions, max_score, None)
//     }
// }
