use std::cell::RefCell;
use std::rc::Rc;
use raylib::drawing::{RaylibDrawHandle, RaylibMode2D};
use raylib::prelude::{Color, RaylibDraw, Rectangle, Texture2D, Vector2};
use crate::model::equipement::Equipement;
use crate::model::skills::Skill;
use crate::raylib_wrapper::draw_handle::DrawRectangle;
use crate::raylib_wrapper::wrapper::Window;

pub struct Sprite {
    pub texture: Texture2D,
    pub offset: i32,
    pub row_direction: i32,
}

pub struct Character {
    pub name: String,
    pub hp: i32,
    pub mana: i32,
    pub armor: i32,
    pub damage: i32,
    pub speed: i32,
    pub equipement: Equipement,
    pub skills: Vec<Skill>,
    pub sprite: Sprite,
    pub position: Vector2,
    pub size: Vector2,
}

const SPRITE_COLUMNS: i32 = 9;
pub const SPRITE_STAND_ROW: i32 = 0;
pub const SPRITE_DOWN_ROW: i32 = 60;
pub const SPRITE_DOWN_LEFT: i32 = 120;
pub const SPRITE_DOWN_RIGHT: i32 = 180;
pub const SPRITE_DOWN_UP: i32 = 240;

impl Character {

    pub fn print_sprite(&mut self, draw_handle: &mut RaylibMode2D<RaylibDrawHandle>, frame_rec: DrawRectangle, position: Vector2, color: Color) {
        if self.sprite.offset > SPRITE_COLUMNS { self.sprite.offset = 0 };
        draw_sprite(&self.sprite, frame_rec, position, color, draw_handle);
        self.sprite.offset += 1;
    }
}

fn draw_sprite(sprite: &Sprite, frame_rec: DrawRectangle, position: Vector2, color: Color, handler: &mut RaylibMode2D<RaylibDrawHandle>) {
    let frame_rec = Rectangle {
        x: frame_rec.x + (frame_rec.width * sprite.offset as f32),
        y: frame_rec.y,
        width: frame_rec.width,
        height: frame_rec.height,
    };
    handler.draw_texture_rec(&sprite.texture, frame_rec, position, color);
}

pub fn crad(window: Rc<RefCell<Window>>, position: Vector2) -> Character {
    Character {
        name: "Crad".to_string(),
        hp: 100,
        mana: 30,
        armor: 10,
        damage: 30,
        speed: 2,
        equipement: Equipement {
            weapon: None,
            helmet: None,
            chest: None,
            legs: None,
            shoes: None,
        },
        skills: vec![],
        sprite: Sprite {
            texture: window.borrow_mut().load_texture("./static/sprites/crad/crad.png"),
            offset: 0,
            row_direction: 0,
        },
        position: position,
        size: Vector2 {
            x: 50.0,
            y: 60.0,
        },
    }
}