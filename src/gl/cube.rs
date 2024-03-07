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

pub struct Cube {
    vertex_buffer: VertexBuffer<Vertex>,
    indices: IndexBuffer<u16>,
    normals: VertexBuffer<Normal>,
    program: glium::Program,
}

impl Cube {
  pub fn new<F: Sized + Facade>(display: &F) -> Self {
    
    let shape = vec![
        Vertex { position: [-0.5, -0.5] },
        Vertex { position: [ 0.0,  0.5] },
        Vertex { position: [ 0.5, -0.25] }
    ];
    let vertex_buffer = glium::VertexBuffer::new(display, &shape).unwrap();

    // let normals = glium::VertexBuffer::new(display, &vec![
    //     // front
    //     Normal { normal: (1.0, 0.0, 0.0) }, 
    //     // back
    //     Normal { normal: (0.0, -1.0, 0.0) },
    //     // up
    //     Normal { normal: (1.0, 0.0, -1.0) },
    //     // down
    //     Normal { normal: (0.0, 0.0, 1.0) },
    //     // right
    //     Normal { normal: (1.0, 0.0, 0.0) },
    //     // left
    //     Normal { normal: (-1.0, 0.0, 0.0) },
    // ]).unwrap();

    // const P: f32 = 0.5;
    // let vertex_buffer = glium::VertexBuffer::new(display, &vec![
    //     // front 0-3
    //     Vertex { position: (-P, -P, 0.0) }, 
    //     Vertex { position: (P, P, 0.0) },
    //     Vertex { position: (P, -P, 0.0) },
    //     Vertex { position: (-P, P, 0.0) },
    //     // back 4-7
    //     Vertex { position: (-P, -P, 1.0) },
    //     Vertex { position: (P, P, 1.0) },
    //     Vertex { position: (P, -P, 1.0) },
    //     Vertex { position: (-P, P, 1.0) },
    //     // up 8-11
    //     Vertex { position: (-P, P, 0.0) }, 
    //     Vertex { position: (P, P, 0.0) },
    //     Vertex { position: (P, P, 1.0) },
    //     Vertex { position: (-P, P, 1.0) },
    //     // down 12-15
    //     Vertex { position: (-P, -P, 0.0) },
    //     Vertex { position: (P, -P, 0.0) },
    //     Vertex { position: (P, -P, 1.0) },
    //     Vertex { position: (-P, -P, 1.0) },
    //     // right 16-19
    //     Vertex { position: (P, -P, 0.0) }, 
    //     Vertex { position: (P, -P, 1.0) },
    //     Vertex { position: (P, P, 1.0) },
    //     Vertex { position: (P, P, 0.0) },
    //     // left 20-23
    //     Vertex { position: (-P, -P, 0.0) },
    //     Vertex { position: (-P, -P, 1.0) },
    //     Vertex { position: (-P, P, 1.0) },
    //     Vertex { position: (-P, P, 0.0) },
    // ]).unwrap();

    // let indices = glium::IndexBuffer::new(display, glium::index::PrimitiveType::TrianglesList,
    //     &[
    //         // front
    //         0, 2, 1, 
    //         0, 3, 1,
    //         // back
    //         4, 6, 5,
    //         4, 7, 5,
    //         // up
    //         8, 9, 10,
    //         8, 11, 10,
    //         // down
    //         12, 13, 14,
    //         12, 15, 14,
    //         // right
    //         16, 17, 18,
    //         16, 19, 18,
    //         // left
    //         20, 21, 22,
    //         20, 23, 22u16,
    //     ]
    // ).unwrap();
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
        program: program };
  }

  pub fn draw(&self, target: &mut Frame) {
    let uniforms = uniform! {
        matrix: [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [  0.0, 0.0, 0.0, 1.0f32],
        ]
    };

    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);
    target.draw(&self.vertex_buffer, 
        &indices, 
        &self.program, 
        &uniforms,
                &Default::default()).unwrap();
  }
}