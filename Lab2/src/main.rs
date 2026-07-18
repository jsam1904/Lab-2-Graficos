mod framebuffer;
mod life;
mod patterns;

use framebuffer::{get_color, FrameBuffer};
use life::{render, GameOfLife};
use raylib::prelude::*;

const GRID_WIDTH: i32 = 100;
const GRID_HEIGHT: i32 = 100;

const Scale: i32 = 8;

const WINDOW_WIDTH: i32 = GRID_WIDTH * Scale;
const WINDOW_HEIGHT: i32 = GRID_HEIGHT * Scale;

const FRAMES_PER_STEP: u32 = 6;

fn seed_pattern(life: &mut GameOfLife) {
    patterns::gosper_glider_gun(life, 5, 5);

    patterns::glider(life, 5, 50);
    patterns::glider(life, 5, 80);
    patterns::glider(life, 90, 5);

    patterns::lwss(life, 60, 10);
    patterns::lwss(life, 60, 18);

    patterns::pulsar(life, 55, 55);

    patterns::pentadecathlon(life, 40, 85);

    patterns::beacon(life, 85, 80);
    patterns::toad(life, 85, 60);

    patterns::r_pentomino(life, 30, 60);
    patterns::diehard(life, 55, 35);
    patterns::acorn(life, 15, 45);

    patterns::block(life, 92, 40);
    patterns::block(life, 92, 45);
}

fn main(){
    let (mut rl, thread) = raylib::init()
        .size(WINDOW_WIDTH, WINDOW_HEIGHT)
        .title("Conway's Game of Life - Javier Alvarado")
        .build();
    
    rl.set_target_fps(60);

    let mut fb = FrameBuffer::new(GRID_WIDTH, GRID_HEIGHT, get_color(false));

    let mut game = GameOfLife::new(GRID_WIDTH, GRID_HEIGHT, true);

    seed_pattern(&mut game);

    let mut frame_count: u32 = 0;

    while !rl.window_should_close(){
        if frame_count % FRAMES_PER_STEP == 0 {
            game.step();
        }
        frame_count += 1;

        render(&game, &mut fb);

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);

        for y in 0..GRID_HEIGHT {
            for x in 0..GRID_WIDTH {
                let color = fb.get(x, y);
                d.draw_rectangle(x * SCALE, y * SCALE, SCALE, SCALE, color);
            }
        }
        d.draw_fps(10, 10);
    }
}