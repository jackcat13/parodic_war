use crate::model::item::{DroppedItem, Item};

pub struct World {
    pub items: Vec<Item>,
    pub dropped_items: Vec<DroppedItem>
}