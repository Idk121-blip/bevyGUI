use crate::MapEvent;

use crate::ENVIRONMENT;
use crate::EVENT;
use crate::PLOTUPDATE;

use crate::ROBOT_COL;
use crate::ROBOT_ROW;

use ohcrab_weather::weather_tool::WeatherPredictionTool;
use rand::Rng;
use robotics_lib::energy::Energy;
use robotics_lib::event::events::Event;
use robotics_lib::interface::{destroy, go, look_at_sky, one_direction_view, Direction};
use robotics_lib::interface::{robot_map, where_am_i};
use robotics_lib::runner::backpack::BackPack;
use robotics_lib::runner::{Robot, Runnable};
use robotics_lib::world::coordinates::Coordinate;
use robotics_lib::world::environmental_conditions::EnvironmentalConditions;
use robotics_lib::world::environmental_conditions::WeatherType::{Rainy, Sunny};
use robotics_lib::world::tile::Content::Coin;
use robotics_lib::world::tile::TileType;
use robotics_lib::world::tile::TileType::*;

use oxagaudiotool::OxAgAudioTool;
use robotics_lib::world::tile::{Content, Tile};
use robotics_lib::world::world_generator::Generator;
use robotics_lib::world::World;
use std::collections::HashMap;
use strum::IntoEnumIterator;
pub(crate) struct MyRobot {
    pub robot: Robot,
    //audio: OxAgAudioTool,
}
impl Runnable for MyRobot {
    fn process_tick(&mut self, world: &mut World) {
        let mut down = false;
        match go(self, world, Direction::Up) {
            Ok(_) => {}
            Err(_) => {
                down = true;
            }
        }
        if down {
            go(self, world, Direction::Down);
        }
        let mut right = false;
        match go(self, world, Direction::Left) {
            Ok(_) => {}
            Err(_) => {
                right = true;
            }
        }
        if right {
            go(self, world, Direction::Right);
        }

        let (_robot_view, robot_position) = where_am_i(self, world);
        //todo! rename

        {
            let mut env = ENVIRONMENT.lock().unwrap();
            *env = Some(look_at_sky(&world));
        }

        let v = robot_map(world);

        let mut plot_update = PLOTUPDATE.lock().unwrap();
        let mut events = EVENT.lock().unwrap();
        let mut robot_col = ROBOT_COL.lock().unwrap();
        let mut robot_row = ROBOT_ROW.lock().unwrap();
        if robot_position.0 != *robot_col || *robot_row != robot_position.1 {
            //todo! add event into an array of event
            events.push(MapEvent::UpdateRobot);
            *robot_col = robot_position.0;
            *robot_row = robot_position.1;
        }
        let mut update_map = false;
        // let mut future_weater_tool = WeatherPredictionTool::new();
        // future_weater_tool.process_event();
        // let future_weather = predict(, 2);
        match v {
            None => {}
            Some(v) => {
                //todo! if something is changed than update the tile or content
                for r in 0..v.len() {
                    for c in 0..v[0].len() {
                        if !v[r][c].is_none() {
                            if plot_update[r][c].is_none() {
                                update_map = true;
                                plot_update[r][c] = v[r][c].clone();
                            } else if plot_update[r][c].as_ref().unwrap().tile_type
                                != v[r][c].as_ref().unwrap().tile_type
                            {
                                update_map = true;
                                plot_update[r][c] = v[r][c].clone();
                            } else if plot_update[r][c].as_ref().unwrap().content
                                != v[r][c].as_ref().unwrap().content
                            {
                                update_map = true;
                                plot_update[r][c].as_mut().unwrap().content =
                                    v[r][c].clone().unwrap().content;
                            }
                        }
                    }
                }
            }
        }

        if update_map {
            events.push(MapEvent::UpdateMap);
        }
    }

    fn handle_event(&mut self, event: Event) {
        //let _ = self.audio.play_audio_based_on_event(&event);
        println!();
        println!("{:?}", event);
        println!();
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

pub(crate) struct WorldGenerator {
    size: usize,
}

impl WorldGenerator {
    pub(crate) fn init(size: usize) -> Self {
        WorldGenerator { size }
    }
}

impl Generator for WorldGenerator {
    fn gen(
        &mut self,
    ) -> (
        Vec<Vec<Tile>>,
        (usize, usize),
        EnvironmentalConditions,
        f32,
        Option<HashMap<Content, f32>>,
    ) {
        let mut rng = rand::thread_rng();
        let mut map: Vec<Vec<Tile>> = Vec::new();
        // Initialize the map with default tiles
        for _ in 0..self.size {
            let mut row: Vec<Tile> = Vec::new();
            for _ in 0..self.size {
                let i_tiletype = rng.gen_range(0..TileType::iter().len());
                let _i_content = rng.gen_range(0..Content::iter().len());
                match i_tiletype {
                    300 => {
                        row.push(Tile {
                            tile_type: Grass,
                            content: Coin(1),
                            elevation: 0,
                        });
                    }
                    5 => {
                        row.push(Tile {
                            tile_type: Sand,
                            content: Coin(1),
                            elevation: 0,
                        });
                    }
                    _ => {
                        row.push(Tile {
                            tile_type: ShallowWater,
                            content: Content::None,
                            elevation: 0,
                        });
                    }
                };
            }
            map.push(row);
        }
        let environmental_conditions =
            EnvironmentalConditions::new(&[Sunny, Rainy], 15, 12).unwrap();

        let max_score = rand::random::<f32>();

        (map, (50, 50), environmental_conditions, max_score, None)
    }
}
