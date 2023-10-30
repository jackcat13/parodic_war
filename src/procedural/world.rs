use std::cell::RefCell;
use std::rc::Rc;
use rand::Rng;
use raylib::ffi::Vector2;
use crate::model::character::Sprite;
use crate::model::item::{DroppedItem, Item};
use crate::model::item::ItemType::{STONE, TREE};
use crate::model::world::World;
use crate::raylib_wrapper::wrapper::Window;

const X_MIN: f32 = -10000.0;
const X_MAX: f32 = 10000.0;
const Y_MIN: f32 = -10000.0;
const Y_MAX: f32 = 10000.0;
const CASUAL_ITEMS_NUMBER: i32 = 300;
const STONE_SIZE: Vector2 = Vector2 { x: 50.0, y: 30.0 };
const STONE_REC: Vector2 = Vector2 { x: 25.0, y: 30.0 };
const TREE_SIZE: Vector2 = Vector2 { x: 100.0, y: 100.0 };
const TREE_REC: Vector2 = Vector2 { x: 50.0, y: 100.0 };

pub fn generate_random_world (window: &Rc<RefCell<Window>>) -> World {
    let mut items: Vec<Item> = vec![];
    let mut dropped_items: Vec<DroppedItem> = vec![];
    let mut rng = rand::thread_rng();
    for _ in 1..CASUAL_ITEMS_NUMBER {
        let mut window_borrow = window.borrow_mut();
        let mut stone_texture = window_borrow.load_texture("./static/sprites/ore/stone.png");
        stone_texture.height = STONE_SIZE.y as i32; stone_texture.width = STONE_SIZE.x as i32;
        let mut tree_texture = window_borrow.load_texture("./static/sprites/environment/tree.png");
        tree_texture.height = TREE_SIZE.y as i32; tree_texture.width = TREE_SIZE.x as i32;
        items.push(Item {
            item_type: STONE,
            position: Vector2 { x: rng.gen_range(X_MIN..X_MAX), y: rng.gen_range(Y_MIN..Y_MAX) },
            sprite: Sprite {
                texture: stone_texture,
                offset: 0,
                row_direction: 0,
            },
            size: STONE_REC,
            hp: 3,
        });
        items.push(Item {
            item_type: TREE,
            position: Vector2 { x: rng.gen_range(X_MIN..X_MAX), y: rng.gen_range(Y_MIN..Y_MAX) },
            sprite: Sprite {
                texture: tree_texture,
                offset: 0,
                row_direction: 0,
            },
            size: TREE_REC,
            hp: 5,
        });
    }
    World {
        items,
        dropped_items,
    }
}