use crate::vertex::Vertex;

#[derive(Debug)]
pub struct Edge {
    pub start: Vertex,
    pub end: Vertex,
}

#[derive(Debug)]
pub struct Face {
    pub vertices: [Vertex; 3],
}

#[derive(Debug)]
pub struct Model3D {
    pub vertices: Vec<Vertex>,
    pub edges: Vec<Edge>,
    pub faces: Vec<Face>,
}

impl Model3D {
    pub fn new() -> Self {
        Self {
            vertices: Vec::new(),
            edges: Vec::new(),
            faces: Vec::new(),
        }
    }

    pub fn add_vertices_from_obj(&mut self, obj: &crate::obj::Obj) {
        for vertex in obj.get_vertex_array() {
            self.vertices.push(vertex);
        }
    }
}
