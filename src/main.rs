use crate::obj::Obj;
use crate::vertex::Vertex;
use crate::color::Color;
use crate::model::Model3D;
use crate::triangle::triangle;
use nalgebra_glm::{Vec3, Mat4};
use pixels::{Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;


// Asegúrate de importar todos los módulos necesarios
mod obj;
mod vertex;
mod color;
mod fragment;
mod line;
mod triangle;
mod model;

// Importa Model3D correctamente

#[derive(Debug)]
struct Framebuffer {
    width: usize,
    height: usize,
    buffer: Vec<u32>,
}


// (Continúa con el resto del código)


impl Framebuffer {
    fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            buffer: vec![0; width * height],
        }
    }

    fn set_current_color(&mut self, x: usize, y: usize, color: u32) {
        if x < self.width && y < self.height {
            self.buffer[y * self.width + x] = color;
        }
    }

    fn clear(&mut self, color: u32) {
        for pixel in &mut self.buffer {
            *pixel = color;
        }
    }
}


pub struct Uniforms {
    pub model_matrix: Mat4,
}

impl Uniforms {
    pub fn new(translation: Vec3, scale: f32) -> Self {
        let model_matrix = Mat4::new(
            scale, 0.0, 0.0, translation.x,
            0.0, scale, 0.0, translation.y,
            0.0, 0.0, scale, translation.z,
            0.0, 0.0, 0.0, 1.0,
        );
        Self { model_matrix }
    }
}

fn calculate_bounding_box(v1: &Vec3, v2: &Vec3, v3: &Vec3) -> (i32, i32, i32, i32) {
    let min_x = v1.x.min(v2.x).min(v3.x).floor() as i32;
    let min_y = v1.y.min(v2.y).min(v3.y).floor() as i32;
    let max_x = v1.x.max(v2.x).max(v3.x).ceil() as i32;
    let max_y = v1.y.max(v2.y).max(v3.y).ceil() as i32;

    (min_x, min_y, max_x, max_y)
}

fn vertex_shader(vertex: &Vertex, uniforms: &Uniforms) -> Vertex {
    let position = nalgebra_glm::vec4(vertex.position.x, vertex.position.y, vertex.position.z, 1.0);
    let transformed = uniforms.model_matrix * position;
    let w = transformed.w;
    let transformed_position = Vec3::new(transformed.x / w, transformed.y / w, transformed.z / w);

    Vertex {
        position: vertex.position,
        color: vertex.color,
        transformed_position,
        ..*vertex
    }
}

fn barycentric_coordinates(p: &Vec3, a: &Vec3, b: &Vec3, c: &Vec3) -> (f32, f32, f32) {
    let v0 = b - a;
    let v1 = c - a;
    let v2 = p - a;

    let d00 = v0.dot(&v0);
    let d01 = v0.dot(&v1);
    let d11 = v1.dot(&v1);
    let d20 = v2.dot(&v0);
    let d21 = v2.dot(&v1);

    let denom = d00 * d11 - d01 * d01;
    let v = (d11 * d20 - d01 * d21) / denom;
    let w = (d00 * d21 - d01 * d20) / denom;
    let u = 1.0 - v - w;

    (u, v, w)
}

fn render(
    framebuffer: &mut Framebuffer,
    z_buffer: &mut Vec<f32>,
    uniforms: &Uniforms,
    vertex_array: &[Vertex],
) {
    let transformed_vertices: Vec<Vertex> = vertex_array
        .iter()
        .map(|vertex| vertex_shader(vertex, uniforms))
        .collect();

    for triangle_vertices in transformed_vertices.chunks(3) {
        if triangle_vertices.len() == 3 {
            let fragments = triangle(
                &triangle_vertices[0],
                &triangle_vertices[1],
                &triangle_vertices[2],
            );

            for fragment in fragments {
                let x = fragment.position.x as usize;
                let y = fragment.position.y as usize;

                if x < framebuffer.width && y < framebuffer.height {
                    let index = y * framebuffer.width + x;

                    if fragment.depth < z_buffer[index] {
                        z_buffer[index] = fragment.depth;
                        framebuffer.set_current_color(x, y, fragment.color.to_hex());
                    }
                }
            }
        }
    }
}




fn main() {
    let width = 800;
    let height = 600;

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Rust Graphics - Renderer Example")
        .with_inner_size(LogicalSize::new(width as f64, height as f64))
        .build(&event_loop)
        .unwrap();

    let surface_texture = SurfaceTexture::new(width as u32, height as u32, &window);
    let mut pixels = Pixels::new(width as u32, height as u32, surface_texture).unwrap();

    let mut framebuffer = Framebuffer::new(width, height);
    framebuffer.clear(Color::black().to_hex());
    let mut z_buffer = vec![f32::INFINITY; width * height];

    let obj = Obj::load("assets/nave.obj").expect("Failed to load OBJ file");
    let mut model = Model3D::new();
    model.add_vertices_from_obj(&obj);
    println!("Vertices count: {}", model.vertices.len()); // Agregar esta líne

    let uniforms = Uniforms::new(Vec3::new(350.0, 300.0, 0.0), 15.0); // Ajusta la posición y la escala
    render(&mut framebuffer, &mut z_buffer, &uniforms, &model.vertices);
    //println!("{:?}", framebuffer.buffer); // Verifica que no esté vacío

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;
        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            Event::RedrawRequested(_) => {
                let frame = pixels.get_frame();
                for (i, pixel) in framebuffer.buffer.iter().enumerate() {
                    let offset = i * 4;
                    frame[offset..offset + 4].copy_from_slice(&pixel.to_le_bytes());
                }
                pixels.render().unwrap();
            }
            Event::MainEventsCleared => {
                window.request_redraw();
            }
            _ => {}
        }
    });
}