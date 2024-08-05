
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
  uniform, uniforms::UniformBuffer
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

#[derive(Copy, Clone)]
struct BufferObject {
  vertex: (f32, f32, f32, f32),
}
implement_vertex!(BufferObject, vertex);

pub struct Line {
  vertex_buffer: VertexBuffer<Vertex>,
  indices: IndexBuffer<u16>,
  normals: VertexBuffer<Normal>,
  ssbo: UniformBuffer<[(f32, f32, f32, f32); 10]>,
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
    
    //let varray = 
    let ssbo = glium::uniforms::UniformBuffer::<[(f32, f32, f32, f32); 10]>::empty_unsized(display, 4 * 4 * 10).unwrap();

    let vertex_shader_src = r#"
      #version 460

      layout(std430, binding = 0) buffer TVertex
      {
        vec4 vertex[10]; 
      };
      
      uniform mat4 perspective;
      uniform mat4 rotation;
      uniform mat4 scale;
      uniform mat4 translation;
      uniform vec2  resolution;
      uniform float thickness;
      
      void main()
      {
        mat4 mvp = perspective * translation * rotation * scale;
        int line_i = gl_VertexID / 6;
        int tri_i  = gl_VertexID % 6;
    
        vec4 va[4];
        for (int i=0; i<4; ++i)
        {
          va[i] = mvp * vertex[line_i+i];
          va[i].xyz /= va[i].w;
          va[i].xy = (va[i].xy + 1.0) * 0.5 * resolution;
        }
    
        vec2 v_line  = normalize(va[2].xy - va[1].xy);
        vec2 nv_line = vec2(-v_line.y, v_line.x);
        
        vec4 pos;
        if (tri_i == 0 || tri_i == 1 || tri_i == 3)
        {
          vec2 v_pred  = normalize(va[1].xy - va[0].xy);
          vec2 v_miter = normalize(nv_line + vec2(-v_pred.y, v_pred.x));
  
          pos = va[1];
          pos.xy += v_miter * thickness * (tri_i == 1 ? -0.5 : 0.5) / dot(v_miter, nv_line);
        }
        else
        {
          vec2 v_succ  = normalize(va[3].xy - va[2].xy);
          vec2 v_miter = normalize(nv_line + vec2(-v_succ.y, v_succ.x));
  
          pos = va[2];
          pos.xy += v_miter * thickness * (tri_i == 5 ? 0.5 : -0.5) / dot(v_miter, nv_line);
        }
    
        pos.xy = pos.xy / resolution * 2.0 - 1.0;
        pos.xyz *= pos.w;
        gl_Position = pos;
      }
    "#;

    // just white
    let fragment_shader_src = r#"
      #version 460

      out vec4 fragColor;
      
      void main()
      {
          fragColor = vec4(1.0);
      }
    "#;

    let program = glium::Program::from_source(display, vertex_shader_src, fragment_shader_src, None).unwrap();

    return Self{ 
      vertex_buffer: vertex_buffer, 
      indices: IndexBuffer::empty(display,glium::index::PrimitiveType::TrianglesList,  0).unwrap(),
      normals: VertexBuffer::empty(display, 0).unwrap(), 
      ssbo: ssbo,
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
      ],
      resolution: [10.0, 10.0f32],
      thickness: 2.0f32,
      TVertex: &self.ssbo
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