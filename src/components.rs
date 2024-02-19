use crate::gui::system::startup::{Component, Deref, DerefMut, Entity, Timer};
use lazy_static::lazy_static;
use robotics_lib::runner::Runner;
use robotics_lib::world::environmental_conditions::{EnvironmentalConditions, WeatherType};
use robotics_lib::world::tile::Tile;
use std::sync::Mutex;

lazy_static! {
/// List of coordinates that the banana has seen so far
    pub(crate) static ref PLOT: Mutex<Vec<Vec<Option<Tile>>>> = Mutex::new(vec![]);
}

lazy_static! {
    pub(crate) static ref ENVIRONMENT: Mutex<Option<EnvironmentalConditions>> = Mutex::new(None);
}

lazy_static! {
    pub(crate) static ref FUTUREENVIRONMENT: Mutex<Option<WeatherType>> = Mutex::new(None);
}
lazy_static! {
/// List of coordinates that the banana has seen so far
    pub(crate) static ref PLOTUPDATE: Mutex<Vec<Vec<Option<Tile>>>> = Mutex::new(vec![]);
}
lazy_static! {
    pub(crate) static ref EVENT: Mutex<Vec<MapEvent>> = Mutex::new(vec![]);
}
pub(crate) static SCORE: Mutex<f32> = Mutex::new(0.);
#[derive(PartialEq, Debug)]
pub(crate) enum MapEvent {
    UpdateMap,
    UpdateRobot,
}

pub(crate) static MAP_DIMENSION: Mutex<usize> = Mutex::new(0);
pub(crate) static ROBOT_COL: Mutex<usize> = Mutex::new(0);
pub(crate) static ROBOT_ROW: Mutex<usize> = Mutex::new(0);

pub(crate) const PROCESS_TICK_TIME: f32 = 1.;

pub(crate) const WEATHER_ASSET_FILE: &str = "weather.png";
pub(crate) const CLOCK_ASSET_FILE: &str = "clock.png";

#[derive(Component, Deref, DerefMut)]
pub(crate) struct TickUpdate(pub Timer);
#[derive(Component, Deref, DerefMut)]
pub(crate) struct TextUpdate(pub Timer);

pub(crate) struct RobotResource {
    pub(crate) runner: Runner,
}

#[derive(Component)]
pub struct MapToDespawn {
    pub(crate) plot: Vec<Vec<Entity>>,
}
#[derive(Component)]
pub struct ContentToDespawn {
    pub(crate) plot: Vec<Vec<Option<Entity>>>,
}

#[derive(Component)]
pub(crate) struct RobotUI {}
