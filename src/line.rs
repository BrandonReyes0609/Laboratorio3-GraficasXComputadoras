// line.rs

use crate::fragment::Fragment;
use crate::vertex::Vertex;

pub fn line(a: &Vertex, b: &Vertex) -> Vec<Fragment> {
    let mut fragments = Vec::new();

    let x0 = a.position.x as i32;
    let y0 = a.position.y as i32;
    let x1 = b.position.x as i32;
    let y1 = b.position.y as i32;
    let color = a.color; // Usa el color del primer vértice o interpolación si lo prefieres
    let depth = 0.0; // Profundidad predeterminada, ajustar según necesidades

    let dx = (x1 - x0).abs();
    let dy = -(y1 - y0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = dx + dy;

    let mut x = x0;
    let mut y = y0;

    while x != x1 || y != y1 {
        fragments.push(Fragment::new(x as f32, y as f32, color, depth));

        let e2 = 2 * err;
        if e2 >= dy {
            err += dy;
            x += sx;
        }
        if e2 <= dx {
            err += dx;
            y += sy;
        }
    }

    fragments
}
