use crate::utils_for_ai::ui_variable_update;
use ohcrab_weather::weather_tool::WeatherPredictionTool;
use oxagaudiotool::OxAgAudioTool;
use robotics_lib::energy::Energy;
use robotics_lib::event::events::Event;
use robotics_lib::interface::{destroy, go, look_at_sky, Direction};
use robotics_lib::runner::backpack::BackPack;
use robotics_lib::runner::{Robot, Runnable};
use robotics_lib::world::coordinates::Coordinate;
use robotics_lib::world::tile::TileType::*;
use robotics_lib::world::World;

pub(crate) struct MyRobot {
    pub robot: Robot,
    pub audio: OxAgAudioTool,
    pub weather_prediction: WeatherPredictionTool,
}
use crate::components::{ENVIRONMENT, FUTUREENVIRONMENT};
use recycle_by_ifrustrati::tool::recycle;

impl Runnable for MyRobot {
    fn process_tick(&mut self, world: &mut World) {
        // Data to be written into the file
        // Write data into the file
        let _ = go(self, world, Direction::Left);
        let _ = go(self, world, Direction::Down);
        ui_variable_update(self, world);
    }

    fn handle_event(&mut self, event: Event) {
        let _ = self.audio.play_audio_based_on_event(&event);
        self.weather_prediction.process_event(&event);
        let future_weather = self.weather_prediction.predict(20);

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
