use std::sync::Arc;

use glium::{
  Surface, 
  backend::Facade, 
  IndexBuffer, VertexBuffer, 
  Frame,
  implement_vertex,
  uniform
};

#[derive(Copy, Clone)]
struct Vertex {
  position: [f32; 2],
}
implement_vertex!(Vertex, position);

#[derive(Copy, Clone)]
struct Normal {
  normal: (f32, f32, f32),
}
implement_vertex!(Normal, normal);

pub struct NeuronShape {
  vertex_buffer: VertexBuffer<Vertex>,
  indices: IndexBuffer<u16>,
  normals: VertexBuffer<Normal>,
  program: glium::Program,
  pub rotation: f32,
}

impl NeuronShape {
  pub fn new<F: Sized + Facade>(neuron_id: &String, display: &F) -> Self {
    
    let shape = vec![
      Vertex { position: [ 0.0,  0.5] },
      Vertex { position: [-0.35, -0.5] },
      Vertex { position: [ 0.35, -0.5] },
    ];
    let vertex_buffer = glium::VertexBuffer::new(display, &shape).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    let vertex_shader_src = r#"
      #version 140

      in vec2 position;
      
      uniform mat4 matrix;

      void main() {
        gl_Position = matrix * vec4(position, 0.0, 1.0);
      }
    "#;

    let fragment_shader_src = r#"
      #version 140

      out vec4 color;

      void main() {
        color = vec4(1.0, 0.0, 0.0, 1.0);
      }
    "#;

    let program = glium::Program::from_source(display, vertex_shader_src, fragment_shader_src, None).unwrap();

    return Self{ 
      vertex_buffer:vertex_buffer, 
      indices: IndexBuffer::empty(display,glium::index::PrimitiveType::TrianglesList,  0).unwrap(),
      normals: VertexBuffer::empty(display, 0).unwrap(), 
      program: program,
      rotation: 0.0f32,
    };
  }

  pub fn update(&mut self, delta: f32) {
    self.rotation += delta;
  }

  pub fn draw(&self, delta: f32, target: &mut Frame) {
    let r = self.rotation;
    let uniforms = uniform! {
      matrix: [
        [r.sin(), r.cos(), 0.0, 0.0],
        [-r.cos(), r.sin(), 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0f32],
      ]
    };

    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);
    target.draw(&self.vertex_buffer, 
        &indices, 
        &self.program, 
        &uniforms,
        &Default::default())
      .unwrap();
  }
}

unsafe impl Sync for NeuronShape {}