//! Raylib wrapper code

//in mod.rs

use crate::raylib_wrapper::draw_handle::DrawHandle;
use raylib::ffi::MouseButton;
use raylib::math::Vector2;
use raylib::{RaylibBuilder, RaylibHandle, RaylibThread};
use raylib::prelude::Texture2D;

pub struct Window {
    pub rl: RaylibHandle,
    pub(crate) thread: RaylibThread,
}

impl Window {
    pub fn set_target_fps(&mut self, fps: u32) {
        self.rl.set_target_fps(fps);
    }

    pub fn window_should_close(&mut self) -> bool {
        self.rl.window_should_close()
    }

    pub fn begin_drawing(&mut self) -> DrawHandle {
        DrawHandle {
            raylib_draw_handle: self.rl.begin_drawing(&self.thread),
        }
    }

    pub fn maybe_click_coordinates(&self) -> Result<Vector2, &'static str> {
        if self
            .rl
            .is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT)
        {
            Ok(self.rl.get_mouse_position())
        } else {
            Err("No click")
        }
    }

    pub fn load_texture(&mut self, path: &str) -> Texture2D {
        match self.rl.load_texture(&self.thread, path) {
            Ok(texture) => { texture }
            Err(error) => { panic!("Could not load sprite with following path : {}. It happened because of following error : {}", path, error); }
        }
    }

    pub fn get_size(&self) -> (i32, i32) {
        (self.rl.get_screen_width(), self.rl.get_screen_height())
    }
}

pub struct WindowBuilder {
    raylib_builder: RaylibBuilder,
}

impl WindowBuilder {
    pub fn init() -> WindowBuilder {
        WindowBuilder {
            raylib_builder: raylib::init(),
        }
    }

    pub fn size(&mut self, w: i32, h: i32) -> &mut Self {
        self.raylib_builder.size(w, h);
        self
    }

    pub fn resizable(&mut self) -> &mut Self {
        self.raylib_builder.resizable();
        self
    }

    pub fn title(&mut self, title: &str) -> &mut Self {
        self.raylib_builder.title(title);
        self
    }

    pub fn build(&self) -> Window {
        let (rl, thread) = self.raylib_builder.build();
        let mut window = Window { rl, thread };
        window.set_target_fps(60);
        window
    }
}
