use macroquad::prelude::*;

use macroquad::experimental::{
    collections::storage,
    scene::{Node, RefMut},
};

use macroquad_platformer::Actor;
use macroquad_platformer::*;
use macroquad_tiled as tiled;

struct Resources {
    whale: Texture2D,
    physics: World,
}

struct Player {
    collider: Actor,
    speed: Vec2,
}

impl Player {
    pub const JUMP_SPEED: f32 = -700.0;
    pub const GRAVITY: f32 = 2000.0;
    pub const MOVE_SPEED: f32 = 300.0;

    fn new() -> Self {
        let mut resources = storage::get_mut::<Resources>();
        Self {
            collider: resources.physics.add_actor(vec2(200.0, 100.0), 36, 66),
            speed: vec2(0., 0.),
        }
    }
}

impl Node for Player {
    fn draw(node: RefMut<Self>) {

        let resources = storage::get_mut::<Resources>();

        let pos = resources.physics.actor_pos(node.collider);

        draw_texture_ex(
            resources.whale,
            pos.x - 20.,
            pos.y,
            WHITE,
            DrawTextureParams {
                source: Some(Rect::new(0.0, 0.0, 76., 66.)),
                ..Default::default()
            },
        );
    }

    fn update(mut node: RefMut<Self>) {
        let world = &mut storage::get_mut::<Resources>().physics;

        let pos = world.actor_pos(node.collider);
        let on_ground = world.collide_check(node.collider, pos + vec2(0., 1.));

        if on_ground == false {
            node.speed.y += Self::GRAVITY * get_frame_time();
        }

        if is_key_down(KeyCode::Right) {
            node.speed.x = Self::MOVE_SPEED;
        } else if is_key_down(KeyCode::Left) {
            node.speed.x = -Self::MOVE_SPEED;
        } else {
            node.speed.x = 0.;
        }

        if is_key_pressed(KeyCode::Space) {
            if on_ground {
                node.speed.y = Self::JUMP_SPEED;
            }
        }

        world.move_h(node.collider, node.speed.x * get_frame_time());
        world.move_v(node.collider, node.speed.y * get_frame_time());
    }
}

mod consts {
    pub const JUMP_SPEED: f32 = -700.0;
    pub const GRAVITY: f32 = 2000.0;
    pub const MOVE_SPEED: f32 = 300.0;
}

#[macroquad::main("FishFash")]

async fn main() {
    let tileset = load_texture("assets/tileset.png").await.unwrap();
    tileset.set_filter(FilterMode::Nearest);

    let decorations = load_texture("assets/decorations1.png").await.unwrap();
    decorations.set_filter(FilterMode::Nearest);

    let tiled_map_json = load_string("assets/map.json").await.unwrap();
    let tiled_map = tiled::load_map(
        &tiled_map_json,
        &[("tileset.png", tileset), ("decorations1.png", decorations)],
        &[],
    )
    .unwrap();

    let mut static_colliders: Vec<macroquad_platformer::Tile> = vec![];
    for (_x, _y, tile) in tiled_map.tiles("main layer", None) {
        static_colliders.push(if tile.is_some() {
            Tile::Solid
        } else {
            Tile::Empty
        }); // Some(tile) ??
    }

    for (_x, _y, tile) in tiled_map.tiles("main layer", None) {
        if tile.is_some() {
            draw_circle(_x as f32, _y as f32, 20., RED);
        }
    }

    let mut physics = World::new();
    physics.add_static_tiled_layer(
        static_colliders,
        tiled_map.raw_tiled_map.tilewidth as f32,
        tiled_map.raw_tiled_map.tileheight as f32,
        tiled_map.raw_tiled_map.width as _,
        1,
    );

    let whale = load_texture("assets/Whale/Whale(76x66)(Orange).png")
        .await
        .unwrap();

    let resources = Resources { whale, physics };
    storage::store(resources);

    let player = Player::new();

    scene::add_node(player);
 
    let width = tiled_map.raw_tiled_map.tilewidth * tiled_map.raw_tiled_map.width;
    let height = tiled_map.raw_tiled_map.tileheight * tiled_map.raw_tiled_map.height;

    loop {
        clear_background(BLACK);

        tiled_map.draw_tiles(
            "main layer",
            Rect::new(0.0, 0.0, width as _, height as _),
            None,
        );
        
        next_frame().await;
    }
}
