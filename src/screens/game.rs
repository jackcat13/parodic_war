use std::cell::RefCell;
use std::rc::Rc;
use raylib::color::Color;
use crate::model::game::Game;
use crate::raylib_wrapper::wrapper::Window;

pub fn game(window: Rc<RefCell<Window>>, game: Game) {

    while !window.clone().borrow_mut().window_should_close() {
        draw(window.clone());
    }
}

pub fn draw(window: Rc<RefCell<Window>>) {
    let mut window_borrow = window.borrow_mut();
    let mut draw_handle = window_borrow.begin_drawing();
    draw_handle.clear_background(Color::WHITE);
    draw_handle.draw_text("TODO", 100, 100, 12, Color::BLACK);
}
