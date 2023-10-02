use crate::raylib_wrapper::button::Button;
use crate::raylib_wrapper::draw_handle::DrawRectangle;
use crate::raylib_wrapper::wrapper::Window;
use crate::update_game;
use std::cell::RefCell;
use std::rc::Rc;
use raylib::color::Color;

pub fn options(window: Rc<RefCell<Window>>) {
    let back_action = Rc::new(RefCell::new(false));

    let back_action_closure = back_action.clone();
    let back_button = Button {
        rectangle: DrawRectangle {
            x: 50.0,
            y: 500.0,
            width: 50.0,
            height: 50.0,
        },
        text: "Back".to_string(),
        action: Box::new(move || {
            *back_action_closure.borrow_mut() = true;
        }),
    };

    let buttons = vec![back_button];

    while !window.clone().borrow_mut().window_should_close() && !*back_action.borrow() {
        draw(window.clone(), &buttons);
        update_game(window.clone(), &buttons);
    }
}

pub fn draw(window: Rc<RefCell<Window>>, buttons: &Vec<Button>) {
    let mut window_borrow = window.borrow_mut();
    let mut draw_handle = window_borrow.begin_drawing();
    draw_handle.clear_background(Color::WHITE);
    buttons.iter().for_each(|button| {
        button.draw(&mut draw_handle);
    });
    draw_handle.draw_text("TODO", 100, 100, 12, Color::BLACK);
}
