//! Hud button code

use crate::raylib_wrapper::draw_handle::{DrawHandle, DrawRectangle};
use raylib::math::Vector2;

pub struct Button {
    pub rectangle: DrawRectangle,
    pub text: String,
    pub action: Box<dyn Fn()>,
}

impl Button {
    pub fn draw(&self, draw_handle: &mut DrawHandle) -> bool {
        draw_handle.gui_button(
            DrawRectangle {
                x: self.rectangle.x,
                y: self.rectangle.y,
                width: self.rectangle.width,
                height: self.rectangle.height,
            },
            &self.text,
        )
    }

    pub fn click_in_hitbox(&self, coordinates: &Vector2) -> bool {
        coordinates.x >= self.rectangle.x
            && coordinates.x <= (self.rectangle.x + self.rectangle.width)
            && coordinates.y >= self.rectangle.y
            && coordinates.y <= (self.rectangle.y + self.rectangle.height)
    }
}
