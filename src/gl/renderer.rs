use std::error::Error;
use std::ffi::{CStr, CString};
use std::num::NonZeroU32;
use std::ops::Deref;

use gl::types::GLfloat;
use raw_window_handle::HasRawWindowHandle;
use imgui_winit_support::winit::event::{Event, KeyEvent, WindowEvent};
use imgui_winit_support::winit::keyboard::{Key, NamedKey};
use imgui_winit_support::winit::window::WindowBuilder;

use glutin::config::{Config, ConfigTemplateBuilder};
use glutin::context::{ContextApi, ContextAttributesBuilder, Version};
use glutin::display::GetGlDisplay;
use glutin::prelude::*;
use glutin::surface::SwapInterval;

use glutin_winit::{self, DisplayBuilder, GlWindow};

pub mod gl {
    #![allow(clippy::all)]
    include!(concat!(env!("OUT_DIR"), "/gl_bindings.rs"));

    // pub use Gles2 as Gl;
}

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

// Find the config with the maximum number of samples, so our triangle will be
// smooth.
// pub fn gl_config_picker(configs: Box<dyn Iterator<Item = Config> + '_>) -> Config {
//     configs
//         .reduce(|accum, config| {
//             let transparency_check = config.supports_transparency().unwrap_or(false)
//                 & !accum.supports_transparency().unwrap_or(false);

//             if transparency_check || config.num_samples() > accum.num_samples() {
//                 config
//             } else {
//                 accum
//             }
//         })
//         .unwrap()
// }

pub struct Renderer {
    program: gl::types::GLuint,
    vao: gl::types::GLuint,
    vbo: gl::types::GLuint,
    gl: gl::Gl,
}

impl Renderer {
    pub fn new<D: GlDisplay>(gl_display: &D) -> Self {
        unsafe {
            let gl = gl::Gl::load_with(|symbol| {
                let symbol = CString::new(symbol).unwrap();
                gl_display.get_proc_address(symbol.as_c_str()).cast()
            });

            if let Some(renderer) = get_gl_string(&gl, gl::RENDERER) {
                println!("Running on {}", renderer.to_string_lossy());
            }
            if let Some(version) = get_gl_string(&gl, gl::VERSION) {
                println!("OpenGL Version {}", version.to_string_lossy());
            }

            if let Some(shaders_version) = get_gl_string(&gl, gl::SHADING_LANGUAGE_VERSION) {
                println!("Shaders version on {}", shaders_version.to_string_lossy());
            }

            let vertex_shader = create_shader(&gl, gl::VERTEX_SHADER, VERTEX_SHADER_SOURCE);
            let fragment_shader = create_shader(&gl, gl::FRAGMENT_SHADER, FRAGMENT_SHADER_SOURCE);

            let program = gl.CreateProgram();

            gl.AttachShader(program, vertex_shader);
            gl.AttachShader(program, fragment_shader);

            gl.LinkProgram(program);

            gl.UseProgram(program);

            gl.DeleteShader(vertex_shader);
            gl.DeleteShader(fragment_shader);

            let mut vao = std::mem::zeroed();
            gl.GenVertexArrays(1, &mut vao);
            gl.BindVertexArray(vao);

            let mut vbo = std::mem::zeroed();
            gl.GenBuffers(1, &mut vbo);
            gl.BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl.BufferData(
                gl::ARRAY_BUFFER,
                (VERTEX_DATA.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
                VERTEX_DATA.as_ptr() as *const _,
                gl::STATIC_DRAW,
            );

            let pos_attrib = gl.GetAttribLocation(program, b"position\0".as_ptr() as *const _);
            let color_attrib = gl.GetAttribLocation(program, b"color\0".as_ptr() as *const _);
            gl.VertexAttribPointer(
                pos_attrib as gl::types::GLuint,
                2,
                gl::FLOAT,
                0,
                5 * std::mem::size_of::<f32>() as gl::types::GLsizei,
                std::ptr::null(),
            );
            gl.VertexAttribPointer(
                color_attrib as gl::types::GLuint,
                3,
                gl::FLOAT,
                0,
                5 * std::mem::size_of::<f32>() as gl::types::GLsizei,
                (2 * std::mem::size_of::<f32>()) as *const () as *const _,
            );
            gl.EnableVertexAttribArray(pos_attrib as gl::types::GLuint);
            gl.EnableVertexAttribArray(color_attrib as gl::types::GLuint);

            Self { program, vao, vbo, gl }
        }
    }

    pub fn draw(&self) {
        self.draw_with_clear_color(0.1, 0.1, 0.1, 0.9)
    }

    pub fn draw_with_clear_color(
        &self,
        red: GLfloat,
        green: GLfloat,
        blue: GLfloat,
        alpha: GLfloat,
    ) {
        unsafe {
            self.gl.UseProgram(self.program);

            self.gl.BindVertexArray(self.vao);
            self.gl.BindBuffer(gl::ARRAY_BUFFER, self.vbo);

            self.gl.ClearColor(red, green, blue, alpha);
            self.gl.Clear(gl::COLOR_BUFFER_BIT);
            self.gl.DrawArrays(gl::TRIANGLES, 0, 3);
        }
    }

    pub fn resize(&self, width: i32, height: i32) {
        unsafe {
            self.gl.Viewport(0, 0, width, height);
        }
    }
}

impl Deref for Renderer {
    type Target = gl::Gl;

    fn deref(&self) -> &Self::Target {
        &self.gl
    }
}

impl Drop for Renderer {
    fn drop(&mut self) {
        unsafe {
            self.gl.DeleteProgram(self.program);
            self.gl.DeleteBuffers(1, &self.vbo);
            self.gl.DeleteVertexArrays(1, &self.vao);
        }
    }
}

unsafe fn create_shader(
    gl: &gl::Gl,
    shader: gl::types::GLenum,
    source: &[u8],
) -> gl::types::GLuint {
    let shader = gl.CreateShader(shader);
    gl.ShaderSource(shader, 1, [source.as_ptr().cast()].as_ptr(), std::ptr::null());
    gl.CompileShader(shader);
    shader
}

fn get_gl_string(gl: &gl::Gl, variant: gl::types::GLenum) -> Option<&'static CStr> {
    unsafe {
        let s = gl.GetString(variant);
        (!s.is_null()).then(|| CStr::from_ptr(s.cast()))
    }
}

#[rustfmt::skip]
static VERTEX_DATA: [f32; 15] = [
    -0.5, -0.5,  1.0,  0.0,  0.0,
     0.0,  0.5,  0.0,  1.0,  0.0,
     0.5, -0.5,  0.0,  0.0,  1.0,
];

const VERTEX_SHADER_SOURCE: &[u8] = b"
#version 100
precision mediump float;

attribute vec2 position;
attribute vec3 color;

varying vec3 v_color;

void main() {
    gl_Position = vec4(position, 0.0, 1.0);
    v_color = color;
}
\0";

const FRAGMENT_SHADER_SOURCE: &[u8] = b"
#version 100
precision mediump float;

varying vec3 v_color;

void main() {
    gl_FragColor = vec4(v_color, 1.0);
}
\0";

// let vertex_shader_src = r#"
//         #version 150

//         in vec3 position;
//         in vec3 normal;

//         out vec3 v_normal;
        
//         uniform mat4 model;
//         uniform mat4 view;
//         uniform mat4 perspective;

//         void main() {
//             mat4 modelview = view * model;
//             v_normal = transpose(inverse(mat3(modelview))) * normal;
//             gl_Position = perspective * modelview * vec4(position, 1.0);
//         }
//     "#;
//     let fragment_shader_src = r#"
//         #version 150

//         in vec3 v_normal;
//         out vec4 color;
//         uniform vec3 u_light;

//         void main() {
//             float brightness = dot(normalize(v_normal), normalize(u_light));
//             vec3 dark_color = vec3(0.6, 0.0, 0.0);
//             vec3 regular_color = vec3(1.0, 0.0, 0.0);
//             color = vec4(mix(dark_color, regular_color, brightness), 1.0);
//         }
//     "#;
