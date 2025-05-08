use crate::math::{Vector2, interpolate_x, Triangle};

use terminal_size::{terminal_size, Width, Height};
use std::io::{self, Write};

#[derive(Debug, Clone, Default)]
pub struct Framebuffer {
    pub width: usize,
    pub height: usize,
    pub data: Vec<char>,
}

pub enum CharacterType {
    Flat,
    LightlyShaded
}

impl Into<char> for CharacterType {
    fn into(self) -> char {
        match(self){
            CharacterType::Flat => '#',
            CharacterType::LightlyShaded => '@'
        }
    }
}

impl Framebuffer {
    pub fn get_terminal_size() -> Vector2<usize> {
        if let Some((Width(w), Height(h))) = terminal_size() {
            let width_usize: usize = w as usize;
            let height_usize: usize = h as usize;
            Vector2 {
                x: width_usize,
                y: height_usize,
            }
        } else {
            Vector2::default()
        }
    }
    pub fn from_terminal() -> Self {
        Framebuffer {
            width: Self::get_terminal_size().x as usize,
            height: Self::get_terminal_size().y as usize,
            data: vec![' '; Self::get_terminal_size().x as usize * Self::get_terminal_size().y as usize]
        }
    }

    pub fn get(&self, point: Vector2<usize>) -> Option<char> {
        if point.x < self.width && point.y < self.height {
            Some(self.data[point.y * self.width + point.x])
        } else {
            None
        }
    }

    pub fn set(&mut self, point: Vector2<usize>, value: char) {
        if point.x < self.width && point.y < self.height {
            self.data[point.y as usize * self.width as usize + point.x as usize] = value;
        }
    }

    pub fn render(&self) {
        let stdout = io::stdout();
        let mut handle = stdout.lock();

        for y in 0..self.height {
            for x in 0..self.width {
                let ch = self.get(Vector2::new(x, y)).unwrap_or(' ');
                write!(handle, "{}", ch).unwrap();
            }
            writeln!(handle).unwrap();
        }

        handle.flush().unwrap();
    }

    pub fn draw_line(&mut self, start: Vector2<usize>, end: Vector2<usize>, ch: char){
        let mut x0 = start.x as isize;
        let mut y0 = start.y as isize;
        let x1 = end.x as isize;
        let y1 = end.y as isize;

        let dx = (x1 - x0).abs();
        let dy = -(y1 - y0).abs();
        let sx = if x0 < x1 { 1 } else { -1 };
        let sy = if y0 < y1 { 1 } else { -1 };
        let mut err = dx + dy;

        loop {
            if x0 >= 0 && y0 >= 0 {
                self.set(Vector2::new(x0 as usize, y0 as usize), ch);
            }
            if x0 == x1 && y0 == y1 { break; }
            let e2 = 2 * err;
            if e2 >= dy {
                err += dy;
                x0 += sx;
            }
            if e2 <= dx {
                err += dx;
                y0 += sy;
            }
        }
    }

    pub fn draw_wireframe_triangle(&mut self, triangle: Triangle) {
        self.draw_line(triangle.c1, triangle.c2, CharacterType::Flat.into());
        self.draw_line(triangle.c2, triangle.c3, CharacterType::Flat.into());
        self.draw_line(triangle.c3, triangle.c1, CharacterType::Flat.into());
    }

    pub fn draw_filled_triangle(&mut self, triangle: Triangle) {
        let mut points = [triangle.c1, triangle.c2, triangle.c3];

        points.sort_by_key(|p| p.y);
        let [p0, p1, p2] = points;

        for y in p0.y..=p2.y {
            let x_left = interpolate_x(p0, p1, y);
            let mut x_right = interpolate_x(p0, p2, y);

            if y >= p1.y {
                let x_left = interpolate_x(p1, p2, y);
            }

            let (x_left, x_right) = if x_left > x_right {
                (x_right, x_left)
            } else {
                (x_left, x_right)
            };

            for x in x_left..=x_right {
                self.set(Vector2::new(x, y), CharacterType::Flat.into());
            }
        }
    }

    pub fn draw_triangle_list_filled(&mut self, triangles: Vec<Triangle>) {
        for triangle in triangles.into_iter() {
            self.draw_filled_triangle(triangle);
        }
    }
}
