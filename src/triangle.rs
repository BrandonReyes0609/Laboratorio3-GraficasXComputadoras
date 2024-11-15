use crate::fragment::Fragment;
use crate::vertex::Vertex;
use crate::color::Color;
use nalgebra_glm::Vec3;
use crate::utils::{calculate_bounding_box, barycentric_coordinates};

pub fn triangle(v1: &Vertex, v2: &Vertex, v3: &Vertex) -> Vec<Fragment> {
    let mut fragments = Vec::new();
    let (a, b, c) = (v1.transformed_position, v2.transformed_position, v3.transformed_position);

    let (min_x, min_y, max_x, max_y) = calculate_bounding_box(&a, &b, &c);

    // Dirección de la fuente de luz
    let light_dir = Vec3::new(0.0, 0.0, -1.0);

    // Normal del triángulo para flat shading
    let normal = (b - a).cross(&(c - a)).normalize();

    // Calcula la intensidad de la luz usando el producto punto
    let intensity = normal.dot(&light_dir).max(0.0);

    // Color base con iluminación aplicada
    let lit_color = Color {
        r: 0.8 * intensity,
        g: 0.8 * intensity,
        b: 0.8 * intensity,
    };

    // Itera sobre cada píxel en el bounding box
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let point = Vec3::new(x as f32, y as f32, 0.0);

            // Calcula las coordenadas baricéntricas
            let (w1, w2, w3) = barycentric_coordinates(&point, &a, &b, &c);

            // Verifica si el punto está dentro del triángulo
            if w1 >= 0.0 && w2 >= 0.0 && w3 >= 0.0 {
                // Calcula la profundidad interpolada
                let depth = w1 * a.z + w2 * b.z + w3 * c.z;
                fragments.push(Fragment::new(x as f32, y as f32, lit_color, depth));
            }
        }
    }

    fragments
}
