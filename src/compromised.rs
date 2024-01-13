extern crate amethyst;

use amethyst::{
    assets::{AssetStorage, Handle, Loader},
    core::Transform,
    ecs::{Component, DenseVecStorage},
    prelude::*,
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
};

pub struct Compromised;

impl SimpleState for Compromised {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        let carwell_sprite_sheet_handle = load_carwell_sprite(world);
        let block_sprite_sheet_handle = load_block_sprite(world);
        world.register::<Carwell>();
        initialise_carwell(world, carwell_sprite_sheet_handle);
        world.register::<Block>();
        initialise_block(world, block_sprite_sheet_handle);
        initialise_camera(world);
    }
}

pub const AREA_WIDTH: f32 = 100.0;
pub const AREA_HEIGHT: f32 = 100.0;

fn initialise_camera(world: &mut World) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(AREA_WIDTH * 0.5, AREA_HEIGHT * 0.5, 1.0);
    world
        .create_entity()
        .with(Camera::standard_2d(AREA_WIDTH, AREA_HEIGHT))
        .with(transform)
        .build();
}

pub const CAR_HEIGHT: f32 = 18.0;
pub const CAR_WIDTH: f32 = 10.0;

pub struct Carwell {
    pub width: f32,
    pub height: f32,
}

impl Carwell {
    fn new() -> Carwell {
        Carwell {
            width: CAR_WIDTH,
            height: CAR_HEIGHT,
        }
    }
}

impl Component for Carwell {
    type Storage = DenseVecStorage<Self>;
}

fn initialise_carwell(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>) {
    let sprite_render = SpriteRender::new(sprite_sheet_handle, 0);
    let mut max_transform = Transform::default();
    max_transform.set_translation_xyz(AREA_WIDTH * 0.275, 15.0, 0.0);
    world
        .create_entity()
        .with(sprite_render)
        .with(Carwell::new())
        .with(max_transform)
        .build();
}

fn load_carwell_sprite(world: &mut World) -> Handle<SpriteSheet> {
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            "carwell/carwell1.png",
            ImageFormat::default(),
            (),
            &texture_storage,
        )
    };
    let loader = world.read_resource::<Loader>();
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        "carwell/carwell.ron",
        SpriteSheetFormat(texture_handle),
        (),
        &sprite_sheet_store,
    )
}

pub const BLOCK_WIDTH: f32 = 50.0;
pub const BLOCK_HEIGHT: f32 = 7.0;

pub struct Block {
    pub width: f32,
    pub height: f32,
}

impl Block {
    fn new() -> Block {
        Block {
            width: BLOCK_WIDTH,
            height: BLOCK_HEIGHT,
        }
    }
}

impl Component for Block {
    type Storage = DenseVecStorage<Self>;
}



fn initialise_block(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>) {
    let sprite_render = SpriteRender::new(sprite_sheet_handle, 0);
    let mut block_transform = Transform::default();
    block_transform.set_translation_xyz(AREA_WIDTH * 0.75, 15.0, -0.01);
    world
        .create_entity()
        .with(sprite_render)
        .with(Block::new())
        .with(block_transform)
        .build();
}

fn load_block_sprite(world: &mut World) -> Handle<SpriteSheet> {
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            "block/block.png",
            ImageFormat::default(),
            (),
            &texture_storage,
        )
    };
    let loader = world.read_resource::<Loader>();
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        "block/block.ron",
        SpriteSheetFormat(texture_handle),
        (),
        &sprite_sheet_store,
    )
}
