use crate::framebuffer::{get_color, FrameBuffer};

pub struct GameOfLife {
    pub width: i32,
    pub height: i32,
    cells: Vec<bool>,
    wrap: bool,
}

impl GameOfLife {
    pub fn new(width: i32, height: i32, wrap: bool) -> Self {
        GameOfLife {
            width,
            height,
            cells: vec![false; (width * height) as usize],
            wrap,
        }
    }

    fn wrapped(&self, x: i32, y: i32) -> (i32, i32) {
        (
            ((x % self.width) + self.width) % self.width,
            ((y % self.height) + self.height) % self.height,
        )
    }

    pub fn set_alive(&mut self, x: i32, y: i32) {
        let (wx, wy) = self.wrapped(x, y);
        let idx = (wy * self.width + wx) as usize;
        self.cells[idx] = true;
    }

    pub fn is_alive(&self, x: i32, y: i32) -> bool {
        if self.wrap {
            let (wx, wy) = self.wrapped(x, y);
            self.cells[(wy * self.width + wx) as usize]
        } else {
            if x < 0 || y < 0 || x >= self.width || y >= self.height {
                false
            } else {
                self.cells[(y * self.width + x) as usize]
            }
        }
    }

    fn count_neighbors(&self, x: i32, y: i32) -> u8 {
        let mut count = 0;
        for dy in -1..=1 {
            for dx in -1..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }
                if self.is_alive(x + dx, y + dy) {
                    count += 1;
                }
            }
        }
        count
    }

    pub fn step(&mut self) {
        let mut next = self.cells.clone();
        for y in 0..self.height {
            for x in 0..self.width {
                let idx = (y * self.width + x) as usize;
                let alive = self.cells[idx];
                let n = self.count_neighbors(x, y);
                next[idx] = matches!((alive, n), (true, 2) | (true, 3) | (false, 3));
            }
        }
        self.cells = next;
    }
}

pub fn render(life: &GameOfLife, fb: &mut FrameBuffer) {
    for y in 0..life.height {
        for x in 0..life.width {
            fb.point(x, y, get_color(life.is_alive(x, y)));
        }
    }
}