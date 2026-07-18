use raylib::prelude::Color;

pub struct FrameBuffer {
    pub width: i32,
    pub height: i32,
    pub pixels: Vec<Color>,
}

impl FrameBuffer {
    pub fn new(width: i32, height: i32, background_color: Color) -> Self {
        FrameBuffer {
            width,
            height,
            pixels: vec![background_color; (width * height) as usize],
        }
    }

    pub fn point(&mut self, x: i32, y: i32, color: Color) {
        if x < 0 || y < 0 || x >= self.width || y >= self.height {
            return;
        }
        let idx = (y * self.width + x) as usize;
        self.pixels[idx] = color;
    }

    pub fn get(&self, x: i32, y: i32) -> Color {
        self.pixels[(y * self.width + x) as usize]
    }
}

pub fn get_color(alive: bool) -> Color {
    if alive {
        Color::new(255, 255, 255, 255)
    } else {
        Color::new(15, 15, 25, 255)
    }
}