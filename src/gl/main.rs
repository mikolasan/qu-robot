#[macro_use]
extern crate glium;
use glium::Surface;

pub fn view_matrix(position: &[f32; 3], direction: &[f32; 3], up: &[f32; 3]) -> [[f32; 4]; 4] {
    let f = {
        let f = direction;
        let len = f[0] * f[0] + f[1] * f[1] + f[2] * f[2];
        let len = len.sqrt();
        [f[0] / len, f[1] / len, f[2] / len]
    };

    let s = [up[1] * f[2] - up[2] * f[1],
             up[2] * f[0] - up[0] * f[2],
             up[0] * f[1] - up[1] * f[0]];

    let s_norm = {
        let len = s[0] * s[0] + s[1] * s[1] + s[2] * s[2];
        let len = len.sqrt();
        [s[0] / len, s[1] / len, s[2] / len]
    };

    let u = [f[1] * s_norm[2] - f[2] * s_norm[1],
             f[2] * s_norm[0] - f[0] * s_norm[2],
             f[0] * s_norm[1] - f[1] * s_norm[0]];

    let p = [-position[0] * s_norm[0] - position[1] * s_norm[1] - position[2] * s_norm[2],
             -position[0] * u[0] - position[1] * u[1] - position[2] * u[2],
             -position[0] * f[0] - position[1] * f[1] - position[2] * f[2]];

    [
        [s_norm[0], u[0], f[0], 0.0],
        [s_norm[1], u[1], f[1], 0.0],
        [s_norm[2], u[2], f[2], 0.0],
        [p[0], p[1], p[2], 1.0],
    ]
}

fn main() {
    let event_loop = winit::event_loop::EventLoopBuilder::new()
        .build()
        .expect("event loop building");
    let (window, display) = glium::backend::glutin::SimpleWindowBuilder::new()
        .with_title("Glium tutorial #4")
        .build(&event_loop);

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
    
    let vertex_shader_src = r#"
        #version 150

        in vec3 position;
        in vec3 normal;

        out vec3 v_normal;
        
        uniform mat4 model;
        uniform mat4 view;
        uniform mat4 perspective;

        void main() {
            mat4 modelview = view * model;
            v_normal = transpose(inverse(mat3(modelview))) * normal;
            gl_Position = perspective * modelview * vec4(position, 1.0);
        }
    "#;
    let fragment_shader_src = r#"
        #version 150

        in vec3 v_normal;
        out vec4 color;
        uniform vec3 u_light;

        void main() {
            float brightness = dot(normalize(v_normal), normalize(u_light));
            vec3 dark_color = vec3(0.6, 0.0, 0.0);
            vec3 regular_color = vec3(1.0, 0.0, 0.0);
            color = vec4(mix(dark_color, regular_color, brightness), 1.0);
        }
    "#;
    let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();

    let mut t: f32 = 0.0;
    let mut s: f32 = 0.002;

    event_loop.run(move |ev, window_target| {
        match ev {
            winit::event::Event::WindowEvent { event, .. } => match event {
                winit::event::WindowEvent::CloseRequested => {
                    window_target.exit();
                },
                // We now need to render everyting in response to a RedrawRequested event due to the animation
                winit::event::WindowEvent::RedrawRequested => {
                    // we update `t`
                    t += s;
                    if t > 180.0 || t < -180.0 {
                        s = -s;
                    }
                    // let x = t.sin() * 0.5;

                    let mut target = display.draw();
                    // target.clear_color(0.0, 0.0, 1.0, 1.0);
                    target.clear_color_and_depth((0.0, 0.0, 1.0, 1.0), 1.0);

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

                    let view = view_matrix(
                        &[2.0, -1.0, 1.0], 
                        &[-2.0, 1.0, 1.0], 
                        &[0.0, 1.0, 0.0]
                    );

                    let light = [-1.0, -0.4, 0.9f32];
                    let uniforms = uniform! {
                        perspective: perspective,
                        view: view,
                        model: [
                            [t.cos(), -t.sin(), 0.0, 0.0],
                            [t.sin(), t.cos(), 0.0, 0.0],
                            [0.0, 0.0, 1.0, 0.0],
                            [0.0, 0.0, 0.0, 1.0f32],
                        ],
                        u_light: light,
                    };

                    let params = glium::DrawParameters {
                        depth: glium::Depth {
                            test: glium::draw_parameters::DepthTest::IfLess,
                            write: true,
                            .. Default::default()
                        },
                        .. Default::default()
                    };

                    target.draw(
                        (&vertex_buffer, &normals), 
                        &indices, 
                        &program, 
                        &uniforms,
                        &params
                    ).unwrap();
                    target.finish().unwrap();
                },
                // Because glium doesn't know about windows we need to resize the display
                // when the window's size has changed.
                winit::event::WindowEvent::Resized(window_size) => {
                    display.resize(window_size.into());
                },
                _ => (),
            },
            // By requesting a redraw in response to a AboutToWait event we get continuous rendering.
            // For applications that only change due to user input you could remove this handler.
            winit::event::Event::AboutToWait => {
                window.request_redraw();
            },
            _ => (),
        }
    })
    .unwrap();
}