use raylib::color::Color;
use raylib::drawing::{RaylibDraw, RaylibDrawHandle};
use raylib::math::Rectangle;
use raylib::prelude::RaylibDrawGui;
use std::ffi::CString;

#[derive(Copy, Clone, PartialEq)]
pub struct DrawRectangle {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

pub struct DrawHandle<'a> {
    pub(crate) raylib_draw_handle: RaylibDrawHandle<'a>,
}

impl<'a> DrawHandle<'a> {
    pub fn clear_background(&mut self, color: Color) {
        self.raylib_draw_handle.clear_background(color);
    }

    pub fn draw_text(&mut self, text: &str, x: i32, y: i32, font_size: i32, color: Color) {
        self.raylib_draw_handle
            .draw_text(text, x, y, font_size, color)
    }

    pub fn gui_button(&mut self, rectangle: DrawRectangle, text: &str) -> bool {
        self.raylib_draw_handle.gui_button(
            Rectangle {
                x: rectangle.x,
                y: rectangle.y,
                width: rectangle.width,
                height: rectangle.height,
            },
            Some(CString::new(text).unwrap().as_c_str()),
        )
    }
}
