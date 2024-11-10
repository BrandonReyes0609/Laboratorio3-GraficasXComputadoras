use tobj;
use nalgebra_glm::{Vec2, Vec3};
use crate::vertex::Vertex;

pub struct Obj {
    vertices: Vec<Vec3>,     // Posiciones de los vértices
    normals: Vec<Vec3>,      // Normales
    texcoords: Vec<Vec2>,    // Coordenadas de textura (UV)
    indices: Vec<u32>,       // Índices para construir las caras
}

impl Obj {
    // Cargar un archivo .obj y extraer vértices, normales, y texcoords
    pub fn load(filename: &str) -> Result<Self, tobj::LoadError> {
        // Llama a `load_obj` con las opciones predeterminadas directamente
        let (models, _) = tobj::load_obj(filename, &tobj::LoadOptions {
            triangulate: true,
            single_index: true,
            ..Default::default()
        })?;

        let mesh = &models[0].mesh;

        let vertices: Vec<Vec3> = mesh.positions.chunks(3)
            .map(|v| Vec3::new(v[0], v[1], v[2]))
            .collect();

        let normals: Vec<Vec3> = mesh.normals.chunks(3)
            .map(|n| Vec3::new(n[0], n[1], n[2]))
            .collect();

        let texcoords: Vec<Vec2> = mesh.texcoords.chunks(2)
            .map(|t| Vec2::new(t[0], t[1]))
            .collect();

        let indices = mesh.indices.clone();

        Ok(Obj {
            vertices,
            normals,
            texcoords,
            indices,
        })
    }

    // Crear un array de vértices (Vec<Vertex>) usando los datos cargados
    pub fn get_vertex_array(&self) -> Vec<Vertex> {
        let mut vertex_array = Vec::new();

        for &index in &self.indices {
            let position = self.vertices[index as usize];
            let normal = if !self.normals.is_empty() {
                self.normals[index as usize]
            } else {
                Vec3::new(0.0, 0.0, 0.0)  // Normal predeterminada si no hay datos
            };
            let tex_coords = if !self.texcoords.is_empty() {
                self.texcoords[index as usize]
            } else {
                Vec2::new(0.0, 0.0)  // UV predeterminado si no hay datos
            };

            vertex_array.push(Vertex::new(position, normal, tex_coords));
        }

        vertex_array
    }
}
