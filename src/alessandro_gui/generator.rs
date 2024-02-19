use crate::alessandro_gui::main::{GLOBAL_CONTEXT, ROBOT_MODEL};
use robotics_lib::world::tile::Content;
use std::collections::HashMap;
use three_d::{
    vec3, Context, CpuModel, Cull, InstancedModel, Instances, Matrix4, Model, PhysicalMaterial,
};
use three_d_asset::io::RawAssets;
use three_d_asset::Mat4;

// struct to hold info about the 3d tiles of the map
pub struct MapTile {
    tipo: String,
    pos: Vec<Matrix4<f32>>,
    cpu_model: CpuModel,
    pub instanced_model: InstancedModel<PhysicalMaterial>,
    content_pos: Vec<Matrix4<f32>>,
}
impl MapTile {
    pub fn new(
        context: &Context,
        tipo: String,
        pos: Vec<Matrix4<f32>>,
        raw_assets: &mut RawAssets,
        content_pos: Vec<Matrix4<f32>>,
    ) -> MapTile {
        let cpu_model = create_tile_cpu_model(raw_assets, tipo.as_str());
        let instanced_model = create_tile_instanced_model(context, &&cpu_model, pos.clone());
        MapTile {
            tipo,
            pos,
            cpu_model,
            instanced_model,
            content_pos,
        }
    }
    pub fn change_pos(&mut self, pos: Vec<Matrix4<f32>>) {
        self.pos = pos
    }

    pub fn push_pos(&mut self, pos: Matrix4<f32>) {
        self.pos.push(pos)
    }

    pub fn pop_pos(&mut self, pos: Matrix4<f32>) {
        self.pos.retain(|&x| x != pos);
    }

    pub fn push_pos_istances(&mut self, pos: Matrix4<f32>) {
        self.content_pos.push(pos)
    }

    pub fn update_istances(&mut self, context: &Context) {
        self.instanced_model =
            create_tile_instanced_model(context, &self.cpu_model, self.pos.clone())
    }
}
// ---------------------------------------------------------
// struct to hold info about the 3d contents of the map
pub struct MapContent {
    tipo: String,
    pos: Vec<Matrix4<f32>>,
    cpu_model: CpuModel,
    pub instanced_model: InstancedModel<PhysicalMaterial>,
}

impl MapContent {
    pub fn new(
        context: &Context,
        tipo: String,
        pos: Vec<Matrix4<f32>>,
        raw_assets: &mut RawAssets,
    ) -> MapContent {
        let cpu_model = create_tile_cpu_model(raw_assets, tipo.as_str());
        let instanced_model = create_tile_instanced_model(context, &&cpu_model, pos.clone());
        MapContent {
            tipo,
            pos,
            cpu_model,
            instanced_model,
        }
    }
    pub fn change_pos(&mut self, pos: Vec<Matrix4<f32>>) {
        self.pos = pos
    }
    pub fn push_pos(&mut self, pos: Matrix4<f32>) {
        self.pos.push(pos)
    }
    pub fn pop_pos(&mut self, pos: Matrix4<f32>) {
        self.pos.retain(|&x| x != pos);
    }
    pub fn update_istances(&mut self, context: &Context) {
        self.instanced_model =
            create_tile_instanced_model(context, &self.cpu_model, self.pos.clone())
    }
}

// ---------------------------------------------------------
// function to create the cpu model
pub fn create_tile_cpu_model(raw_asset: &mut RawAssets, tipo: &str) -> CpuModel {
    let mut cpu_model: CpuModel = raw_asset.deserialize(tipo).unwrap();
    cpu_model.geometries.iter_mut().for_each(|g| {
        g.compute_normals();
        g.compute_tangents();
    });
    return cpu_model;
}

// function to create the instanced model
pub fn create_tile_instanced_model(
    context: &Context,
    cpu_model: &CpuModel,
    pos: Vec<Matrix4<f32>>,
) -> InstancedModel<PhysicalMaterial> {
    InstancedModel::new(
        &context,
        &Instances {
            transformations: pos,
            texture_transformations: None,
            colors: None,
        },
        cpu_model,
    )
    .unwrap()
}

// ------------------------------------------
// function to create the robot model
pub fn create_robot_model() {
    let mut l = three_d_asset::io::load(&["./assets/robot.glb"]).unwrap();
    let mut cpu_model: CpuModel = l.deserialize(".glb").unwrap();
    cpu_model
        .geometries
        .iter_mut()
        .for_each(|g| g.compute_normals());
    unsafe {
        let mut model1 =
            Model::<PhysicalMaterial>::new(&GLOBAL_CONTEXT.as_ref().unwrap(), &cpu_model).unwrap();
        model1.iter_mut().for_each(|m| {
            m.geometry
                .set_transformation(Mat4::from_translation(vec3(0.0, 0.0, 0.0)));
            m.material.render_states.cull = Cull::Back;
        });
        ROBOT_MODEL = Some(model1);
    }
}

pub fn create_3d_tile(context: &Context, tiles_glb: &mut RawAssets) -> Vec<Box<MapTile>> {
    // create the model for the tiles
    let mut tiles_vec: Vec<Box<MapTile>> = vec![];
    let deep_water: MapTile =
        MapTile::new(context, "deepWater".to_string(), vec![], tiles_glb, vec![]);
    tiles_vec.push(Box::new(deep_water));
    let shallow_water: MapTile = MapTile::new(
        context,
        "shallowWater".to_string(),
        vec![],
        tiles_glb,
        vec![],
    );
    tiles_vec.push(Box::new(shallow_water));
    let sand: MapTile = MapTile::new(context, "sand".to_string(), vec![], tiles_glb, vec![]);
    tiles_vec.push(Box::new(sand));
    let grass: MapTile = MapTile::new(context, "grass".to_string(), vec![], tiles_glb, vec![]);
    tiles_vec.push(Box::new(grass));
    let street: MapTile = MapTile::new(context, "street".to_string(), vec![], tiles_glb, vec![]);
    tiles_vec.push(Box::new(street));
    let hill: MapTile = MapTile::new(context, "hill".to_string(), vec![], tiles_glb, vec![]);
    tiles_vec.push(Box::new(hill));
    let mountain: MapTile =
        MapTile::new(context, "mountain".to_string(), vec![], tiles_glb, vec![]);
    tiles_vec.push(Box::new(mountain));
    let snow: MapTile = MapTile::new(context, "snow".to_string(), vec![], tiles_glb, vec![]);
    tiles_vec.push(Box::new(snow));
    let lava: MapTile = MapTile::new(context, "lava".to_string(), vec![], tiles_glb, vec![]);
    tiles_vec.push(Box::new(lava));
    let teleport: MapTile =
        MapTile::new(context, "teleport".to_string(), vec![], tiles_glb, vec![]);
    tiles_vec.push(Box::new(teleport));
    let wall: MapTile = MapTile::new(context, "wall".to_string(), vec![], tiles_glb, vec![]);
    tiles_vec.push(Box::new(wall));
    let unknown: MapTile = MapTile::new(context, "unknown".to_string(), vec![], tiles_glb, vec![]);
    tiles_vec.push(Box::new(unknown));
    tiles_vec
}

pub fn create_3d_content(
    context: &Context,
    content_glb: &mut RawAssets,
) -> HashMap<Content, MapContent> {
    // create the model for the contents
    let mut content_map: HashMap<Content, MapContent> = HashMap::new();
    content_map.insert(
        Content::Rock(0),
        MapContent::new(context, "rock".to_string(), vec![], content_glb),
    );
    content_map.insert(
        Content::Tree(0).to_default(),
        MapContent::new(context, "tree".to_string(), vec![], content_glb),
    );
    content_map.insert(
        Content::Garbage(0).to_default(),
        MapContent::new(context, "garbage".to_string(), vec![], content_glb),
    );
    content_map.insert(
        Content::Fire,
        MapContent::new(context, "fire".to_string(), vec![], content_glb),
    );
    content_map.insert(
        Content::Coin(0).to_default(),
        MapContent::new(context, "coin".to_string(), vec![], content_glb),
    );
    content_map.insert(
        Content::Bin(0..0).to_default(),
        MapContent::new(context, "bin".to_string(), vec![], content_glb),
    );
    content_map.insert(
        Content::Crate(0..0).to_default(),
        MapContent::new(context, "crate".to_string(), vec![], content_glb),
    );
    content_map.insert(
        Content::Bank(0..0).to_default(),
        MapContent::new(context, "bank".to_string(), vec![], content_glb),
    );
    content_map.insert(
        Content::Market(0).to_default(),
        MapContent::new(context, "market".to_string(), vec![], content_glb),
    );
    content_map.insert(
        Content::Fish(0).to_default(),
        MapContent::new(context, "fish".to_string(), vec![], content_glb),
    );
    content_map.insert(
        Content::Building.to_default(),
        MapContent::new(context, "building".to_string(), vec![], content_glb),
    );
    content_map.insert(
        Content::Bush(0).to_default(),
        MapContent::new(context, "bush".to_string(), vec![], content_glb),
    );
    content_map
}
