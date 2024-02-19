use crate::alessandro_gui::map_update::{
    change_time, change_weather, future_weather, score_update, update_content,
};
use crate::alessandro_gui::robot_utils::{
    add_energy, add_to_backpack, move_robot, set_initial_config, sub_energy, sub_to_backpack,
};
use crate::utils::utils_for_ai::ui_variable_update;
use bessie::bessie::road_paving_machine;
use bessie::bessie::State;
use fixedbitset::IndexRange;
use lazy_static::lazy_static;
use ohcrab_weather::weather_tool::WeatherPredictionTool;
use oxagaudiotool::OxAgAudioTool;
use robotics_lib::energy::Energy;
use robotics_lib::event::events::Event;
use robotics_lib::interface::{
    destroy, discover_tiles, get_score, go, look_at_sky, one_direction_view, put, robot_map,
    where_am_i, Direction,
};
use robotics_lib::runner::backpack::BackPack;
use robotics_lib::runner::{Robot, Runnable};
use robotics_lib::world::coordinates::Coordinate;
use robotics_lib::world::tile::{Content, Tile};
use robotics_lib::world::World;
use std::cmp::{max, min};
use std::sync::Mutex;
#[derive(Clone)]
pub enum Graphics {
    Alessio,
    Alessandro,
}

pub enum Module {
    Tool,
    Ai,
}

pub(crate) struct MyRobot {
    pub robot: Robot,
    pub audio: OxAgAudioTool,
    pub weather_prediction: WeatherPredictionTool,
    pub route_planner: RoutePlanner,
    pub graphics: Graphics,
}
pub(crate) struct Bessie {
    pub robot: Robot,
    pub audio: OxAgAudioTool,
    pub weather_prediction: WeatherPredictionTool,
    pub route_planner: RoutePlanner,
    pub graphics: Graphics,
}

use crate::alessandro_gui::map_update::robot_around_tile;
use crate::components::FUTUREENVIRONMENT;
use rastanidoumen_route_planner::tool::{RoutePlanner, RoutePlannerError};

static DIRECTION: Mutex<Vec<isize>> = Mutex::new(vec![]);

static FIRST_TICK: Mutex<bool> = Mutex::new(true);
static COIN_COORDS: Mutex<Vec<(usize, usize)>> = Mutex::new(vec![]);
static BANK_COORDS: Mutex<Vec<(usize, usize)>> = Mutex::new(vec![]);

impl Runnable for MyRobot {
    fn process_tick(&mut self, world: &mut World) {
        let mut direction = DIRECTION.lock().unwrap();
        where_am_i(self, world);
        let mut first_tick = FIRST_TICK.lock().unwrap();

        if *first_tick {
            *first_tick = false;
            println!("First tick");
            (*direction).push(0);
            for c in 0..10 {
                (*direction).push(-2);
                if (c - 2) % 3 == 0 {
                    (*direction).push(-3);
                }
                (*direction).push(-1);
                if (c - 2) % 3 == 0 {
                    (*direction).push(3);
                }
            }

            match self.graphics {
                Graphics::Alessio => {
                    ui_variable_update(self, world);
                }
                Graphics::Alessandro => {
                    robot_around_tile(robot_map(world).unwrap_or(vec![vec![None]]));
                    score_update(get_score(&world));
                }
            }
            println!("{:?}", *direction);
            return;
        }
        let mut coin_coords = COIN_COORDS.lock().unwrap();

        if (*direction).is_empty() {
            if (*coin_coords).is_empty() {
                let mut found = false;
                {
                    let robot_map = robot_map(world).unwrap();
                    for (r, row) in robot_map.iter().enumerate() {
                        for (c, tile) in row.iter().enumerate() {
                            if tile.is_some() {
                                let tile = tile.as_ref().unwrap();
                                if let Content::Coin(_) = tile.content {
                                    (*coin_coords).push((r, c));
                                    found = true;
                                }
                                if let Content::Bank(x) = tile.content.clone() {
                                    if x.start() != x.end() {
                                        let mut bank_coords = BANK_COORDS.lock().unwrap();
                                        (*bank_coords).push((r, c));
                                    }
                                }
                            }
                        }
                    }
                }
                if !found {
                    one_direction_view(self, world, Direction::Right, 30);
                    one_direction_view(self, world, Direction::Up, 30);
                    one_direction_view(self, world, Direction::Left, 30);
                    one_direction_view(self, world, Direction::Down, 30);
                    let robot_map = robot_map(world).unwrap();
                    for r in max(self.get_coordinate().get_row() as isize - 20 as isize, 0) as usize
                        ..min(self.get_coordinate().get_row() + 20, 101)
                    {
                        for c in max(self.get_coordinate().get_col() as isize - 20 as isize, 0)
                            as usize
                            ..min(self.get_coordinate().get_col() + 20, 101)
                        {
                            if robot_map[r][c].is_none() {
                                discover_tiles(self, world, &vec![(r, c)]);
                            }
                        }
                    }
                }
            } else {
                let robot_row = self.get_coordinate().get_row();
                let robot_col = self.get_coordinate().get_col();
                let mut go_bank = false;
                for (content, quantity) in self.robot.backpack.get_contents() {
                    if let Content::Coin(_) = content {
                        if *quantity > 8 {
                            go_bank = true;
                        }
                    }
                }
                let mut min_dist = 1000000000;
                let mut coordiantes_to_reach = &(0, 0);
                let mut pos = 0;

                let mut i = 0;

                let mut coordinate_clone = (*coin_coords).clone();

                if go_bank && (*BANK_COORDS.lock().unwrap()).is_empty() {
                    one_direction_view(self, world, Direction::Up, 30);
                    one_direction_view(self, world, Direction::Left, 30);
                    one_direction_view(self, world, Direction::Right, 30);
                    one_direction_view(self, world, Direction::Down, 30);
                    let robot_map = robot_map(world).unwrap();
                    for r in max(self.get_coordinate().get_row() as isize - 20 as isize, 0) as usize
                        ..min(self.get_coordinate().get_row() + 20, 101)
                    {
                        for c in max(self.get_coordinate().get_col() as isize - 20 as isize, 0)
                            as usize
                            ..min(self.get_coordinate().get_col() + 20, 101)
                        {
                            if robot_map[r][c].is_none() {
                                discover_tiles(self, world, &vec![(r, c)]);
                            }
                        }
                    }
                } else {
                    if go_bank {
                        let bank_coords = BANK_COORDS.lock().unwrap();
                        coordinate_clone = (*bank_coords).clone();
                    }
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
                    if !go_bank {
                        (*coin_coords).remove(pos);
                    }

                    match route {
                        Ok(x) => {
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

                            (*direction).reverse();
                        }
                        Err(e) => {
                            println!("Error {:?}", e);
                        }
                    }
                }
            }
        } else {
            let direction_len = direction.len();
            if direction_len == 1 {
                match direction[0] {
                    1 => {
                        let vec = where_am_i(self, world).0;
                        if let Content::Bank(x) = (&vec[1][2]).clone().unwrap().content {
                            if put(self, world, Content::Coin(0), 5, Direction::Right).is_err() {
                                direction.remove(0);
                            }
                            if x.start() == x.end() {
                                let mut bank_coords = BANK_COORDS.lock().unwrap();
                                let mut i = 0;
                                println!("{:?}", bank_coords.clone());
                                for b in &*bank_coords.clone() {
                                    println!("{:?}", b);
                                    println!("{}", i);
                                    if b.0 == self.get_coordinate().get_row() {
                                        if b.1 == self.get_coordinate().get_col() + 1 {
                                            bank_coords.remove(i);
                                            i -= 1;
                                        }
                                    }
                                    i += 1;
                                }
                                direction.remove(0);
                            }
                        } else {
                            destroy(self, world, Direction::Right);
                            direction.remove(0);
                        }
                    }
                    -1 => {
                        let vec = where_am_i(self, world).0;
                        if let Content::Bank(x) = (&vec[1][0]).clone().unwrap().content {
                            if put(self, world, Content::Coin(0), 5, Direction::Left).is_err() {
                                direction.remove(0);
                            }
                            if x.start() == x.end() {
                                let mut bank_coords = BANK_COORDS.lock().unwrap();
                                let mut i = 0;
                                println!("{:?}", bank_coords.clone());
                                for b in &*bank_coords.clone() {
                                    println!("{:?}", b);
                                    println!("{}", i);
                                    if b.0 == self.get_coordinate().get_row() {
                                        if b.1 == self.get_coordinate().get_col() - 1 {
                                            bank_coords.remove(i);
                                            i -= 1;
                                        }
                                    }
                                    i += 1;
                                }
                                direction.remove(0);
                            }
                        } else {
                            destroy(self, world, Direction::Left);
                            direction.remove(0);
                        }
                    }
                    2 => {
                        let vec = where_am_i(self, world).0;
                        if let Content::Bank(x) = (&vec[0][1]).clone().unwrap().content {
                            if put(self, world, Content::Coin(0), 5, Direction::Up).is_err() {
                                direction.remove(0);
                            }
                            if x.start() == x.end() {
                                let mut bank_coords = BANK_COORDS.lock().unwrap();
                                let mut i = 0;
                                println!("{:?}", bank_coords.clone());
                                for b in &*bank_coords.clone() {
                                    println!("{:?}", b);
                                    println!("{}", i);
                                    if b.1 == self.get_coordinate().get_col() {
                                        if b.0 == self.get_coordinate().get_row() - 1 {
                                            bank_coords.remove(i);
                                            i -= 1;
                                        }
                                    }
                                    i += 1;
                                }
                                direction.remove(0);
                            }
                        } else {
                            destroy(self, world, Direction::Up);
                            direction.remove(0);
                        }
                    }
                    -2 => {
                        let vec = where_am_i(self, world).0;
                        if let Content::Bank(x) = (&vec[2][1]).clone().unwrap().content {
                            if put(self, world, Content::Coin(0), 30, Direction::Down).is_err() {
                                direction.remove(0);
                            }
                            if x.start() == x.end() {
                                let mut bank_coords = BANK_COORDS.lock().unwrap();
                                let mut i = 0;
                                println!("{:?}", bank_coords.clone());
                                for b in &*bank_coords.clone() {
                                    println!("{:?}", b);
                                    println!("{}", i);
                                    if b.1 == self.get_coordinate().get_col() {
                                        if b.0 == self.get_coordinate().get_row() + 1 {
                                            bank_coords.remove(i);
                                            i -= 1;
                                        }
                                    }
                                    i += 1;
                                }
                                direction.remove(0);
                            }
                        } else {
                            destroy(self, world, Direction::Down);
                            direction.remove(0);
                        }
                    }

                    _ => {
                        direction.remove(0);
                    }
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
                    3 => {
                        one_direction_view(self, world, Direction::Down, 10);
                        direction.remove(direction_len - 1);
                    }
                    -3 => {
                        one_direction_view(self, world, Direction::Left, 10);
                        direction.remove(direction_len - 1);
                    }
                    _ => {}
                }
            }
        }

        match self.graphics {
            Graphics::Alessio => {
                ui_variable_update(self, world);
            }
            Graphics::Alessandro => {
                robot_around_tile(robot_map(world).unwrap_or(vec![vec![None]]));
                score_update(get_score(&world));
            }
        }
    }

    fn handle_event(&mut self, event: Event) {
        const ONE_DAY_TICK: usize = 96;
        self.weather_prediction.process_event(&event);
        // println!();
        let fut_weather = self.weather_prediction.predict(ONE_DAY_TICK);
        // println!("{:?}", event);
        // println!();

        match self.graphics {
            Graphics::Alessio => {
                let _ = self.audio.play_audio_based_on_event(&event);
                {
                    let mut env = FUTUREENVIRONMENT.lock().unwrap();
                    match fut_weather {
                        Ok(weather) => {
                            *env = Some(weather);
                        }
                        Err(_) => *env = None,
                    }
                }
            }
            Graphics::Alessandro => {
                let _ = self.audio.play_audio_based_on_event(&event);
                match fut_weather {
                    Ok(weather) => {
                        future_weather(weather);
                    }
                    Err(_) => {}
                }
                match event {
                    robotics_lib::event::events::Event::Ready => set_initial_config(
                        self.get_coordinate().get_row(),
                        self.get_coordinate().get_col(),
                    ),
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
                        add_energy(x)
                    },
                    robotics_lib::event::events::Event::EnergyConsumed(x) => sub_energy(x),
                    robotics_lib::event::events::Event::Moved(tile, (x, y)) => {
                        move_robot(x, y);
                    }
                    robotics_lib::event::events::Event::TileContentUpdated(tile, c) => {
                        update_content(tile, c)
                    }
                    robotics_lib::event::events::Event::AddedToBackpack(content, amount) => {
                        add_to_backpack(&content, amount)
                    }
                    robotics_lib::event::events::Event::RemovedFromBackpack(content, amount) => {
                        sub_to_backpack(&content, amount)
                    }
                }
            }
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

impl Runnable for Bessie {
    fn process_tick(&mut self, world: &mut World) {
        let mut first_tick = FIRST_TICK.lock().unwrap();
        where_am_i(self, world);
        if *first_tick {
            match self.graphics {
                Graphics::Alessio => {
                    ui_variable_update(self, world);
                }
                Graphics::Alessandro => {
                    robot_around_tile(robot_map(world).unwrap_or(vec![vec![None]]));
                    score_update(get_score(&world));
                }
            }
            *first_tick = false;
            return;
        }
        println!(
            "{:?}",
            road_paving_machine(self, world, Direction::Up, State::MakeRoad)
        );
        println!(
            "{:?}",
            road_paving_machine(self, world, Direction::Up, State::GetStones)
        );

        match self.graphics {
            Graphics::Alessio => {
                ui_variable_update(self, world);
            }
            Graphics::Alessandro => {
                robot_around_tile(robot_map(world).unwrap_or(vec![vec![None]]));
                score_update(get_score(&world));
            }
        }
    }
    fn handle_event(&mut self, event: Event) {
        const ONE_DAY_TICK: usize = 96;
        self.weather_prediction.process_event(&event);
        // println!();
        let fut_weather = self.weather_prediction.predict(ONE_DAY_TICK);
        // println!("{:?}", event);
        // println!();

        match self.graphics {
            Graphics::Alessio => {
                let _ = self.audio.play_audio_based_on_event(&event);
                {
                    let mut env = FUTUREENVIRONMENT.lock().unwrap();
                    match fut_weather {
                        Ok(weather) => {
                            *env = Some(weather);
                        }
                        Err(_) => *env = None,
                    }
                }
            }
            Graphics::Alessandro => {
                let _ = self.audio.play_audio_based_on_event(&event);
                match fut_weather {
                    Ok(weather) => {
                        future_weather(weather);
                    }
                    Err(_) => {}
                }
                match event {
                    robotics_lib::event::events::Event::Ready => set_initial_config(
                        self.get_coordinate().get_row(),
                        self.get_coordinate().get_col(),
                    ),
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
                        add_energy(x)
                    },
                    robotics_lib::event::events::Event::EnergyConsumed(x) => sub_energy(x),
                    robotics_lib::event::events::Event::Moved(tile, (x, y)) => {
                        move_robot(x, y);
                    }
                    robotics_lib::event::events::Event::TileContentUpdated(tile, c) => {
                        update_content(tile, c)
                    }
                    robotics_lib::event::events::Event::AddedToBackpack(content, amount) => {
                        add_to_backpack(&content, amount)
                    }
                    robotics_lib::event::events::Event::RemovedFromBackpack(content, amount) => {
                        sub_to_backpack(&content, amount)
                    }
                }
            }
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
