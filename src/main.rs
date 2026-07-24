mod framebuffer;
mod life;
mod patterns;

use framebuffer::{get_color, FrameBuffer};
use life::{render, GameOfLife};
use raylib::prelude::*;

const GRID_WIDTH: i32 = 100;
const GRID_HEIGHT: i32 = 100;

const SCALE: i32 = 8;

const WINDOW_WIDTH: i32 = GRID_WIDTH * SCALE;
const WINDOW_HEIGHT: i32 = GRID_HEIGHT * SCALE;

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

/// Genera un GIF animado de la simulacion sin abrir ventana.
/// Reutiliza exactamente la misma logica y colores que el render en pantalla.
fn record_gif(path: &str) {
    const GIF_SCALE: i32 = 4;
    const GIF_FRAMES: u32 = 150;

    let mut game = GameOfLife::new(GRID_WIDTH, GRID_HEIGHT, true);
    seed_pattern(&mut game);

    let file = std::fs::File::create(path).expect("no se pudo crear el archivo GIF");
    let mut encoder = image::codecs::gif::GifEncoder::new(std::io::BufWriter::new(file));
    encoder
        .set_repeat(image::codecs::gif::Repeat::Infinite)
        .expect("no se pudo configurar el loop del GIF");

    let w = (GRID_WIDTH * GIF_SCALE) as u32;
    let h = (GRID_HEIGHT * GIF_SCALE) as u32;

    for _ in 0..GIF_FRAMES {
        let mut buf = image::RgbaImage::new(w, h);
        for y in 0..GRID_HEIGHT {
            for x in 0..GRID_WIDTH {
                let c = get_color(game.is_alive(x, y));
                let px = image::Rgba([c.r, c.g, c.b, 255]);
                for dy in 0..GIF_SCALE {
                    for dx in 0..GIF_SCALE {
                        buf.put_pixel((x * GIF_SCALE + dx) as u32, (y * GIF_SCALE + dy) as u32, px);
                    }
                }
            }
        }
        let frame = image::Frame::from_parts(buf, 0, 0, image::Delay::from_numer_denom_ms(90, 1));
        encoder.encode_frame(frame).expect("no se pudo escribir el frame");
        game.step();
    }
}

fn main() {
    if std::env::args().any(|a| a == "--gif") {
        record_gif("game_of_life.gif");
        println!("GIF guardado en game_of_life.gif");
        return;
    }

    let (mut rl, thread) = raylib::init()
        .size(WINDOW_WIDTH, WINDOW_HEIGHT)
        .title("Conway's Game of Life - Javier Alvarado")
        .build();

    rl.set_target_fps(60);

    let mut fb = FrameBuffer::new(GRID_WIDTH, GRID_HEIGHT, get_color(false));
    let mut game = GameOfLife::new(GRID_WIDTH, GRID_HEIGHT, true);

    seed_pattern(&mut game);

    let mut frame_count: u32 = 0;

    while !rl.window_should_close() {
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