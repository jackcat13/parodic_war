use raylib::ffi::Vector2;
use crate::model::character::Sprite;

pub struct Item {
    pub item_type: ItemType,
    pub position: Vector2,
    pub sprite: Sprite,
    pub size: Vector2,
}

pub enum ItemType {
    TREE, STONE
}