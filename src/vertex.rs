// vertex.rs
use nalgebra_glm::{Vec2, Vec3};
use crate::color::Color;

#[derive(Clone, Debug)]
pub struct Vertex {
    pub position: Vec3,               // Posición del vértice en 3D
    pub normal: Vec3,                 // Vector normal en 3D
    pub tex_coords: Vec2,             // Coordenadas de textura en 2D
    pub color: Color,                 // Color difuso
    pub transformed_position: Vec3,   // Posición transformada
    pub transformed_normal: Vec3,     // Normal transformada
}

impl Vertex {
    // Constructor para crear un vértice con posición, normal y coordenadas de textura
    pub fn new(position: Vec3, normal: Vec3, tex_coords: Vec2) -> Self {
        Vertex {
            position,
            normal,
            tex_coords,
            color: Color::black(),
            transformed_position: position,
            transformed_normal: normal,
        }
    }

    // Constructor para crear un vértice solo con posición y color
    pub fn new_with_color(position: Vec3, color: Color) -> Self {
        Vertex {
            position,
            normal: Vec3::new(0.0, 0.0, 0.0),
            tex_coords: Vec2::new(0.0, 0.0),
            color,
            transformed_position: Vec3::new(0.0, 0.0, 0.0),
            transformed_normal: Vec3::new(0.0, 0.0, 0.0),
        }
    }

    // Método para establecer la posición y la normal transformadas
    pub fn set_transformed(&mut self, position: Vec3, normal: Vec3) {
        self.transformed_position = position;
        self.transformed_normal = normal;
    }
}

// Implementación de Default para Vertex
impl Default for Vertex {
    fn default() -> Self {
        Vertex {
            position: Vec3::new(0.0, 0.0, 0.0),
            normal: Vec3::new(0.0, 1.0, 0.0),
            tex_coords: Vec2::new(0.0, 0.0),
            color: Color::black(),
            transformed_position: Vec3::new(0.0, 0.0, 0.0),
            transformed_normal: Vec3::new(0.0, 1.0, 0.0),
        }
    }
}
