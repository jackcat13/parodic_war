use std::cell::{RefCell, RefMut};
use std::rc::Rc;
use raylib::color::Color;
use raylib::prelude::{Camera2D, RaylibDraw, RaylibMode2DExt, Vector2};
use raylib::prelude::KeyboardKey::{KEY_D, KEY_Q, KEY_S, KEY_Z};
use crate::model::character::{Character, SPRITE_DOWN_LEFT, SPRITE_DOWN_RIGHT, SPRITE_DOWN_ROW, SPRITE_DOWN_UP, SPRITE_STAND_ROW};
use crate::model::game::Game;
use crate::raylib_wrapper::draw_handle::DrawRectangle;
use crate::raylib_wrapper::wrapper::Window;

pub fn game(window: Rc<RefCell<Window>>, game: Rc<RefCell<Game>>) {
    let (monitor_width, monitor_height) = window.clone().borrow_mut().get_size();
    let mut camera: Camera2D = Camera2D {
        offset: Vector2 { x: monitor_width as f32 / 2.0, y: monitor_height as f32 / 2.0 },
        target: Vector2 { x: 0.0, y: 0.0 },
        rotation: 0.0,
        zoom: 2.0,
    };

    while !window.clone().borrow_mut().window_should_close() {
        reprocess_coordinates(&game, window.clone());
        draw(window.clone(), &game, &mut camera);
    }
}

fn reprocess_coordinates(game: &Rc<RefCell<Game>>, window: Rc<RefCell<Window>>) {
    let window_borrow = window.borrow_mut();
    let mut game_borrow = game.borrow_mut();
    check_movement(game_borrow.team.characters.first_mut().unwrap(), window_borrow);
}

fn check_movement(character: &mut Character, window_borrow: RefMut<Window>) {
    character.sprite.row_direction = SPRITE_STAND_ROW;
    if window_borrow.rl.is_key_down(KEY_S) {
        character.position.y += character.speed as f32;
        character.sprite.row_direction = SPRITE_DOWN_ROW;
    }
    if window_borrow.rl.is_key_down(KEY_Z) {
        character.position.y -= character.speed as f32;
        character.sprite.row_direction = SPRITE_DOWN_UP;
    }
    if window_borrow.rl.is_key_down(KEY_D) {
        character.position.x += character.speed as f32;
        character.sprite.row_direction = SPRITE_DOWN_RIGHT;
    }
    if window_borrow.rl.is_key_down(KEY_Q) {
        character.position.x -= character.speed as f32;
        character.sprite.row_direction = SPRITE_DOWN_LEFT;
    }
}

pub fn draw(window: Rc<RefCell<Window>>, game: &Rc<RefCell<Game>>, camera: &mut Camera2D) {
    let mut window_borrow = window.borrow_mut();
    let mut draw_handle = window_borrow.begin_drawing();
    let mut mode_2d = draw_handle.raylib_draw_handle.begin_mode2D(*camera);
    let mut game_borrow = game.borrow_mut();

    mode_2d.clear_background(Color::GRAY);

    let character: &mut Character = game_borrow.team.characters.first_mut().unwrap();
    camera.target = character.position;
    character.print_sprite(&mut mode_2d,
                           DrawRectangle{
                               x: 0.0,
                               y: character.sprite.row_direction as f32,
                               width: character.size.x,
                               height: character.size.y,
                           },
                           character.position,
                           Color::WHITE);

    mode_2d.draw_text("test", 20, 20, 12, Color::BLACK);
}
