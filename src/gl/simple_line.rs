
// OBB tutorial https://www.opengl-tutorial.org/miscellaneous/clicking-on-objects/picking-with-custom-ray-obb-function/
// C++ example how ray should work https://github.com/opengl-tutorials/ogl/blob/master/misc05_picking/misc05_picking_custom.cpp
// drawing a line for debug https://stackoverflow.com/questions/60440682/drawing-a-line-in-modern-opengl
// which uses storage shader buffer https://www.khronos.org/opengl/wiki/Shader_Storage_Buffer_Object
// this is the interface for glium https://github.com/glium/glium/pull/954/files
// and possible issue if it is dynamic https://github.com/glium/glium/issues/1918

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

pub struct Line {
  vertex_buffer: VertexBuffer<Vertex>,
  indices: IndexBuffer<u16>,
  normals: VertexBuffer<Normal>,
  // coords: [f32; 4],
  program: glium::Program,
  pub rotation: f32,
  pub scale: [f32; 3],
  pub translation: [f32; 3],
}

impl Line {
  pub fn new<F: Sized + Facade>(display: &F) -> Self {
    
    let coords = vec![
      Vertex { position: [-1.0, -1.0] },
      Vertex { position: [1.0, 1.0] }
    ];
    let vertex_buffer = glium::VertexBuffer::new(display, &coords).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TriangleFan);
    
    let vertex_shader_src = r#"
        #version 140

        in vec2 position;

        void main() {
            gl_Position = vec4(position, 0.0, 1.0);
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
      vertex_buffer: vertex_buffer, 
      indices: IndexBuffer::empty(display,glium::index::PrimitiveType::TrianglesList,  0).unwrap(),
      normals: VertexBuffer::empty(display, 0).unwrap(), 
      program: program,
      rotation: 0.0f32,
      scale: [0.1, 0.1, 1.0],
      translation: [0.0, 0.0, 2.0],
    };
  }

  pub fn update(&mut self, delta: f32) {
    self.rotation += delta;
  }

  pub fn draw(&self, delta: f32, target: &mut Frame) {
    let r = self.rotation;
    let sx: f32 = self.scale[0];
    let sy: f32 = self.scale[1];
    let sz: f32 = self.scale[2];
    let tx: f32 = self.translation[0];
    let ty: f32 = self.translation[1];
    let tz: f32 = self.translation[2];
    let perspective = {
      let (width, height) = target.get_dimensions();
      let aspect_ratio = height as f32 / width as f32;

      let fov: f32 = 3.141592 / 3.0;
      let zfar = 1024.0;
      let znear = 0.1;

      let f = 1.0 / (fov / 2.0).tan();

      [
          [f *   aspect_ratio   ,    0.0,              0.0              ,   0.0],
          [         0.0         ,     f ,              0.0              ,   0.0],
          [         0.0         ,    0.0,  (zfar+znear)/(zfar-znear)    ,   1.0],
          [         0.0         ,    0.0, -(2.0*zfar*znear)/(zfar-znear),   0.0],
      ]
    };
    let (c, s) = (r.cos(), r.sin());
    let uniforms = uniform! {
      perspective: perspective,
      // Rotation around the Z-axis: 
      rotation: [
        [c, -s, 0.0, 0.0],
        [s, c, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0],
      ],
      scale: [
        [sx, 0.0, 0.0, 0.0],
        [0.0, sy, 0.0, 0.0],
        [0.0, 0.0, sz, 0.0],
        [0.0, 0.0, 0.0, 1.0],
      ],
      translation: [
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [tx, ty, tz, 1.0],
      ]
    };

    let indices = glium::index::NoIndices(glium::index::PrimitiveType::LineStrip);
    target.draw(&self.vertex_buffer, 
        &indices, 
        &self.program, 
        &uniforms,
        &Default::default())
      .unwrap();
  }
}

unsafe impl Sync for Line {}