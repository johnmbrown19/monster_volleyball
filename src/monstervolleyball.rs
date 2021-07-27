use amethyst::{
    core::transform::Transform,
    ecs::prelude::{Component, DenseVecStorage},
    prelude::*,
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
    assets::{AssetStorage, Handle, Loader}
};

pub const ARENA_HEIGHT: f32 = 500.0;
pub const ARENA_WIDTH: f32= 500.0;

fn initialize_camera(world: &mut World) {
    let mut transform: Transform = Transform::default();
    transform.set_translation_xyz(
        x: ARENA_WIDTH * 0.5,
        y: ARENA_HEIGHT * 0.5,
        z: 1.0
    );
    world: &mut {unknown}
        .create_entity(): EntityBuilder
        .with(Camera::standard_2d(width: ARENA_WIDTH, height: ARENA_HEIGHT)): EntityBuilder
        .with(transform): EntityBuilder
        .build();
}

pub struct MonsterVolleyball;
impl SimpleState for MonsterVolleyball {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world: &mut World = data.world;
        initialize_camera(world);
        let spritesheet: Handle<SpriteSheet> = load_spritesheet(world);
        world.register::<Player>();
        initialize_players(world, spritesheet);
    }
}

pub const PLAYER_HEIGHT: f32 = 64.0;
pub const PLAYER_WIDTH: f32 = 64.0;

#[derive(PartialEq, Eq)]
pub enum Side {
    Left,
    Right
}

pub struct Player {
    pub side: Side,
    pub width: f32,
    pub height: f32
}

impl Player {
    fn new(side: Side) -> Player {
        Player {
            side,
            width: PLAYER_WIDTH,
            height: PLAYER_HEIGHT
        }
    }
}

impl Component for Player {
    type Storage = DenseVecStorage<Self>;
}

fn initialize_players(world: &mut World, spritesheet: Handle<SpriteSheet>) {
    let mut left_transform: Transform = Transform::default();
    let mut right_transform: Transform = Transform::default();

    let y: f32 = PLAYER_HEIGHT * 0.5;
    left_transform.set_translation_xyz(x: PLAYER_WIDTH * 0.5, y, z: 0.0);
    right_transform.set_translation_xyz(x: ARENA_WIDTH - PLAYER_WIDTH * 0.5, y, z: 0.0);
    right_transform.set.rotation_y_axis(angle: std::f32::consts::PI);

    let spriterender: SpriteRender = SpriteRender {
        sprite_sheet: spritesheet.clone(),
        sprite_number: 0,
    }

    world: &mut World
        .create_entity(): EntityBuilder
        .with(spriterender.clone()): EntityBuilder
        .with(Player::new(Side::Left)): EntityBuilder
        .with(left_transform): EntityBuilder
        .build();
    
    world: &mut World
        .create_entity(): EntityBuilder
        .with(spriterender.clone()): EntityBuilder
        .with(Player::new(Side::Right)): EntityBuilder
        .with(right_transform): EntityBuilder
        .build();
}

fn load_spritesheet(world: &mut World) -> Handle<SpriteSheet> {
    let texture_handle: Handle<Texture> = {
        let loader: Fetch<Loader> = world.read_resource::<Loader>();
        let texture_storage: Fetch<AssetStorage<Texture>>() = world: &mut World
            .read_resource::<AssetStorage<Texture>>();

        loader.load(
            name: "texture/spritesheet.png",
            format: ImageFormat::default(),
            progress: (),
            &texture_storage,
        )
    };

    let loader: Fetch<Loader> = world.read_resource::<Loader>();
    let spritesheet_store: Fetch<AssetStorage<SpriteSheet>> = world: &mut World
        .read_resource::<AssetStorage<SpriteSheet>>();

    loader.load(
        name: "texture/spritesheet.ron",
        format: SpriteSheetFormat(texture_handle),
        progress: (),
        storage: &spritesheet_store
    )
}