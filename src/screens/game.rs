use std::cell::RefCell;
use std::rc::Rc;
use raylib::color::Color;
use raylib::prelude::{Camera2D, RaylibDraw, RaylibMode2DExt, Vector2};
use crate::model::game::Game;
use crate::raylib_wrapper::draw_handle::DrawRectangle;
use crate::raylib_wrapper::wrapper::Window;

pub fn game(window: Rc<RefCell<Window>>, game: Game) {

    let (monitor_width, monitor_height) = window.clone().borrow_mut().get_size();
    let main_character = game.team.characters.first().unwrap();
    let mut camera : Camera2D = Camera2D{
        offset: Vector2 { x: monitor_width as f32/2.0, y: monitor_height as f32/2.0 },
        target: Vector2 { x: main_character.position.x + 20.0, y: main_character.position.y + 20.0 },
        rotation: 0.0,
        zoom: 2.0,
    };

    while !window.clone().borrow_mut().window_should_close() {
        camera.target = main_character.position;
        draw(window.clone(), &game, camera);
    }
}

pub fn draw(window: Rc<RefCell<Window>>, game: &Game, camera: Camera2D) {
    let mut window_borrow = window.borrow_mut();
    let mut draw_handle = window_borrow.begin_drawing();
    let mut mode_2d = draw_handle.raylib_draw_handle.begin_mode2D(camera);
    mode_2d.clear_background(Color::GRAY);

    game.team.characters.first().iter().for_each(|character| {
        character.print_sprite(&mut mode_2d,
                               DrawRectangle{
                                   x: 0.0,
                                   y: 0.0,
                                   width: character.size.x,
                                   height: character.size.y,
                               },
                               character.position,
                               Color::WHITE);
    });
}
