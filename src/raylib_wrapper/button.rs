//! Hud button code

use std::cell::RefCell;
use std::rc::Rc;
use crate::raylib_wrapper::draw_handle::{DrawHandle, DrawRectangle};
use raylib::math::Vector2;
use crate::raylib_wrapper::wrapper::Window;

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

    pub fn action_on_click(&self, window: Rc<RefCell<Window>>) {
        let maybe_click_coordinates;
        {
            maybe_click_coordinates = window.borrow_mut().maybe_click_coordinates();
        }
        if let Ok(maybe_click_coordinates) = maybe_click_coordinates {
            if self.click_in_hitbox(&maybe_click_coordinates) {
                (self.action)();
            }
        }
    }

    fn click_in_hitbox(&self, coordinates: &Vector2) -> bool {
        coordinates.x >= self.rectangle.x
            && coordinates.x <= (self.rectangle.x + self.rectangle.width)
            && coordinates.y >= self.rectangle.y
            && coordinates.y <= (self.rectangle.y + self.rectangle.height)
    }
}