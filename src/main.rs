mod alessio_gui;

mod components;

mod alessandro_gui;
mod robot;
mod alberto_gui;

use crate::alessandro_gui::main::run;
use crate::components::MAP_DIMENSION;
use crate::robot::Bessie;
use crate::robot::Graphics;
use crate::robot::Module;
use crate::robot::MyRobot;
use crate::utils::make_map;
use crate::utils::utils_for_ai::{robot_audio, ui_runner};
use alessio_gui::gui;
use alessio_gui::utils;
use exclusion_zone::content::bank::BankSettings;
use exclusion_zone::content::bin::BinSettings;
use exclusion_zone::content::coin::CoinSettings;
use exclusion_zone::content::fire::FireSettings;
use exclusion_zone::content::fish::FishSettings;
use exclusion_zone::content::garbage::GarbageSettings;
use exclusion_zone::content::market::MarketSettings;
use exclusion_zone::content::rock::RockSettings;
use exclusion_zone::content::tree::TreeSettings;
use exclusion_zone::content::wood_crate::CrateSettings;
use exclusion_zone::generator::{
    get_default_spawn_order, NoiseSettings, Thresholds, WorldGenerator,
};
use exclusion_zone::tile_type::lava::LavaSettings;
use robotics_lib;
use robotics_lib::runner::{Robot, Runner};
use robotics_lib::world::environmental_conditions::EnvironmentalConditions;
use robotics_lib::world::environmental_conditions::WeatherType::*;
use robotics_lib::world::environmental_conditions::WeatherType::{Rainy, Sunny};
use robotics_lib::world::tile::TileType::*;
use std::io;

use robotics_lib::world::tile::{Content, Tile};
use robotics_lib::world::world_generator::Generator;
use std::collections::HashMap;
use oxagaudiotool::sound_config::OxAgSoundConfig;

pub(crate) struct TestBessie {
    size: usize,
}


impl TestBessie {
    pub(crate) fn init(size: usize) -> Self {
        TestBessie { size }
    }
}

impl Generator for TestBessie {
    fn gen(
        &mut self,
    ) -> (
        Vec<Vec<Tile>>,
        (usize, usize),
        EnvironmentalConditions,
        f32,
        Option<HashMap<Content, f32>>,
    ) {
        rand::thread_rng();
        let mut map: Vec<Vec<Tile>> = Vec::new();
        // Initialize the map with default tiles
        for _ in 0..self.size {
            let mut row: Vec<Tile> = Vec::new();
            for _ in 0..self.size {
                row.push(Tile {
                    tile_type: Grass,
                    content: Content::Rock(10),
                    elevation: 0,
                });
            }
            // row[1] = Tile {
            //     tile_type: TileType::Lava,
            //     content: Content::None,
            //     elevation: 0,
            // };
            map.push(row);
        }
        map[3][3] = Tile {
            tile_type: Wall,
            content: Content::Fire,
            elevation: 0,
        };

        map[3][2] = Tile {
            tile_type: Teleport(false),
            content: Content::None,
            elevation: 0,
        };

        let environmental_conditions = EnvironmentalConditions::new(
            &[Sunny, Rainy, TrentinoSnow, TropicalMonsoon, Foggy],
            15,
            12,
        )
        .unwrap();

        let max_score = rand::random::<f32>();
        //(1,2)
        (map, (2, 2), environmental_conditions, max_score, None)
    }
}
fn main() {
    let mut my_banana;
    let robot;

    let world_size = 101;
    let mut world_generator = WorldGenerator::new(
        world_size,
        get_default_spawn_order(),
        NoiseSettings::default(),
        Thresholds::default(),
        LavaSettings::default(world_size),
        BankSettings::default(world_size),
        BinSettings::default(world_size),
        CrateSettings::default(world_size),
        GarbageSettings::default(world_size),
        FireSettings::default(world_size),
        TreeSettings::default(world_size),
        CoinSettings::default(world_size),
        MarketSettings::default(world_size),
        FishSettings::default(world_size),
        RockSettings::default(world_size),
    );
    let mut bessie_world = TestBessie::init(4);
    println!(
        "Do you want to test the tool or test the AI:
    -1: tool
    -2: AI"
    );
    let mut input_value = String::new();
    io::stdin()
        .read_line(&mut input_value)
        .expect("Failed to read line");

    let input_value: i32 = input_value.trim().parse().unwrap_or_else(|_| {
        println!("Invalid input. Defaulting to 1");
        1
    });
    let mode1;

    if input_value == 1 {
        mode1 = Module::Tool;
    } else {
        mode1 = Module::Ai;
    }

    println!(
        "Select which graphic you would like to use:
    -1: Alessio's one
    -2: Alessandro's one
    -3: Alberto's onw"

    );

    let mut input_value = String::new();
    io::stdin()
        .read_line(&mut input_value)
        .expect("Failed to read line");

    let input_value: i32 = input_value.trim().parse().unwrap_or_else(|_| {
        println!("Invalid input. Defaulting to 1");
        1
    });
    let graphics;

    if input_value == 1 {
        graphics = Graphics::Alessio;
    } else if input_value==2 {
        graphics = Graphics::Alessandro;
    }else{
        graphics = Graphics::Alberto;
    }

    my_banana = MyRobot {
        robot: Robot::new(),
        audio: robot_audio(&graphics),
        weather_prediction: ohcrab_weather::weather_tool::WeatherPredictionTool::new(),
        route_planner: rastanidoumen_route_planner::tool::RoutePlanner::default(),
        graphics: graphics.clone(),
    };
    let mut bessie = Bessie {
        robot: Robot::new(),
        audio: robot_audio(&graphics),
        weather_prediction: ohcrab_weather::weather_tool::WeatherPredictionTool::new(),
        route_planner: rastanidoumen_route_planner::tool::RoutePlanner::default(),
        graphics: graphics.clone(),
    };
    let background_music = OxAgSoundConfig::new_looped_with_volume("assets/default/music.ogg", 2.0);
    match mode1 {
        Module::Tool => {
            let _ = bessie.audio.play_audio(&background_music);
            robot = Runner::new(Box::new(bessie), &mut bessie_world);
        }
        Module::Ai => {
            let _ = my_banana.audio.play_audio(&background_music);
            robot = Runner::new(Box::new(my_banana), &mut world_generator);
        }
    }

    match robot {
        Ok(runner) => match graphics {
            Graphics::Alessio => {
                alessio_graphics(world_size, runner);
            }
            Graphics::Alessandro => {
                let _ = robot;
                alessandro_gui(runner);
            }
            _=>{
                alberto_gui::my_gui();
            }
        },
        Err(x) => {
            println!("Something went wrong!");
            println!("{:?}", x);
        }
    }
}

fn alessio_graphics(map_dim: usize, robot: Runner) {
    {
        let mut map_dimension = MAP_DIMENSION.lock().unwrap();
        *map_dimension = map_dim;
        make_map(*map_dimension);
    }
    ui_runner(robot)
}

fn alessandro_gui(robot: Runner) {
    run(robot)
}
