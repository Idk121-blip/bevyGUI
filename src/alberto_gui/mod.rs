use bevy::app::App;
use bevy::app::Update;
use bevy::app::Startup;
use bevy::ecs::component::Component;
use bevy::ecs::system::Commands;
use bevy::prelude::*;
use bevy::DefaultPlugins;
use bevy::render::color::Color;
use bevy::core_pipeline::clear_color::ClearColor;
use bevy::core_pipeline::core_2d::Camera2dBundle;
use bevy::transform::components::Transform;
use bevy::sprite::SpriteBundle;
use bevy::utils::default;
use bevy::asset::AssetServer;
use bevy::ecs::system::Res;
use robotics_lib;
// use robotics_lib::utils::LibError::*;

use robotics_lib::world::tile::Content::{
    Fire,
};
use robotics_lib::world::tile::TileType::*;
use robotics_lib::world::tile::{Tile};


#[derive(Component)]
struct Person;

#[derive(Component)]
pub struct PixelColor(Color);
#[derive(Component)]
struct CustomUV;



pub fn my_gui(){

    //Questo resta, ma non deve essere un main
    let _app = App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, spawn_camera)
        .add_systems(Update, spawn_world)
        .run();
}

pub fn spawn_camera(
    mut commands: Commands,
){  //Genera il punto di vista
    commands.spawn(Camera2dBundle::default());
    //Imposta un colore di sfondo (Ciano)
    commands.insert_resource(ClearColor(Color::WHITE));

}

pub fn show_world(
    mut command: Commands,
    asset_server: Res<AssetServer>,
    mondo: Vec<Vec<Tile>>,

){

    //px e py definiscono la posizione del personaggio, TOGLIERE nella versione finale (già implementato)
    let px: f32 = 200.0;
    let py: f32 = 200.0;
    let z: f32 = 50.0;

    //Variabili del mondo per definire la posizione del personaggio, basta DECOMMENTARLE nella versione finale
    //let (robot_view, robot_position)=where_am_i(robot, &mut world);
    //let mut px: f32 = robot_position.0 as f32;
    //let mut py: f32 = robot_position.1 as f32;

    //Funzione che stampa il mondo
    let mut n:f32 = 0.0;
    for i in mondo{
        let mut m:f32 = 0.0;
        for k in i{

            command.spawn(SpriteBundle {
                texture: match k.tile_type{
                    Sand =>{ asset_server.load("Tiles/Sabbia.png") }
                    DeepWater => { asset_server.load("Tiles/AcquaProfonda.png") }
                    ShallowWater => { asset_server.load("Tiles/Acqua.png") }
                    Grass => { asset_server.load("Tiles/Erba.png") }
                    Street => { asset_server.load("Tiles/Strada.png") }
                    Hill => { asset_server.load("Tiles/Campo.png") }
                    Mountain => { asset_server.load("Tiles/Montagna.png") }
                    Snow => { asset_server.load("Tiles/Neve.png") }
                    Lava => { asset_server.load("Tiles/Lava.png") }
                    Teleport(_) => { asset_server.load("Tiles/Teletrasporto.png") }
                    Wall => { asset_server.load("Tiles/Muro.png") }
                },
                transform: Transform::from_xyz(- px + n ,  - py + m, 0.0),
                sprite: Sprite{
                    custom_size: Some(Vec2::new(z, z)),
                    ..default()
                },
                ..default()
            });

            if n == px{
                if m == py{
                    command.spawn(SpriteBundle{
                        texture: asset_server.load("Tiles/Personaggio.png"),
                        transform: Transform::from_xyz(- px + n ,  - py + m, 0.0),
                        sprite: Sprite{
                            custom_size: Some(Vec2::new(z, z)),
                            ..default()
                        },

                        ..default()
                    });
                }

            }

            m += z;
        }
        n += z;
    }


}


fn spawn_world(
    command: Commands,
    asset_server: Res<AssetServer>,
    //DECOMENTARE nella versione finale
    //mondo: Vec<Vec<Tile>>,
){
    //Genera il mondo, TOGLIERE nella versione finale
    let mut mondo: Vec<Vec<Tile>> = Vec::new();
    let mut v: Vec<Tile> = vec![];
    v.push(Tile{tile_type: DeepWater,content: Fire,elevation: 0,});
    v.push(Tile{tile_type: DeepWater,content: Fire,elevation: 0,});
    v.push(Tile{tile_type: ShallowWater,content: Fire,elevation: 0,});
    v.push(Tile{tile_type: ShallowWater,content: Fire,elevation: 0,});
    v.push(Tile{tile_type: Street,content: Fire,elevation: 0,});
    v.push(Tile{tile_type: Grass,content: Fire,elevation: 0,});
    v.push(Tile{tile_type: Grass,content: Fire,elevation: 0,});
    mondo.push(v.clone());
    let mut v2: Vec<Tile> = vec![];
    v2.push(Tile{tile_type: DeepWater,content: Fire,elevation: 0,});
    v2.push(Tile{tile_type: DeepWater,content: Fire,elevation: 0,});
    v2.push(Tile{tile_type: ShallowWater,content: Fire,elevation: 0,});
    v2.push(Tile{tile_type: ShallowWater,content: Fire,elevation: 0,});
    v2.push(Tile{tile_type: Street,content: Fire,elevation: 0,});
    v2.push(Tile{tile_type: Grass,content: Fire,elevation: 0,});
    v2.push(Tile{tile_type: Grass,content: Fire,elevation: 0,});
    mondo.push(v2.clone());
    mondo.push(v.clone());
    mondo.push(v2.clone());
    mondo.push(v.clone());
    mondo.push(v2.clone());
    mondo.push(v.clone());
    show_world(command, asset_server, mondo);
}




