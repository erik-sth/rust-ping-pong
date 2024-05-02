#![windows_subsystem = "windows"]
extern crate minifb;

use minifb::{Key, Window, WindowOptions};

// Define your 2D array dimensions
const WIDTH: usize = 800;
const HEIGHT: usize = 600;

// Define colors
const WHITE_COLOR: u32 = 0xFFFFFF;
const GREEN_COLOR: u32 = 0x00FF00;
const RED_COLOR: u32 = 0xFF0000;

fn main() {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    // Variables for player, ball, and bot
    let mut player_y = 200;
    let mut ball_y = 400;
    let mut ball_x = 200;
    let mut ball_dir_to_left = false;
    let mut ball_dir_to_up = true;
    let mut player_lost = false;
    let mut player_won = false;
    let mut bot_y = 200;

    // Create a window
    let mut window = Window::new("Rendering Rust", WIDTH, HEIGHT, WindowOptions::default())
        .unwrap_or_else(|e| {
            panic!("{}", e);
        });

    // Main loop
    while window.is_open() && !window.is_key_down(Key::Escape) {
        if player_won {
            clear_buffer_with_color(&mut buffer, GREEN_COLOR);
        }
        if player_lost {
            clear_buffer_with_color(&mut buffer, RED_COLOR);
        }

        if !player_won && !player_lost {
            handle_input(&window, &mut player_y, &mut bot_y);
            
            move_ball(&mut ball_x, &mut ball_y, &mut ball_dir_to_left, &mut ball_dir_to_up, player_y, bot_y);
            check_game_state(&mut player_won, &mut player_lost, &mut ball_dir_to_up, &ball_x, ball_y);

            move_bot(&mut bot_y, &ball_y);

            clear_buffer(&mut buffer);
        }

        draw_game_objects(&mut buffer, player_y, ball_x, ball_y, bot_y);

        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}

// Function to handle keyboard input
fn handle_input(window: &Window, player_y: &mut usize, _bot_y: &mut usize) {
    // up movement
    if window.is_key_down(Key::W) && *player_y > 0 {
        *player_y -= 1;
    }
    // down movement
    if window.is_key_down(Key::S) && *player_y < HEIGHT - 100 - 1 {
        *player_y += 1;
    }
}

// Function to move the ball
fn move_ball(ball_x: &mut usize, ball_y: &mut usize, dir_to_left: &mut bool, dir_to_up: &mut bool, player_y: usize, bot_y: usize) {
    if *dir_to_left {
        *ball_x += 1;
    } else {
        *ball_x -= 1;
    }
    if *dir_to_up {
        *ball_y -= 1;
    } else {
        *ball_y += 1;
    }
    // hit box player
    if *ball_x <= 40 && *ball_y + 10 >= player_y && *ball_y <= player_y + 100 {
        *dir_to_left = true;
    }
    // hit box bot
    if *ball_x >= WIDTH - 40 && *ball_y + 10 >= bot_y && *ball_y <= bot_y + 100 {
        *dir_to_left = false;
    }
}



fn check_game_state(won: &mut bool, lost: &mut bool, dir_to_up: &mut bool, ball_x: &usize, ball_y: usize) {
    // change ball dir when hitting top/bottom frame
    if ball_y + 10 >= HEIGHT {
        *dir_to_up = true;
    } else if ball_y <= 10 {
        *dir_to_up = false;
    }

    // check for player/bot missing
    if *ball_x + 10 >= WIDTH {
        *won = true;
    } else if *ball_x <= 10 {
        *lost = true;
    }
}


// Function to move the bot
fn move_bot(bot_y: &mut usize, ball_y: &usize) {
    if *ball_y < *bot_y && HEIGHT >= *bot_y + 10 {
        *bot_y -= 1;
    } else if *bot_y + 50 < *ball_y && 10 + *bot_y <= HEIGHT {
        *bot_y += 1;
    }
}

// Function to clear the buffer
fn clear_buffer(buffer: &mut Vec<u32>) {
    buffer.iter_mut().for_each(|pixel| *pixel = WHITE_COLOR);
}

// Function to clear the buffer with a specific color
fn clear_buffer_with_color(buffer: &mut Vec<u32>, color: u32) {
    buffer.iter_mut().for_each(|pixel| *pixel = color);
}

// Function to draw game objects
fn draw_game_objects(buffer: &mut Vec<u32>, player_y: usize, ball_x: usize, ball_y: usize, bot_y: usize) {
    // player
    draw_rectangle(buffer, 20, player_y, 100, 20);
    // ball
    draw_rectangle(buffer, ball_x, ball_y, 20, 20);
    // bot
    draw_rectangle(buffer, WIDTH - 40, bot_y, 100, 20);
}

// Function to draw a rectangle
fn draw_rectangle(buffer: &mut Vec<u32>, start_pos_x: usize, start_pos_y: usize, height: usize, width: usize) {
    for y in start_pos_y..start_pos_y + height {
        for x in start_pos_x..start_pos_x + width {
            if y < HEIGHT && x < WIDTH {
                let index = y * WIDTH + x;
                buffer[index] = 0;
            }
        }
    }
}
