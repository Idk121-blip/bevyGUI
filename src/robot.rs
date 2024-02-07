use crate::MapEvent;

use crate::EVENT;
use crate::PLOTUPDATE;

use crate::ROBOT_COL;
use crate::ROBOT_ROW;

use rand::Rng;
use robotics_lib::energy::Energy;
use robotics_lib::event::events::Event;
use robotics_lib::interface::{destroy, go, one_direction_view, Direction};
use robotics_lib::interface::{robot_map, where_am_i};
use robotics_lib::runner::backpack::BackPack;
use robotics_lib::runner::{Robot, Runnable};
use robotics_lib::world::coordinates::Coordinate;
use robotics_lib::world::environmental_conditions::EnvironmentalConditions;
use robotics_lib::world::environmental_conditions::WeatherType::{Rainy, Sunny};
use robotics_lib::world::tile::Content::{
    Bank, Bin, Coin, Crate, Fire, Fish, Garbage, Market, Rock, Tree,
};
use robotics_lib::world::tile::TileType;
use robotics_lib::world::tile::TileType::*;

use robotics_lib::world::tile::{Content, Tile};
use robotics_lib::world::world_generator::Generator;
use robotics_lib::world::World;
use std::collections::HashMap;
use strum::IntoEnumIterator;
pub(crate) struct MyRobot(pub Robot);
impl Runnable for MyRobot {
    fn process_tick(&mut self, world: &mut World) {
        let (_robot_view, robot_position) = where_am_i(self, world);

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
        let _ = go(self, world, Direction::Down);
        let _ = one_direction_view(self, world, Direction::Right, 4);
        let _ = destroy(self, world, Direction::Right);
    }

    fn handle_event(&mut self, event: Event) {
        println!();
        println!("{:?}", event);
        println!();
    }

    fn get_energy(&self) -> &Energy {
        &self.0.energy
    }

    fn get_energy_mut(&mut self) -> &mut Energy {
        &mut self.0.energy
    }

    fn get_coordinate(&self) -> &Coordinate {
        &self.0.coordinate
    }

    fn get_coordinate_mut(&mut self) -> &mut Coordinate {
        &mut self.0.coordinate
    }

    fn get_backpack(&self) -> &BackPack {
        &self.0.backpack
    }

    fn get_backpack_mut(&mut self) -> &mut BackPack {
        &mut self.0.backpack
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
                    3 => {
                        row.push(Tile {
                            tile_type: Grass,
                            content: Coin(1),
                            elevation: 0,
                        });
                    }
                    4 => {
                        row.push(Tile {
                            tile_type: ShallowWater,
                            content: Content::None,
                            elevation: 0,
                        });
                    }
                    _ => {
                        row.push(Tile {
                            tile_type: Sand,
                            content: Coin(1),
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

        (map, (17, 1), environmental_conditions, max_score, None)
    }
}
