use minifb::{Key, Window, WindowOptions};
use std::time::{Instant, Duration};

// Define your 2D array dimensions
const WIDTH: usize = 800;
const HEIGHT: usize = 600;
// Define your desired frame rate
const DESIRED_FPS: f64 = 250.0;
const FRAME_TIME: f64 = 1.0 / DESIRED_FPS;

//speeds
const PLAYER_SPEED: f64 = 800.0;
const BOT_SPEED: f64 = 800.0;
const BALL_SPEED: f64 = 800.0;

// Define colors
const WHITE_COLOR: u32 = 0xFFFFFF;
const GREEN_COLOR: u32 = 0x00FF00;
const RED_COLOR: u32 = 0xFF0000;

 //dimensions
 const PLAYER_WIDTH: usize = 20; 
 const PLAYER_HEIGHT:usize = 100; 
 const BALL_WIDTH: usize = 20;
 //spacing
 const DISTANCE_TO_WALL: usize = 20;

fn main() {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    let mut last_frame_time = Instant::now();
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
            let delta_time = Instant::now() - last_frame_time;
            let delta_seconds = delta_time.as_secs_f64();
            handle_input(&window, &mut player_y, &mut bot_y,delta_seconds );
            move_ball(&mut ball_x, &mut ball_y, &mut ball_dir_to_left, &mut ball_dir_to_up, player_y, bot_y, delta_seconds);
            check_game_state(&mut player_won, &mut player_lost, &mut ball_dir_to_up, &ball_x, ball_y);

            move_bot(&mut bot_y, &ball_y, delta_seconds);

            clear_buffer(&mut buffer);
        }

        draw_game_objects(&mut buffer, player_y, ball_x, ball_y, bot_y);

        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();

        // If frame was rendered too quickly, wait to maintain frame rate
        let elapsed_frame_time = Instant::now() - last_frame_time;
        if elapsed_frame_time.as_secs_f64() < FRAME_TIME {
            let sleep_duration = Duration::from_secs_f64(FRAME_TIME - elapsed_frame_time.as_secs_f64());
            std::thread::sleep(sleep_duration);
        }

        last_frame_time = Instant::now();
        let elapsed_frame_time = Instant::now() - last_frame_time;
        let elapsed_seconds = elapsed_frame_time.as_secs_f64();

        if elapsed_seconds < FRAME_TIME {
            let sleep_duration = Duration::from_secs_f64(FRAME_TIME - elapsed_seconds);
            std::thread::sleep(sleep_duration);
        }

    }
}

//-------------------------------Movement-----------------------------------------------------------------------------------------

// Function to handle keyboard input
fn handle_input(window: &Window, player_y: &mut usize, _bot_y: &mut usize, delta_seconds: f64) {
    // Calculate speed adjusted by delta_seconds
    let speed_adjusted = (PLAYER_SPEED * delta_seconds) as usize;
    
    // up movement
    if window.is_key_down(Key::W) && *player_y  - speed_adjusted> DISTANCE_TO_WALL {
        *player_y -= speed_adjusted;
    }
    // down movement
    if window.is_key_down(Key::S) && *player_y + speed_adjusted < HEIGHT - PLAYER_HEIGHT - DISTANCE_TO_WALL {
        *player_y += speed_adjusted;
    }
}


// Function to move the ball
fn move_ball(ball_x: &mut usize, ball_y: &mut usize, dir_to_left: &mut bool, dir_to_up: &mut bool, player_y: usize, bot_y: usize, delta_seconds: f64) {
    // Calculate speed adjusted by delta_seconds
    let speed_adjusted = (BALL_SPEED * delta_seconds) as usize;

    // Adjust ball position based on direction and speed
    if *dir_to_left {
        *ball_x -= speed_adjusted;
    } else {
        *ball_x += speed_adjusted;
    }
    if *dir_to_up {
        *ball_y -= speed_adjusted;
    } else {
        *ball_y += speed_adjusted;
    }

    // Update ball direction if it hits player or bot
    let bot_x_distance_wall = PLAYER_WIDTH + DISTANCE_TO_WALL;
    if *ball_x <= bot_x_distance_wall && *ball_y >= player_y && *ball_y <= player_y + PLAYER_HEIGHT {
        *dir_to_left = false;
    }
    if *ball_x >= WIDTH - bot_x_distance_wall -10 && *ball_y >= bot_y && *ball_y <= bot_y + PLAYER_HEIGHT {
        *dir_to_left = true;
    }
}


// Function to move the bot
fn move_bot(bot_y: &mut usize, ball_y: &usize, delta_seconds: f64) {
    let speed_adjusted = (BOT_SPEED * delta_seconds) as usize;

    if *ball_y < *bot_y && HEIGHT >= *bot_y {
        *bot_y -= speed_adjusted;
    } else if *bot_y + 40 < *ball_y &&  *bot_y + PLAYER_HEIGHT + DISTANCE_TO_WALL<= HEIGHT {
        *bot_y += speed_adjusted;
    }
}

//---------------Game State-------------------------------------------------------------------------------------


fn check_game_state(won: &mut bool, lost: &mut bool, dir_to_up: &mut bool, ball_x: &usize, ball_y: usize) {
    // change ball dir when hitting top/bottom frame
    if ball_y + DISTANCE_TO_WALL >= HEIGHT {
        *dir_to_up = true;
    } else if ball_y <= DISTANCE_TO_WALL {
        *dir_to_up = false;
    }

    // check for player/bot missing
    if *ball_x + DISTANCE_TO_WALL >= WIDTH {
        *won = true;
    } else if *ball_x <= DISTANCE_TO_WALL {
        *lost = true;
    }
}


//-------------------Rendering----------------------------------------------------------------------------------


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
    draw_rectangle(buffer, DISTANCE_TO_WALL, player_y, PLAYER_HEIGHT, PLAYER_WIDTH);
    // ball
    draw_rectangle(buffer, ball_x, ball_y, BALL_WIDTH, BALL_WIDTH);
    // bot
    let bot_x: usize = WIDTH -DISTANCE_TO_WALL - PLAYER_WIDTH;
    draw_rectangle(buffer, bot_x, bot_y, PLAYER_HEIGHT, PLAYER_WIDTH);
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

//-------------Rendering End------------------------------------------------------------------------------------