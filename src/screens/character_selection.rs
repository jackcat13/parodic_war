//! Character selection screen

use crate::raylib_wrapper::button::Button;
use crate::raylib_wrapper::draw_handle::DrawRectangle;
use crate::raylib_wrapper::wrapper::Window;
use crate::{draw_buttons, update_game};
use std::cell::RefCell;
use std::rc::Rc;

pub fn character_selection(window: Rc<RefCell<Window>>) {
    let back_action = Rc::new(RefCell::new(false));

    let crad_button = Button {
        rectangle: DrawRectangle {
            x: 50.0,
            y: 50.0,
            width: 50.0,
            height: 50.0,
        },
        text: "Crad".to_string(),
        action: Box::new(|| println!("TODO !")),
    };
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

    let buttons = vec![&crad_button, &back_button];

    while !window.clone().borrow_mut().window_should_close() && !*back_action.borrow() {
        draw_buttons(window.clone(), &buttons);
        update_game(window.clone(), &buttons);
    }
}
