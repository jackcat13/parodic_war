use std::cell::RefCell;
use std::rc::Rc;
use raylib::color::Color;
use crate::model::game::Game;
use crate::raylib_wrapper::draw_handle::DrawRectangle;
use crate::raylib_wrapper::wrapper::Window;

pub fn game(window: Rc<RefCell<Window>>, game: Game) {



    while !window.clone().borrow_mut().window_should_close() {
        draw(&window.clone(), &game);
    }
}

pub fn draw(window: &Rc<RefCell<Window>>, game: &Game) {
    let mut window_borrow = window.borrow_mut();
    let mut draw_handle = window_borrow.begin_drawing();
    draw_handle.clear_background(Color::BROWN);

    game.team.characters.first().iter().for_each(|character| {
        character.print_sprite(&mut draw_handle,
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
