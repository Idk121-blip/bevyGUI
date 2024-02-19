mod alessio_gui;

mod components;

mod robot;
use crate::components::MAP_DIMENSION;
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

fn main() {
    let my_banana;
    let robot;

    // let mut world_generator = OxAgWorldGenerator::builder()
    //     .set_size(250)
    //     .set_environmental_conditions(
    //         EnvironmentalConditions::new(
    //             &[Sunny, Rainy, Foggy, TrentinoSnow, TropicalMonsoon],
    //             15,
    //             12,
    //         )
    //         .unwrap(),
    //     )
    //     .set_seed(17304017242468406439)
    //     .build()
    //     .unwrap();
    //println!("{:?}", generator.get_seed());
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
    //print the debug of generator.gen()on file output:txt

    {
        let mut map_dimension = MAP_DIMENSION.lock().unwrap();
        *map_dimension = 101;
        my_banana = MyRobot {
            robot: Robot::new(),
            audio: robot_audio(),
            weather_prediction: ohcrab_weather::weather_tool::WeatherPredictionTool::new(),

            route_planner: rastanidoumen_route_planner::tool::RoutePlanner::default(),
        };
        make_map(*map_dimension);
        robot = Runner::new(Box::new(my_banana), &mut world_generator);
    }

    match robot {
        Ok(runner) => {
            ui_runner(runner);
        }
        Err(x) => {
            println!("Something went wrong!");
            println!("{:?}", x);
        }
    }
}
