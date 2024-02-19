use crate::utils::utils_for_ai::ui_variable_update;
use lazy_static::lazy_static;
use ohcrab_weather::weather_tool::WeatherPredictionTool;
use oxagaudiotool::OxAgAudioTool;
use robotics_lib::energy::Energy;
use robotics_lib::event::events::Event;
use robotics_lib::interface::{
    destroy, go, look_at_sky, one_direction_view, robot_map, where_am_i, Direction,
};
use robotics_lib::runner::backpack::BackPack;
use robotics_lib::runner::{Robot, Runnable};
use robotics_lib::world::coordinates::Coordinate;
use robotics_lib::world::tile::{Content, Tile};
use robotics_lib::world::World;
use std::sync::Mutex;

pub(crate) struct MyRobot {
    pub robot: Robot,
    pub audio: OxAgAudioTool,
    pub weather_prediction: WeatherPredictionTool,
    pub route_planner: RoutePlanner,
}
use crate::components::FUTUREENVIRONMENT;
use rastanidoumen_route_planner::tool::{RoutePlanner, RoutePlannerError};
static DIRECTION: Mutex<Vec<isize>> = Mutex::new(vec![]);

static FIRST_TICK: Mutex<bool> = Mutex::new(true);
static COIN_COORDS: Mutex<Vec<(usize, usize)>> = Mutex::new(vec![]);
impl Runnable for MyRobot {
    fn process_tick(&mut self, world: &mut World) {
        let mut direction = DIRECTION.lock().unwrap();
        where_am_i(self, world);
        //print!("{:?}", self.get_coordinate().get_col());
        let mut first_tick = FIRST_TICK.lock().unwrap();

        if *first_tick {
            *first_tick = false;
            for _ in 0..10 {
                (*direction).push(-1);
                (*direction).push(-2);
            }
            ui_variable_update(self, world);
            return;
        }
        let mut coin_coords = COIN_COORDS.lock().unwrap();
        if (*direction).is_empty() {
            if (*coin_coords).is_empty() {
                let robot_map = robot_map(world).unwrap();
                let mut found = false;
                for (r, row) in robot_map.iter().enumerate() {
                    for (c, tile) in row.iter().enumerate() {
                        if tile.is_some() {
                            let tile = tile.as_ref().unwrap();
                            if let Content::Coin(_) = tile.content {
                                (*coin_coords).push((r, c));

                                found = true;
                            }
                        }
                    }
                }
                if !found {
                    one_direction_view(self, world, Direction::Left, 10);
                    one_direction_view(self, world, Direction::Down, 10);
                    one_direction_view(self, world, Direction::Up, 5);
                    one_direction_view(self, world, Direction::Right, 5);
                }
            } else {
                let robot_row = self.get_coordinate().get_row();
                let robot_col = self.get_coordinate().get_col();
                let mut min_dist = 1000000000;
                let mut coordiantes_to_reach = &(0, 0);
                let mut pos = 0;

                let mut i = 0;
                let coordinate_clone = (*coin_coords).clone();
                for cords in &coordinate_clone {
                    let dist = cords.0.abs_diff(robot_row) + cords.1.abs_diff(robot_col);
                    if dist < min_dist {
                        min_dist = dist;
                        coordiantes_to_reach = cords;
                        pos = i;
                    }
                    i += 1;
                }

                let route = self.route_planner.get_directions_vec(
                    &robot_map(world),
                    look_at_sky(world),
                    (robot_row, robot_col),
                    *coordiantes_to_reach,
                );
                (*coin_coords).remove(pos);
                match route {
                    Ok(x) => {
                        println!("{:?}", x);
                        let mut prev_pos = (
                            self.get_coordinate().get_row(),
                            self.get_coordinate().get_col(),
                        );
                        for i in x.0 {
                            if i.0 == prev_pos.0 {
                                if i.1 < prev_pos.1 {
                                    (*direction).push(-1);
                                } else if i.1 > prev_pos.1 {
                                    (*direction).push(1);
                                }
                                prev_pos = (i.0, i.1);
                            } else if i.1 == prev_pos.1 {
                                if i.0 > prev_pos.0 {
                                    (*direction).push(-2);
                                } else if i.0 < prev_pos.0 {
                                    (*direction).push(2);
                                }
                                prev_pos = (i.0, i.1);
                            }
                        }
                    }
                    Err(e) => {
                        println!("Error {:?}", e);
                    }
                }
            }
        } else {
            let direction_len = direction.len();
            if direction_len == 1 {
                println!("why");
                println!("{:?}", direction);
                match direction[0] {
                    1 => {
                        destroy(self, world, Direction::Right);
                        direction.remove(0);
                    }
                    -1 => {
                        destroy(self, world, Direction::Left);
                        direction.remove(0);
                    }
                    2 => {
                        destroy(self, world, Direction::Up);
                        direction.remove(0);
                    }
                    -2 => {
                        destroy(self, world, Direction::Down);
                        direction.remove(0);
                    }
                    _ => {}
                }
            } else {
                match direction[direction_len - 1] {
                    1 => {
                        go(self, world, Direction::Right);
                        direction.remove(direction_len - 1);
                    }
                    -1 => {
                        go(self, world, Direction::Left);
                        direction.remove(direction_len - 1);
                    }
                    2 => {
                        go(self, world, Direction::Up);
                        direction.remove(direction_len - 1);
                    }
                    -2 => {
                        go(self, world, Direction::Down);
                        direction.remove(direction_len - 1);
                    }
                    _ => {}
                }
            }
        }

        ui_variable_update(self, world);
    }

    fn handle_event(&mut self, event: Event) {
        let _ = self.audio.play_audio_based_on_event(&event);
        self.weather_prediction.process_event(&event);
        let future_weather = self.weather_prediction.predict(96);

        {
            let mut env = FUTUREENVIRONMENT.lock().unwrap();
            match future_weather {
                Ok(weather) => {
                    *env = Some(weather);
                }
                Err(_) => *env = None,
            }
        }
        // println!();
        // println!("{:?}", event);
        // println!();
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
