mod math;
mod rasterizer;

use math::{Matrix, Triangle, Vector2, Vector3};
use rasterizer::Framebuffer;

fn main() {
    let mut framebuffer = Framebuffer::from_terminal();
    let mut triangles: Vec<Triangle> = Vec::new();
    println!("width: {}, height: {}", framebuffer.width, framebuffer.height);
    let c1 = Vector2 {
        x: 30,
        y: 6,
    };
    let c2 = Vector2 {
        x: 30,
        y: 18,
    };
    let c3 = Vector2 {
        x: 50,
        y: 18,
    };

    let c4 = Vector2 {
        x: 30,
        y: 18,
    };
    let c5 = Vector2 {
        x: 50,
        y: 6,
    };
    let c6 = c3;
    let triangle1 = Triangle::new(c1, c2, c3);
    let triangle2 = Triangle::new(c4, c5, c6);
    triangles.push(triangle1);
    triangles.push(triangle2);

    framebuffer.draw_triangle_list_filled(triangles);
    framebuffer.render();
}
