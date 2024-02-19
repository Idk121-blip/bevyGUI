use egui_extras::install_image_loaders;
use robotics_lib::runner::{Runner};
use robotics_lib::world::environmental_conditions::{DayTime, WeatherType};
use robotics_lib::world::tile::{Content, Tile};
use std::collections::HashMap;
use std::thread;
use std::time::Duration;
// use strum::IntoEnumIterator;
use crate::alessandro_gui::generator::{
    create_3d_content, create_3d_tile, create_robot_model, MapContent, MapTile,
};
use three_d::*;

// to change if name of robot change
use crate::alessandro_gui::robot_utils::set_robot_start_pos;

// ------------ global variables -----------
// --------- Context -----------------------
pub(crate) static mut GLOBAL_CONTEXT: Option<Context> = None;
// --------- 3d tiles / content / old world ------------
pub(crate) static mut GLOBAL_3D_TILES: Vec<Box<MapTile>> = vec![];
pub(crate) static mut GLOBAL_3D_CONTENT: Option<HashMap<Content, MapContent>> = None;
pub(crate) static mut PAST_WORLD: Vec<Vec<Option<Tile>>> = Vec::new();

// --------- robot -----------------------
pub(crate) static mut ENERGY: usize = 1000;
pub(crate) static mut ROBOT_MODEL: Option<Model<PhysicalMaterial>> = None;
pub(crate) static mut ROBOT_POS: (usize, usize, i8) = (2, 2, 0);
pub(crate) static mut INVENTORY: Option<HashMap<Content, usize>> = None;
pub(crate) static mut GLOBAL_SCORE: f32 = 0.0;
// ---------- weather and time ---------
pub(crate) static mut TIME: DayTime = DayTime::Morning;
pub(crate) static mut EXACT_TIME: String = String::new();
pub(crate) static mut WEATHER: WeatherType = WeatherType::Sunny;
pub(crate) static mut FUTURE_WEATHER: WeatherType = WeatherType::Sunny;
// static mut FUTURE_WEATHER: Vec<WeatherType> = vec![];

// ----- time control ------
pub(crate) static mut GLOBAL_TIMER: bool = true;
pub(crate) static mut GLOBAL_DELAY: Duration = Duration::from_secs(5);
pub(crate) static mut PAUSE: bool = true;

pub fn run(mut r: Runner) {
    let window = Window::new(WindowSettings {
        title: "RobotPark!".to_string(),
        max_size: Some((1280, 720)),
        ..Default::default()
    })
    .unwrap();
    let context = window.gl();
    unsafe {
        GLOBAL_CONTEXT = Some(context.clone());
    }

    let mut sky_camera = Camera::new_perspective(
        window.viewport(),
        vec3(100.0, 100.0, 100.0),
        vec3(10.0, 0.0, 10.0),
        vec3(0.0, 1.0, 0.0),
        degrees(45.0),
        0.1,
        10000.0,
    );
    let mut sky_control = FlyControl::new(1.0);

    let mut third_person_camera = Camera::new_perspective(
        window.viewport(),
        vec3(100.0, 100.0, 100.0),
        vec3(10.0, 0.0, 10.0),
        vec3(0.0, 1.0, 0.0),
        degrees(45.0),
        0.1,
        10000.0,
    );
    // _________________________________
    let mut tiles_glb = three_d_asset::io::load(&[
        "./assets/deepWater.glb",
        "./assets/shallowWater.glb",
        "./assets/sand.glb",
        "./assets/grass.glb",
        "./assets/street.glb",
        "./assets/hill.glb",
        "./assets/mountain.glb",
        "./assets/snow.glb",
        "./assets/lava.glb",
        "./assets/unknown.glb",
        "./assets/teleport.glb",
        "./assets/wall.glb",
    ])
    .unwrap();
    let mut content_glb = three_d_asset::io::load(&[
        "./assets/rock.glb",
        "./assets/tree.glb",
        "./assets/garbage.glb",
        "./assets/fire.glb",
        "./assets/coin.glb",
        "./assets/bin.glb",
        "./assets/crate.glb",
        "./assets/bank.glb",
        "./assets/market.glb",
        "./assets/fish.glb",
        "./assets/building.glb",
        "./assets/bush.glb",
    ])
    .unwrap();
    // Lights
    // let ambient = AmbientLight::new(&context, 10.0, Srgba::WHITE);
    let ambient = DirectionalLight::new(&context, 10.0, Srgba::WHITE, &vec3(-1.0, -1.0, -1.0));
    let directional = DirectionalLight::new(&context, 10.0, Srgba::WHITE, &vec3(1.0, 1.0, 1.0));
    // ------------------------------------------------
    // Skybox
    let mut loaded = three_d_asset::io::load(&[
        "./assets/afternoon_right.png",
        "./assets/afternoon_left.png",
        "./assets/afternoon_top.png",
        "./assets/afternoon_front.png",
        "./assets/afternoon_back.png",
        "./assets/afternoon_down.png",
        "./assets/day_right.png",
        "./assets/day_left.png",
        "./assets/day_up.png",
        "./assets/day_front.png",
        "./assets/day_back.png",
        "./assets/day_down.png",
        "./assets/night_right.png",
        "./assets/night_left.png",
        "./assets/night_up.png",
        "./assets/night_front.png",
        "./assets/night_back.png",
        "./assets/night_down.png",
    ])
    .unwrap();

    // let top_tex = loaded.deserialize("top").unwrap();
    // let bottom_tex = loaded.deserialize("top").unwrap();
    let aft_skybox = Skybox::new(
        &context,
        &loaded.deserialize("afternoon_right").unwrap(),
        &loaded.deserialize("afternoon_left").unwrap(),
        &loaded.deserialize("afternoon_top").unwrap(),
        &loaded.deserialize("afternoon_down").unwrap(),
        &loaded.deserialize("afternoon_front").unwrap(),
        &loaded.deserialize("afternoon_back").unwrap(),
    );
    let day_skybox = Skybox::new(
        &context,
        &loaded.deserialize("day_right").unwrap(),
        &loaded.deserialize("day_left").unwrap(),
        &loaded.deserialize("day_up").unwrap(),
        &loaded.deserialize("day_down").unwrap(),
        &loaded.deserialize("day_front").unwrap(),
        &loaded.deserialize("day_back").unwrap(),
    );
    let night_skybox = Skybox::new(
        &context,
        &loaded.deserialize("night_right").unwrap(),
        &loaded.deserialize("night_left").unwrap(),
        &loaded.deserialize("night_up").unwrap(),
        &loaded.deserialize("night_down").unwrap(),
        &loaded.deserialize("night_front").unwrap(),
        &loaded.deserialize("night_back").unwrap(),
    );

    // main loop
    let mut robot_pos = vec3(0.0, 0.0, 0.0);
    let mut third_person = false;
    create_robot_model();
    unsafe {
        set_robot_start_pos();
    }
    // -------- robot thing ---------------
    //     let mut generator = crate::robot::WorldGenerator::init(16);  //TO CHANGE WITH THE WORLD GENERATOR
    // let mut run = Runner::new(Box::new(my_banana), &mut generator).unwrap();
    // world generator --------------------
    // ------------- generate 3d map--------------
    unsafe {
        GLOBAL_3D_TILES = create_3d_tile(&GLOBAL_CONTEXT.as_ref().unwrap(), &mut tiles_glb);
        GLOBAL_3D_CONTENT = Some(create_3d_content(
            &GLOBAL_CONTEXT.as_ref().unwrap(),
            &mut content_glb,
        ));
    }
    let _ = thread::spawn(|| unsafe {
        loop {
            // Wait for the global delay
            thread::sleep(GLOBAL_DELAY);
            GLOBAL_TIMER = true;
        }
    });

    // --- EGUI variables --------------
    let mut gui = GUI::new(&context);
    // ------------------------------
    // ------------ delay saver -----
    let mut tick_delay: u64 = 5;
    // -------------------------------
    let _ = r.game_tick();
    window.render_loop(move |mut frame_input| {
        unsafe {
            if !PAUSE && GLOBAL_TIMER {
                GLOBAL_TIMER = false;
                r.game_tick().expect("TODO: panic message");
            }
        }
        let mut redraw = frame_input.first_frame;
        // --------------------------------
        if third_person {
            redraw |= third_person_camera.set_viewport(frame_input.viewport);
        } else {
            redraw |= sky_camera.set_viewport(frame_input.viewport);
        }
        // --------------------------------
        for event in &frame_input.events{
            match event {
                Event::KeyPress { kind,.. } => {
                    if *kind == Key::C{
                        // handle the refocus to the robot of the sky camera
                        let mut pos = sky_camera.position().clone();
                        let up = sky_camera.up().clone();
                        unsafe {
                            pos.x = (12.0) * ROBOT_POS.0 as f32;
                            pos.z = (12.0) * ROBOT_POS.1 as f32;
                            pos.y = 3.0;
                            sky_camera.set_view(pos, vec3((1.4+14.0) * ROBOT_POS.0 as f32, 3.0, (1.4+8.0) * ROBOT_POS.0 as f32), up);
                            robot_pos.x = (1.4+14.0) * ROBOT_POS.0 as f32;
                            robot_pos.y = 50.0;
                            robot_pos.z = (1.4+8.0) *  ROBOT_POS.0 as f32;
                        }
                        sky_control = FlyControl::new(
                            1.0
                        );
                        redraw=true;
                    }
                    else if *kind==Key::V {
                        if third_person {
                            third_person=false;
                        } else {
                            third_person = true;
                        }
                        redraw=true;
                    }
                    else if *kind==Key::P {
                        unsafe {
                            if PAUSE {
                                PAUSE = false;
                            } else {
                                PAUSE = true;
                            }
                        }
                        redraw=true;
                    }
                }
                _ => {}
            }
        }
        //------------- update the third person camera (every frame because the robot can move around)
        let pos = third_person_camera.position().clone();
        let up = third_person_camera.up().clone();
        unsafe {
            robot_pos.x = 6.0 + (12.0) * ROBOT_POS.0 as f32;
            if ROBOT_POS.2 == 0 {
                third_person_camera.set_view(pos, vec3(6.0 + (12.0) * ROBOT_POS.0 as f32 , 3.0, -6.0+(12.0) * ROBOT_POS.1 as f32), up);
                robot_pos.y = 3.0;
            } else if ROBOT_POS.2 == 1 {
                third_person_camera.set_view(pos, vec3(6.0 + (12.0) * ROBOT_POS.0 as f32 , 8.45, -6.0+(12.0) * ROBOT_POS.1 as f32), up);
                robot_pos.y = 8.45;
            } else if ROBOT_POS.2 == 2 {
                third_person_camera.set_view(pos, vec3(6.0 + (12.0) * ROBOT_POS.0 as f32 , 12.0, -6.0+(12.0) * ROBOT_POS.1 as f32), up);
                robot_pos.y = 12.0;
            }
            robot_pos.z = -6.0 + (12.0) * ROBOT_POS.1 as f32;
        }
        let mut third_p_control = OrbitControl::new(
            robot_pos,
            5.0,
            50.0,
        );


        // ------------ EGUI ---------------
        let mut panel_width = 0.0;
        gui.update(
            &mut frame_input.events,
            frame_input.accumulated_time,
            frame_input.viewport,
            frame_input.device_pixel_ratio,
            |gui_context| {
                use three_d::egui::*;
                install_image_loaders(gui_context);
                SidePanel::left("side_panel").show(gui_context, |ui| {
                    ui.heading(RichText::from("Robot Control Panel").strong()
                        .font(FontId::new(15.0, FontFamily::Proportional)));
                    ui.horizontal(|ui| {
                        unsafe {
                            if ui.add_enabled(PAUSE, Button::new("▶ PLAY")).clicked() {
                                PAUSE = false;
                            }
                            if ui.add_enabled(!PAUSE, Button::new("■ STOP")).clicked() {
                                PAUSE = true;
                            }
                        }
                    });
                    if ui.add(Slider::new(&mut tick_delay, 0..=10).text("Tick speed")).changed() {
                        unsafe {
                            GLOBAL_DELAY = Duration::from_secs(tick_delay)
                        }
                    }
                    ui.horizontal(|ui| {
                        ui.label(RichText::from("Camera").strong()
                            .font(FontId::new(15.0, FontFamily::Proportional)));
                        ui.label(RichText::from("(?)").strong())
                        .on_hover_text("Press 'V' to change camera, press 'C' to center the sky camera");
                    });
                    if ui.add(RadioButton::new(third_person == true, "Third person view")).clicked() {
                        third_person = true;
                        redraw = true;
                    }
                    if ui.add(RadioButton::new(third_person == false, "Sky view")).clicked() {
                        third_person = false;
                        redraw = true;
                    }
                    unsafe {
                        ui.label(RichText::from(format!("SCORE \u{1F3C6}: {}", GLOBAL_SCORE)).strong()
                            .font(FontId::new(15.0, FontFamily::Proportional)));
                    }
                    ui.label(RichText::from("Weather").strong()
                        .font(FontId::new(15.0, FontFamily::Proportional)));
                    unsafe {
                        match WEATHER {
                            WeatherType::Sunny => {
                                ui.label(WidgetText::from("Today is: SUNNY",).italics());
                                ui.add(
                                    Image::new(include_image!("../../assets/sunny_weather.png"))
                                        .rounding(5.0)
                                );
                            }
                            WeatherType::Rainy => {
                                ui.label(WidgetText::from("Today is: RAINY",).italics());
                                ui.add(
                                    Image::new(include_image!("../../assets/rainy_weather.jpg"))
                                        .rounding(5.0)
                                );
                            }
                            WeatherType::Foggy => {
                                ui.label(WidgetText::from("Today is: FOGGY",).italics());
                                ui.add(
                                    Image::new(include_image!("../../assets/fog_weather.jpg"))
                                        .rounding(5.0)
                                );
                            }
                            WeatherType::TropicalMonsoon => {
                                ui.label(WidgetText::from("Today there is a TROPICAL MONSOON ",).italics());
                                ui.add(
                                    Image::new(include_image!("../../assets/tropical_weather.jpg"))
                                        .rounding(5.0)
                                );
                            }
                            WeatherType::TrentinoSnow => {
                                ui.label(WidgetText::from("Today is: SNOWY",).italics());
                                ui.add(
                                    Image::new(include_image!("../../assets/snow_weather.jpg"))
                                        .rounding(5.0)
                                );
                            }
                        }
                        ui.label(RichText::from("Forecast:").strong());
                        match FUTURE_WEATHER {
                            WeatherType::Sunny => {
                                ui.label(RichText::from("\u{2600}")).on_hover_text("Sunny");
                            }
                            WeatherType::Rainy => {
                                ui.label(RichText::from("\u{2614}")).on_hover_text("Rainy");
                            }
                            WeatherType::Foggy => {
                                ui.label(RichText::from("\u{1f301}")).on_hover_text("Foggy");
                            }
                            WeatherType::TropicalMonsoon => {
                                ui.label(RichText::from("\u{26A0}")).on_hover_text("Tropical monsoon");
                            }
                            WeatherType::TrentinoSnow => {
                                ui.label(RichText::from("\u{2744}")).on_hover_text("Trentino snow");
                            }
                        }
                    }
                    ui.label(RichText::from("Time").strong()
                        .font(FontId::new(15.0, FontFamily::Proportional)));
                    unsafe {
                        match TIME {
                            DayTime::Morning => {
                                ui.label("Good morning!");
                            }
                            DayTime::Afternoon => {
                                ui.label("Good afternoon!");
                            }
                            DayTime::Night => {
                                ui.label("Good night!");
                            }
                        }
                        ui.label("It's ".to_string()+ &*EXACT_TIME.clone());
                    }
                    ui.label(RichText::from("Energy").strong()
                        .font(FontId::new(15.0, FontFamily::Proportional)));
                    unsafe {
                        ui.label(RichText::from(format!("the robot has {} energy left", ENERGY)).italics());
                        let mut energy_bar = ProgressBar::new(ENERGY as f32 / 1000f32);
                        energy_bar = energy_bar.text(format!("ENERGY: {}", ENERGY));
                        ui.add(energy_bar);
                        redraw = true;
                        ui.label(RichText::from("Inventory").strong()
                            .font(FontId::new(15.0, FontFamily::Proportional)));
                        ui.horizontal(|ui| {
                            ui.label(RichText::new("\u{1F330}".to_string()).font(FontId::new(40.0, FontFamily::Proportional))
                                .color(Color32::DARK_GRAY))
                                .on_hover_text(
                                RichText::new(format!("Rock"))
                            );
                            ui.label(RichText::new(format!("⮊ {}",INVENTORY.as_ref().unwrap()[&Content::Rock(0).to_default()]))
                                .font(FontId::new(15.0, FontFamily::Proportional)));
                            ui.label(RichText::new("\u{1F332}".to_string()).font(FontId::new(40.0, FontFamily::Proportional))
                                .color(Color32::DARK_GREEN))
                                .on_hover_text(
                                    RichText::new(format!("Tree"))
                                );
                            ui.label(RichText::new(format!("⮊ {}",INVENTORY.as_ref().unwrap()[&Content::Tree(0).to_default()]))
                                .font(FontId::new(15.0, FontFamily::Proportional)));
                        });
                        ui.horizontal(|ui| {
                            ui.label(RichText::new("\u{1F5D1}".to_string()).font(FontId::new(40.0, FontFamily::Proportional))
                                .color(Color32::GRAY))
                                .on_hover_text(
                                    RichText::new(format!("Garbage"))
                                );
                            ui.label(RichText::new(format!("⮊ {}",INVENTORY.as_ref().unwrap()[&Content::Garbage(0).to_default()]))
                                .font(FontId::new(15.0, FontFamily::Proportional)));
                            ui.label(RichText::new("\u{1F525}".to_string()).font(FontId::new(40.0, FontFamily::Proportional))
                                .color(Color32::RED))
                                .on_hover_text(
                                    RichText::new(format!("Fire"))
                                );
                            ui.label(RichText::new(format!("⮊ {}",INVENTORY.as_ref().unwrap()[&Content::Fire.to_default()]))
                                .font(FontId::new(15.0, FontFamily::Proportional)));
                        });
                        ui.horizontal(|ui| {
                            ui.label(RichText::new("\u{20AC}".to_string()).font(FontId::new(40.0, FontFamily::Proportional))
                                .color(Color32::GOLD))
                                .on_hover_text(
                                    RichText::new(format!("Coin"))
                                );
                            ui.label(RichText::new(format!("⮊ {}",INVENTORY.as_ref().unwrap()[&Content::Coin(0).to_default()]))
                                .font(FontId::new(15.0, FontFamily::Proportional)));
                            ui.label(RichText::new("\u{1F30A}".to_string()).font(FontId::new(40.0, FontFamily::Proportional))
                                .color(Color32::BLUE))
                                .on_hover_text(
                                    RichText::new(format!("Water"))
                                );
                            ui.label(RichText::new(format!("⮊ {}",INVENTORY.as_ref().unwrap()[&Content::Water(0).to_default()]))
                                .font(FontId::new(15.0, FontFamily::Proportional)));
                        });
                        ui.horizontal(|ui| {
                            ui.label(RichText::new("\u{1F41F}".to_string()).font(FontId::new(40.0, FontFamily::Proportional))
                                .color(Color32::LIGHT_YELLOW))
                                .on_hover_text(
                                    RichText::new(format!("Fish"))
                                );
                            ui.label(RichText::new(format!("⮊ {}",INVENTORY.as_ref().unwrap()[&Content::Fish(0).to_default()]))
                                .font(FontId::new(15.0, FontFamily::Proportional)));
                            ui.label(RichText::new("\u{1F341}".to_string()).font(FontId::new(40.0, FontFamily::Proportional))
                                .color(Color32::LIGHT_GREEN))
                                .on_hover_text(
                                    RichText::new(format!("Bush"))
                                );
                            ui.label(RichText::new(format!("⮊ {}",INVENTORY.as_ref().unwrap()[&Content::Bush(0).to_default()]))
                                .font(FontId::new(15.0, FontFamily::Proportional)));
                        });
                    }
                    let popup_id = ui.make_persistent_id("my_unique_id");
                    let response = ui.add(Button::new("Credits"));
                    if response.clicked() {
                        // open the credits pop-up
                        ui.memory_mut(|mem| mem.toggle_popup(popup_id));
                    }
                    let below = AboveOrBelow::Above;
                    popup_above_or_below_widget(ui, popup_id, &response, below, |ui| {
                        ui.set_min_width(200.0); // if you want to control the size
                        ui.heading("CREDITS");
                        ui.label(RichText::new("3D models:").strong());
                        ui.label("A lot of 3d model from https://www.kenney.nl/");
                        ui.label("Lego 3d model from https://www.mecabricks.com/");
                        ui.separator();
                        ui.label(" 'Common Things #1' (https://skfb.ly/6UEVV) by Helindu is licensed under Creative Commons Attribution (https://creativecommons.org/licenses/by/4.0/).");
                        ui.label(" 'ATM' (https://skfb.ly/oIySE) by haohao2210 is licensed under Creative Commons Attribution (https://creativecommons.org/licenses/by/4.0/).");
                        ui.label(" 'Coin' (https://skfb.ly/6RLBv) by Folly is licensed under Creative Commons Attribution (https://creativecommons.org/licenses/by/4.0/).");
                    });
                });
                panel_width = gui_context.used_rect().width();
            },
        );
        // ---------------------------------

        if third_person {
            redraw |= third_p_control.handle_events(&mut third_person_camera, &mut frame_input.events);
        } else {
            redraw |= sky_control.handle_events(&mut sky_camera, &mut frame_input.events);
        }

        if redraw {
            frame_input

                .screen()
                .clear(ClearState::color_and_depth(0.8, 0.8, 0.8, 1.0, 1.0))
                .write(|| {
                    unsafe {
                        // render of tile and content
                        for tile in GLOBAL_3D_TILES.iter()
                        {
                            for o in tile.instanced_model.into_iter() {
                                if third_person {
                                    o.render(&third_person_camera, &[&ambient, &directional]);
                                } else {
                                    o.render(&sky_camera, &[&ambient, &directional]);
                                }
                            }
                        }
                        for (_,map_content) in GLOBAL_3D_CONTENT.as_mut().unwrap()
                        {
                            for o in map_content.instanced_model.into_iter() {
                                if third_person {
                                    o.render(&third_person_camera, &[&ambient, &directional]);
                                } else {
                                    o.render(&sky_camera, &[&ambient, &directional]);
                                }
                            }
                        }
                    }

                    // -------------render of the robot -------------
                    unsafe {
                        for o in ROBOT_MODEL.as_ref().unwrap().into_iter()
                        {
                            if third_person {
                                o.render(&third_person_camera, &[&ambient, &directional]);
                            } else {
                                o.render(&sky_camera, &[&ambient, &directional]);
                            }
                        }
                    }
                    unsafe {
                        if third_person {
                            // (&Axes::new(&context,0.2,100.0).render(&third_person_camera, &[&ambient, &directional]));
                            match TIME {
                                DayTime::Morning => {
                                    day_skybox.render(&third_person_camera, &[&ambient, &directional]);
                                }
                                DayTime::Afternoon => {
                                    aft_skybox.render(&third_person_camera, &[&ambient, &directional]);
                                }
                                DayTime::Night => {
                                    night_skybox.render(&third_person_camera, &[&ambient]);
                                }
                            }
                        } else {
                            match TIME {
                                DayTime::Morning => {
                                    day_skybox.render(&sky_camera, &[&ambient, &directional]);
                                }
                                DayTime::Afternoon => {
                                    aft_skybox.render(&sky_camera, &[&ambient, &directional]);
                                }
                                DayTime::Night => {
                                    night_skybox.render(&sky_camera, &[&ambient]);
                                }
                            }
                            // (&Axes::new(&context,0.2,100.0).render(&sky_camera, &[&ambient, &directional]));
                        }
                    }
                    gui.render();
                });
        } else {
            frame_input.screen().write(|| gui.render());
        };

        FrameOutput {
            swap_buffers: redraw,
            ..Default::default()
        }
    });
}
