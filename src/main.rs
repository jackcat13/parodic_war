use crate::raylib_wrapper::button::Button;
use crate::raylib_wrapper::draw_handle::DrawRectangle;
use crate::raylib_wrapper::wrapper::{Window, WindowBuilder};
use crate::screens::character_selection::character_selection;
use crate::screens::options::options;
use crate::screens::skills_tree::skills_tree;
use raylib::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

mod model;
mod raylib_wrapper;
mod screens;

const WINDOW_HEIGHT: i32 = 800;
const WINDOW_WIDTH: i32 = 600;

#[tokio::main]
async fn main() {
    let window = Rc::new(RefCell::new(
        WindowBuilder::init()
            .size(WINDOW_WIDTH, WINDOW_HEIGHT)
            .resizable()
            .title("Parodic War")
            .build(),
    ));

    let window_closure = window.clone();
    let game_start_button = Button {
        rectangle: DrawRectangle {
            x: ((WINDOW_WIDTH / 2) - 50) as f32,
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
            x: ((WINDOW_WIDTH / 2) - 50) as f32,
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
            x: ((WINDOW_WIDTH / 2) - 50) as f32,
            y: 200.0,
            width: 100.0,
            height: 30.0,
        },
        text: "Options".to_string(),
        action: Box::new(move || options(window_closure.clone())),
    };

    let buttons = vec![&game_start_button, &skills_tree_button, &options_button];

    let window_loop = window.clone();
    while !window_loop.clone().borrow_mut().window_should_close() {
        draw_buttons(window_loop.clone(), &buttons);
        update_game(window_loop.clone(), &buttons);
    }
}

pub fn draw_buttons(window: Rc<RefCell<Window>>, buttons: &Vec<&Button>) {
    let mut window_borrow = window.borrow_mut();
    let mut draw_handle = window_borrow.begin_drawing();
    draw_handle.clear_background(Color::WHITE);
    buttons.iter().for_each(|button| {
        button.draw(&mut draw_handle);
    })
}

pub fn update_game(window: Rc<RefCell<Window>>, buttons: &Vec<&Button>) {
    buttons.iter().for_each(|button| {
        button.action_on_click(window.clone());
    })
}
