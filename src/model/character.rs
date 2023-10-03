use std::cell::RefCell;
use std::rc::Rc;
use raylib::drawing::{RaylibDrawHandle, RaylibMode2D};
use raylib::prelude::{Color, RaylibDraw, Rectangle, Texture2D, Vector2};
use crate::model::equipement::Equipement;
use crate::model::skills::Skill;
use crate::raylib_wrapper::draw_handle::DrawRectangle;
use crate::raylib_wrapper::wrapper::Window;

pub struct Character {
    pub name: String,
    pub hp: i32,
    pub mana: i32,
    pub armor: i32,
    pub damage: i32,
    pub speed: i32,
    pub equipement: Equipement,
    pub skills: Vec<Skill>,
    pub sprite: Texture2D,
    pub position: Vector2,
    pub size: Vector2,
}

impl Character {

    pub fn print_sprite(&self, draw_handle: &mut RaylibMode2D<RaylibDrawHandle>, frame_rec: DrawRectangle, position: Vector2, color: Color) {
        draw_sprite(&self.sprite, frame_rec, position, color, draw_handle)
    }
}

fn draw_sprite(sprite: &Texture2D, frame_rec: DrawRectangle, position: Vector2, color: Color, handler: &mut RaylibMode2D<RaylibDrawHandle>) {
    let frame_rec = Rectangle {
        x: frame_rec.x,
        y: frame_rec.y,
        width: frame_rec.width,
        height: frame_rec.height,
    };
    handler.draw_texture_rec(sprite, frame_rec, position, color);
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
        sprite: window.borrow_mut().load_texture("./static/sprites/crad/crad.png"),
        position: position,
        size: Vector2 {
            x: 50.0,
            y: 60.0,
        },
    }
}