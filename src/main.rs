mod map_update;
mod robot;
mod startup;
mod utils;
use crate::map_update::update_map;
use crate::robot::MyRobot;
use crate::robot::WorldGenerator;
use crate::startup::setup;
use crate::utils::make_map;
use bevy::input::mouse::MouseMotion;
pub(crate) use bevy::prelude::*;
pub(crate) use bevy::time::Timer;
use lazy_static::lazy_static;

use robotics_lib;

use robotics_lib::runner::{Robot, Runner};

use robotics_lib::world::tile::Tile;
use robotics_lib::world::tile::TileType::*;

use std::sync::Mutex;
use std::vec::Vec;

lazy_static! {
/// List of coordinates that the robot has seen so far
    static ref PLOT: Mutex<Vec<Vec<Option<Tile>>>> = Mutex::new(vec![]);
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
    UpdateBackPack,
    UpdateWeather,
    UpdateRobot,
}

pub(crate) static MAP_DIMENSION: Mutex<usize> = Mutex::new(0);
pub(crate) static ROBOT_COL: Mutex<usize> = Mutex::new(0);
pub(crate) static ROBOT_ROW: Mutex<usize> = Mutex::new(0);

pub(crate) const TILE_DIMENSION: f32 = 32.0;
pub(crate) const PROCESS_TICK_TIME: f32 = 2.0;
pub(crate) const MAP_UPDATE_TICK_TIME: f32 = 0.5;

#[derive(Component, Deref, DerefMut)]
struct TickUpdate(Timer);
#[derive(Component, Deref, DerefMut)]
struct MapTickUpdate(Timer);

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
    let mut generator: WorldGenerator;
    {
        let mut map_dimension = MAP_DIMENSION.lock().unwrap();
        *map_dimension = 20;
        println!("{}", map_dimension);
        my_banana = MyRobot(Robot::new());
        generator = WorldGenerator::init(*map_dimension);
        make_map(*map_dimension);
        robot = Runner::new(Box::new(my_banana), &mut generator);
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
                                resolution: (672.0, 672.0).into(),
                                resizable: false,
                                ..default()
                            }),
                            ..default()
                        })
                        .build(),
                )
                .insert_non_send_resource(RobotResource { runner: runner })
                .add_systems(Startup, setup /*, setup_tileex*/)
                //.add_systems(PostStartup, setup_map)
                .add_systems(Update, (tick, update_map /*camera_movement*/))
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

fn tick(
    time: Res<Time>,
    mut query: Query<&mut MapTickUpdate>,
    mut runner: NonSendMut<RobotResource>,
) {
    match query.get_single_mut() {
        Ok(mut timer) => {
            timer.tick(time.delta());
            if timer.just_finished() {
                let _ = runner.runner.game_tick();
                timer.0 = Timer::from_seconds(PROCESS_TICK_TIME, TimerMode::Repeating);
            }
        }
        Err(x) => {
            panic!("{:?}", x);
        }
    }
}
