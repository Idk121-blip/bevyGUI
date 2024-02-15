mod gui;
mod robot;

mod utils;

use crate::gui::GuiPlugin;
use crate::robot::MyRobot;
use crate::utils::make_map;
pub(crate) use bevy::prelude::*;
pub(crate) use bevy::time::Timer;
use exclusion_zone::content::bank::BankSettings;
use exclusion_zone::content::bin::BinSettings;
use exclusion_zone::content::fire::FireSettings;
use exclusion_zone::content::garbage::GarbageSettings;
use exclusion_zone::content::tree::TreeSettings;
use exclusion_zone::content::wood_crate::CrateSettings;
/*use exclusion_zone::generator::WorldGenerator;*/
use exclusion_zone::generator::{get_default_spawn_order, NoiseSettings, Thresholds};
use exclusion_zone::tile_type::lava::LavaSettings;
use gui::hud::HudPlugin;
use lazy_static::lazy_static;
use oxagaudiotool::OxAgAudioTool;
use robot::WorldGenerator;
use robotics_lib;
use robotics_lib::runner::{Robot, Runner};
use robotics_lib::world::environmental_conditions::EnvironmentalConditions;
use robotics_lib::world::tile::Tile;
use robotics_lib::world::world_generator::Generator;
use std::sync::Mutex;
use std::vec::Vec;

lazy_static! {
/// List of coordinates that the robot has seen so far
    static ref PLOT: Mutex<Vec<Vec<Option<Tile>>>> = Mutex::new(vec![]);
}

lazy_static! {
    static ref ENVIRONMENT: Mutex<Option<EnvironmentalConditions>> = Mutex::new(None);
}

lazy_static! {
/// List of coordinates that the robot has seen so far
    static ref PLOTUPDATE: Mutex<Vec<Vec<Option<Tile>>>> = Mutex::new(vec![]);
}
lazy_static! {
    static ref EVENT: Mutex<Vec<MapEvent>> = Mutex::new(vec![]);
}

#[derive(PartialEq, Debug)]
pub(crate) enum MapEvent {
    UpdateMap,
    UpdateEnergy,
    //UpdateBackPack,
    UpdateWeather,
    UpdateRobot,
}

pub(crate) static MAP_DIMENSION: Mutex<usize> = Mutex::new(0);
pub(crate) static ROBOT_COL: Mutex<usize> = Mutex::new(0);
pub(crate) static ROBOT_ROW: Mutex<usize> = Mutex::new(0);

pub(crate) const PROCESS_TICK_TIME: f32 = 1.;
pub(crate) const MAP_UPDATE_TICK_TIME: f32 = 0.3;

pub(crate) const WEATHER_ASSET_FILE: &str = "tile_type.png";
pub(crate) const CLOCK_ASSET_FILE: &str = "clock.png";

#[derive(Component, Deref, DerefMut)]
struct TickUpdate(Timer);
#[derive(Component, Deref, DerefMut)]
struct MapTickUpdate(Timer);
#[derive(Component, Deref, DerefMut)]
struct TextUpdate(Timer);

pub(crate) struct RobotResource {
    pub(crate) runner: Runner,
}
/*
#[derive(Component)]
struct Map {
    plot: Vec<Vec<(Handle<TextureAtlas>, usize)>>,
}*/
#[derive(Component)]
pub struct MapToDespawn {
    plot: Vec<Vec<Entity>>,
}
#[derive(Component)]
pub struct ContentToDespawn {
    plot: Vec<Vec<Option<Entity>>>,
}

#[derive(Component)]
struct RobotUI {}

fn main() {
    let my_banana;
    let robot;
    let mut world_generator: WorldGenerator;
    let world_size = 1000;
    // let mut world_generator = WorldGenerator::new(
    //     world_size,
    //     get_default_spawn_order(),
    //     NoiseSettings::default(),
    //     Thresholds::default(),
    //     LavaSettings::default(world_size),
    //     BankSettings::default(world_size),
    //     BinSettings::default(world_size),
    //     CrateSettings::default(world_size),
    //     GarbageSettings::default(world_size),
    //     FireSettings::default(world_size),
    //     TreeSettings::default(world_size),
    // );

    {
        let mut map_dimension = MAP_DIMENSION.lock().unwrap();
        *map_dimension = 1000;
        my_banana = MyRobot {
            robot: Robot::new(),
            //audio: OxAgAudioTool::
        };
        make_map(*map_dimension);
        world_generator = WorldGenerator::init(*map_dimension);
        robot = Runner::new(Box::new(my_banana), &mut world_generator);
    }

    match robot {
        Ok(runner) => {
            App::new()
                .add_plugins(
                    DefaultPlugins
                        .set(ImagePlugin::default_nearest())
                        .set(WindowPlugin {
                            primary_window: Some(Window {
                                title: "Robot".into(),
                                resolution: (1420.0, 800.0).into(),
                                resizable: false,
                                ..default()
                            }),
                            ..default()
                        })
                        .build(),
                )
                .insert_non_send_resource(RobotResource { runner })
                //.add_systems(Startup, startup::setup /*, setup_tileex*/)
                //.add_systems(PostStartup, setup_map)
                .add_plugins(GuiPlugin)
                //.add_plugins()
                .run();
        }
        Err(x) => {
            println!("Something went wrong!");
            println!("{:?}", x);
        }
    }
}
// .add_systems(
//     Update,
//     (
//         characters_movement,
//         spawn_coin,
//         function,
//         zoom_handler,
//         camera_movement,
//         get_mouse_position,
//     ),
// )
