
pub struct Cube {}

impl Cube {
  fn new() -> Self {
    #[derive(Copy, Clone)]
    struct Vertex {
        position: (f32, f32, f32),
    }
    implement_vertex!(Vertex, position);

    // let shape = vec![
    //     Vertex { position: [-0.5, -0.5] },
    //     Vertex { position: [ 0.0,  0.5] },
    //     Vertex { position: [ 0.5, -0.25] }
    // ];
    // let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
    
    #[derive(Copy, Clone)]
    struct Normal {
        normal: (f32, f32, f32),
    }
    implement_vertex!(Normal, normal);

    let normals = glium::VertexBuffer::new(&display, &vec![
        // front
        Normal { normal: (1.0, 0.0, 0.0) }, 
        // back
        Normal { normal: (0.0, -1.0, 0.0) },
        // up
        Normal { normal: (1.0, 0.0, -1.0) },
        // down
        Normal { normal: (0.0, 0.0, 1.0) },
        // right
        Normal { normal: (1.0, 0.0, 0.0) },
        // left
        Normal { normal: (-1.0, 0.0, 0.0) },
    ]).unwrap();

    const P: f32 = 0.5;
    let vertex_buffer = glium::VertexBuffer::new(&display, &vec![
        // front 0-3
        Vertex { position: (-P, -P, 0.0) }, 
        Vertex { position: (P, P, 0.0) },
        Vertex { position: (P, -P, 0.0) },
        Vertex { position: (-P, P, 0.0) },
        // back 4-7
        Vertex { position: (-P, -P, 1.0) },
        Vertex { position: (P, P, 1.0) },
        Vertex { position: (P, -P, 1.0) },
        Vertex { position: (-P, P, 1.0) },
        // up 8-11
        Vertex { position: (-P, P, 0.0) }, 
        Vertex { position: (P, P, 0.0) },
        Vertex { position: (P, P, 1.0) },
        Vertex { position: (-P, P, 1.0) },
        // down 12-15
        Vertex { position: (-P, -P, 0.0) },
        Vertex { position: (P, -P, 0.0) },
        Vertex { position: (P, -P, 1.0) },
        Vertex { position: (-P, -P, 1.0) },
        // right 16-19
        Vertex { position: (P, -P, 0.0) }, 
        Vertex { position: (P, -P, 1.0) },
        Vertex { position: (P, P, 1.0) },
        Vertex { position: (P, P, 0.0) },
        // left 20-23
        Vertex { position: (-P, -P, 0.0) },
        Vertex { position: (-P, -P, 1.0) },
        Vertex { position: (-P, P, 1.0) },
        Vertex { position: (-P, P, 0.0) },
    ]).unwrap();

    let indices = glium::IndexBuffer::new(&display, glium::index::PrimitiveType::TrianglesList,
        &[
            // front
            0, 2, 1, 
            0, 3, 1,
            // back
            4, 6, 5,
            4, 7, 5,
            // up
            8, 9, 10,
            8, 11, 10,
            // down
            12, 13, 14,
            12, 15, 14,
            // right
            16, 17, 18,
            16, 19, 18,
            // left
            20, 21, 22,
            20, 23, 22u16,
        ]
    ).unwrap();
    // let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

  }
}