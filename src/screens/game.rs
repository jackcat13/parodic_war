use std::cell::{RefCell, RefMut};
use std::rc::Rc;
use raylib::color::Color;
use raylib::prelude::{Camera2D, MouseButton, RaylibDraw, RaylibMode2DExt, Vector2};
use raylib::prelude::KeyboardKey::{KEY_D, KEY_DOWN, KEY_Q, KEY_S, KEY_UP, KEY_Z};
use crate::model::character::{Character, SPRITE_DOWN_LEFT, SPRITE_DOWN_RIGHT, SPRITE_DOWN_ROW, SPRITE_DOWN_UP, SPRITE_STAND_ROW};
use crate::model::game::Game;
use crate::model::item::Item;
use crate::model::world::World;
use crate::procedural::world::generate_random_world;
use crate::raylib_wrapper::draw_handle::DrawRectangle;
use crate::raylib_wrapper::wrapper::Window;

pub fn game(window: Rc<RefCell<Window>>, game: Rc<RefCell<Game>>) {
    let random_world = generate_random_world(&window);
    let mut window_borrow = window.borrow_mut();
    let (monitor_width, monitor_height) = &window_borrow.get_size();
    let mut camera: Camera2D = Camera2D {
        offset: Vector2 { x: *monitor_width as f32 / 2.0, y: *monitor_height as f32 / 2.0 },
        target: Vector2 { x: 0.0, y: 0.0 },
        rotation: 0.0,
        zoom: 1.0,
    };

    while !&window_borrow.window_should_close() {
        let (monitor_width, monitor_height) = &window_borrow.get_size();
        camera.offset = Vector2 { x: *monitor_width as f32 / 2.0, y: *monitor_height as f32 / 2.0 };
        reprocess_coordinates(&game, &window_borrow);
        reprocess_zoom(&mut camera, &window_borrow);
        process_actions(&window_borrow, &random_world, &camera);
        draw(&mut window_borrow, &game, &mut camera, &random_world);
    }
}

fn reprocess_zoom(camera: &mut Camera2D, window: &RefMut<Window>) {
    if window.rl.is_key_down(KEY_UP) {
        camera.zoom += 0.1;
    }
    if window.rl.is_key_down(KEY_DOWN) && camera.zoom > 0.5 {
        camera.zoom -= 0.1;
    }
}

fn reprocess_coordinates(game: &Rc<RefCell<Game>>, window: &RefMut<Window>) {
    let mut game_borrow = game.borrow_mut();
    check_movement(game_borrow.team.characters.first_mut().unwrap(), window);
}

fn check_movement(character: &mut Character, window_borrow: &RefMut<Window>) {
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

fn process_actions(window: &RefMut<Window>, random_world: &World, camera: &Camera2D) {
    if window.rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
        let cursor_position = window.rl.get_screen_to_world2D(window.rl.get_mouse_position(), camera);
        println!("{}::{}", cursor_position.x, cursor_position.y);
        random_world.items.iter().for_each(|item| {
            if is_item_collision(cursor_position.x, cursor_position.y, item) {
                println!("Cursor : {}::{} /// HIT item : {:?}", cursor_position.x, cursor_position.y, item);
            }
        })
    }
}

fn is_item_collision(cursor_relative_x: f32, cursor_relative_y: f32, item: &Item) -> bool {
    cursor_relative_x >= item.position.x
        && cursor_relative_x <= item.position.x + item.sprite.texture.width as f32
        && cursor_relative_y >= item.position.y
        && cursor_relative_y <= item.position.y + item.sprite.texture.height as f32
}

fn draw(window: &mut RefMut<Window>, game: &Rc<RefCell<Game>>, camera: &mut Camera2D, random_world: &World) {
    let mut draw_handle = window.begin_drawing();
    let mut mode_2d = draw_handle.raylib_draw_handle.begin_mode2D(*camera);
    let mut game_borrow = game.borrow_mut();
    let character: &mut Character = game_borrow.team.characters.first_mut().unwrap();

    camera.target = Vector2 {
        x: character.position.x + (character.size.x/2.0),
        y: character.position.y + (character.size.y/2.0),
    };

    //Clear
    mode_2d.clear_background(Color::LIMEGREEN);

    //Items
    random_world.items.iter().for_each(|item| {
        mode_2d.draw_texture(&item.sprite.texture, item.position.x as i32, item.position.y as i32, Color::WHITE);
    });

    //Characters
    character.print_sprite(&mut mode_2d,
                           DrawRectangle{
                               x: 0.0,
                               y: character.sprite.row_direction as f32,
                               width: character.size.x,
                               height: character.size.y,
                           },
                           character.position,
                           Color::WHITE);

    mode_2d.draw_rectangle(0, 0, 50, 50, Color::WHITE);
}
