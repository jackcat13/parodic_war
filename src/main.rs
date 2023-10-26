use crate::raylib_wrapper::button::Button;
use crate::raylib_wrapper::draw_handle::DrawRectangle;
use crate::raylib_wrapper::wrapper::{Window, WindowBuilder};
use crate::screens::character_selection::character_selection;
use crate::screens::options::options;
use crate::screens::skills_tree::skills_tree;
use raylib::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;
use raylib::ffi::MaximizeWindow;

mod model;
mod procedural;
mod raylib_wrapper;
mod screens;

#[tokio::main]
async fn main() {
    let window = Rc::new(RefCell::new(
        WindowBuilder::init()
            .size(800, 600)
            .resizable()
            .title("Parodic War")
            .build(),
    ));
    unsafe { MaximizeWindow(); }
    let (monitor_width, _) = window.clone().borrow_mut().get_size();

    let window_closure = window.clone();
    let game_start_button = Button {
        rectangle: DrawRectangle {
            x: ((monitor_width / 2) - 50) as f32,
            y: 100.0,
            width: 100.0,
            height: 30.0,
        },
        text: "Game start".to_string(),
        action: Box::new(move || character_selection(window_closure.clone())),
    };
    let window_closure = window.clone();
    let skills_tree_button = Button {
        rectangle: DrawRectangle {
            x: ((monitor_width / 2) - 50) as f32,
            y: 150.0,
            width: 100.0,
            height: 30.0,
        },
        text: "Skills tree".to_string(),
        action: Box::new(move || skills_tree(window_closure.clone())),
    };
    let window_closure = window.clone();
    let options_button = Button {
        rectangle: DrawRectangle {
            x: ((monitor_width / 2) - 50) as f32,
            y: 200.0,
            width: 100.0,
            height: 30.0,
        },
        text: "Options".to_string(),
        action: Box::new(move || options(window_closure.clone())),
    };

    let mut buttons = vec![game_start_button, skills_tree_button, options_button];

    let window_loop = window.clone();
    while !window_loop.clone().borrow_mut().window_should_close() {
        let (monitor_width, _) = window.clone().borrow_mut().get_size();
        reprocess_coordinates(monitor_width, &mut buttons);
        draw_buttons(window_loop.clone(), &buttons);
        update_game(window_loop.clone(), &buttons);
    }
}

fn reprocess_coordinates(monitor_width: i32, buttons: &mut Vec<Button>) {
    buttons.iter_mut().for_each(|button: &mut Button| {
        button.rectangle.x = ((monitor_width / 2) - 50) as f32;
    });
}

pub fn draw_buttons(window: Rc<RefCell<Window>>, buttons: &Vec<Button>) {
    let mut window_borrow = window.borrow_mut();
    let mut draw_handle = window_borrow.begin_drawing();
    draw_handle.clear_background(Color::WHITE);
    buttons.iter().for_each(|button| {
        button.draw(&mut draw_handle);
    })
}

pub fn update_game(window: Rc<RefCell<Window>>, buttons: &Vec<Button>) {
    buttons.iter().for_each(|button| {
        button.action_on_click(window.clone());
    })
}
