use crate::obj::Obj;
use crate::vertex::Vertex;
use crate::color::Color;
use crate::model::Model3D;
use crate::triangle::triangle;
use pixels::{Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::{Event, MouseScrollDelta, WindowEvent, ElementState, MouseButton};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use nalgebra_glm::{Vec3, Mat4, rotate_x, rotate_y};

mod obj;
mod vertex;
mod color;
mod fragment;
mod line;
mod triangle;
mod model;
mod utils;

#[derive(Debug)]
struct Framebuffer {
    width: usize,
    height: usize,
    buffer: Vec<u32>,
}

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
    pub fn new(translation: Vec3, scale: f32, rotation: Mat4) -> Self {
        let model_matrix = Mat4::new_translation(&translation)
            * rotation
            * Mat4::new_nonuniform_scaling(&Vec3::new(scale, scale, scale));
        Self { model_matrix }
    }
}

fn render(
    framebuffer: &mut Framebuffer,
    z_buffer: &mut Vec<f32>,
    uniforms: &Uniforms,
    vertex_array: &[Vertex],
) {
    let transformed_vertices: Vec<Vertex> = vertex_array
        .iter()
        .map(|vertex| {
            let position = nalgebra_glm::vec4(vertex.position.x, vertex.position.y, vertex.position.z, 1.0);
            let transformed = uniforms.model_matrix * position;
            let transformed_position = Vec3::new(transformed.x, transformed.y, transformed.z);
            Vertex {
                position: vertex.position,
                color: vertex.color,
                transformed_position,
                ..*vertex
            }
        })
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

                    // Verificar y actualizar el z-buffer
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
    let mut scale = 1.0;
    let mut camera_angle_x = 0.0;
    let mut camera_angle_y = 0.0;
    let mut is_rotating = false;
    let mut last_mouse_position = (0.0, 0.0);

    let width = 800;
    let height = 600;
    let half_width = width as f32 / 2.0;
    let half_height = height as f32 / 2.0;

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Rust Graphics - Renderer Example")
        .with_inner_size(LogicalSize::new(width as f64, height as f64))
        .build(&event_loop)
        .unwrap();

    let surface_texture = SurfaceTexture::new(width as u32, height as u32, &window);
    let mut pixels = Pixels::new(width as u32, height as u32, surface_texture).unwrap();

    let mut framebuffer = Framebuffer::new(width, height);
    framebuffer.clear(Color { r: 0.0, g: 0.2, b: 0.0 }.to_hex());

    //framebuffer.clear(Color::black().to_hex());
    let mut z_buffer = vec![f32::INFINITY; width * height];

    let obj = Obj::load("assets/naveT.obj").expect("Failed to load OBJ file");
    let mut model = Model3D::new();
    model.add_vertices_from_obj(&obj);

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;
        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                WindowEvent::MouseWheel { delta, .. } => {
                    if let MouseScrollDelta::LineDelta(_, y) = delta {
                        scale = (scale + y * 0.1).clamp(0.1, 100.0);
                    }
                }
                WindowEvent::MouseInput { button: MouseButton::Middle, state, .. } => {
                    is_rotating = state == ElementState::Pressed;
                }
                WindowEvent::CursorMoved { position, .. } => {
                    let (x, y) = (position.x as f32, position.y as f32);
                    if is_rotating {
                        let dx = (x - last_mouse_position.0) * 0.01;
                        let dy = (y - last_mouse_position.1) * 0.01;
                        camera_angle_x += dy;
                        camera_angle_y += dx;
                    }
                    last_mouse_position = (x, y);
                }
                _ => {}
            },
            Event::RedrawRequested(_) => {
                //framebuffer.clear(Color::black().to_hex());
                framebuffer.clear(Color { r: 0.0, g: 0.2, b: 0.0 }.to_hex());

                z_buffer.iter_mut().for_each(|z| *z = f32::INFINITY);

                let rotation_x = rotate_x(&Mat4::identity(), camera_angle_x);
                let rotation_y = rotate_y(&Mat4::identity(), camera_angle_y);
                let camera_transform = rotation_y * rotation_x;

                let uniforms = Uniforms::new(Vec3::new(half_width, half_height, 0.0), scale, camera_transform);

                render(&mut framebuffer, &mut z_buffer, &uniforms, &model.vertices);

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
