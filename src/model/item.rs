use raylib::ffi::Vector2;
use crate::model::character::Sprite;

#[derive(Debug)]
pub struct Item {
    pub item_type: ItemType,
    pub position: Vector2,
    pub sprite: Sprite,
    pub size: Vector2,
    pub hp: i8,
}

#[derive(Debug)]
pub enum ItemType {
    TREE, STONE
}

#[derive(Debug)]
pub struct DroppedItem {
    pub item_type: DroppedItemType,
    pub position: Vector2,
    pub sprite: Sprite,
    pub size: Vector2,
}

#[derive(Debug)]
pub enum DroppedItemType {
    TREE, STONE
}