use raylib::color::Color;
use raylib::drawing::{RaylibDraw, RaylibDrawHandle};
use raylib::math::Rectangle;
use raylib::prelude::{RaylibDrawGui, Texture2D, Vector2};
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

    pub fn draw_sprite(&mut self, texture: &Texture2D, frame_rec: DrawRectangle, position: Vector2, color: Color) {
        let frame_rec = Rectangle {
            x: frame_rec.x,
            y: frame_rec.y,
            width: frame_rec.width,
            height: frame_rec.height,
        };
        self.raylib_draw_handle.draw_texture_rec(texture, frame_rec, position, color);
    }
}
