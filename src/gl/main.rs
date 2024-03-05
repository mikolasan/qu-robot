use std::num::NonZeroU32;

use glium::Surface;
use raw_window_handle::HasRawWindowHandle;
use imgui_winit_support::WinitPlatform;
use imgui_winit_support::winit::event::{Event, KeyEvent, WindowEvent};
use imgui_winit_support::winit::keyboard::{Key, NamedKey};
use imgui_winit_support::winit::window::{WindowBuilder, Window};
use imgui_winit_support::winit::event_loop::{EventLoopBuilder, EventLoop};

use glutin::config::{ConfigTemplateBuilder, Config};
use glutin::context::{ContextAttributesBuilder, PossiblyCurrentContext};
use glutin::display::GetGlDisplay;
use glutin::surface::{SurfaceAttributesBuilder, WindowSurface};
use glutin_winit::DisplayBuilder;

use imgui::{DrawCmd, DrawCmdParams};
use imgui_glium_renderer::Renderer;

// mod renderer;
// use renderer::Renderer;
// mod cube;
// use cube::Cube;


fn gl_config_picker(mut configs: Box<dyn Iterator<Item = Config> + '_>) -> Config {
    // Just use the first configuration since we don't have any special preferences here
    configs.next().unwrap()
}

fn create_window() -> (
    EventLoop<()>,
    Window,
    glium::Display<WindowSurface>,
    PossiblyCurrentContext,
    Config
) {
    
    // let event_loop = EventLoopBuilder::new()
    //     .build()
    //     .expect("Failed to create EventLoop");
    let event_loop = EventLoop::new()
        .expect("Failed to create EventLoop");
    
    let window_builder = WindowBuilder::new();
    let config_template_builder = ConfigTemplateBuilder::new();
    let display_builder = DisplayBuilder::new()
        .with_window_builder(Some(window_builder));

    // First we create a window
    let (window, gl_config) = display_builder
        .build(&event_loop, config_template_builder, gl_config_picker)
        .expect("Failed to create OpenGL window");
    let window = window.unwrap();

    // Then the configuration which decides which OpenGL version we'll end up using, here we just use the default which is currently 3.3 core
    // When this fails we'll try and create an ES context, this is mainly used on mobile devices or various ARM SBC's
    // If you depend on features available in modern OpenGL Versions you need to request a specific, modern, version. Otherwise things will very likely fail.
    let raw_window_handle: raw_window_handle::RawWindowHandle = window.raw_window_handle();
    let context_attributes = ContextAttributesBuilder::new()
        .build(Some(raw_window_handle));
    let context = unsafe {
        gl_config.display()
            .create_context(&gl_config, &context_attributes)
            .expect("Failed to create OpenGL context")
    };

    // Determine our framebuffer size based on the window size, or default to 800x600 if it's invisible
    let (width, height): (u32, u32) = window.inner_size().into();
    let surface_attribs = SurfaceAttributesBuilder::<WindowSurface>::new()
        .build(
            raw_window_handle,
            NonZeroU32::new(width).unwrap(),
            NonZeroU32::new(height).unwrap(),
    );
    // Now we can create our surface, use it to make our context current and finally create our display

    let surface = unsafe { 
        gl_config.display()
            .create_window_surface(&gl_config, &surface_attribs)
            .expect("Failed to create OpenGL surface")
    };
    let context = context
        .make_current(&surface)
        .expect("Failed to make OpenGL context current");

    let display = glium::Display::from_context_surface(context, surface)
        .expect("Failed to create glium Display");

    (event_loop, window, display, context, gl_config)
}

fn imgui_init(window: &Window) -> (WinitPlatform, imgui::Context) {
    let mut imgui_context = imgui::Context::create();
    imgui_context.set_ini_filename(None);

    let mut winit_platform = WinitPlatform::init(&mut imgui_context);
    winit_platform.attach_window(
        imgui_context.io_mut(),
        window,
        imgui_winit_support::HiDpiMode::Rounded,
    );

    imgui_context
        .fonts()
        .add_font(&[imgui::FontSource::DefaultFontData { config: None }]);

    imgui_context.io_mut().font_global_scale = (1.0 / winit_platform.hidpi_factor()) as f32;

    (winit_platform, imgui_context)
}

fn main() {
    let (event_loop, 
        window, 
        display, 
        gl_context, 
        gl_config) = create_window();
    let (mut winit_platform, mut imgui_context) = imgui_init(&window);

    //let mut state = None;
    // let gl_display = gl_config.display();
    
    // The context needs to be current for the Renderer to set up shaders and
    // buffers. It also performs function loading, which needs a current context on
    // WGL.
    // let mut renderer = Renderer::new(&gl_display);

    let mut renderer = Renderer::init(&mut imgui_context, &display)
        .expect("Failed to initialize renderer");

    // Timer for FPS calculation
    let mut last_frame = std::time::Instant::now();

    event_loop.run(move |event, window_target| {
        match event {
            Event::NewEvents(_) => {
                let now = std::time::Instant::now();
                imgui_context.io_mut().update_delta_time(now - last_frame);
                last_frame = now;
            }
            Event::WindowEvent { event: WindowEvent::RedrawRequested, .. } => {
                let ui = imgui_context.frame();

                // Draw our example content
                ui.show_demo_window(&mut true);
                let draw_data = imgui_context.render();   
                
                let fb_width = draw_data.display_size[0] * draw_data.framebuffer_scale[0];
                let fb_height = draw_data.display_size[1] * draw_data.framebuffer_scale[1];

                let left = draw_data.display_pos[0];
                let right = draw_data.display_pos[0] + draw_data.display_size[0];
                let top = draw_data.display_pos[1];
                let bottom = draw_data.display_pos[1] + draw_data.display_size[1];
                let matrix = [
                    [(2.0 / (right - left)), 0.0, 0.0, 0.0],
                    [0.0, (2.0 / (top - bottom)), 0.0, 0.0],
                    [0.0, 0.0, -1.0, 0.0],
                    [
                        (right + left) / (left - right),
                        (top + bottom) / (bottom - top),
                        0.0,
                        1.0,
                    ],
                ];
                let clip_off = draw_data.display_pos;
                let clip_scale = draw_data.framebuffer_scale;
                // for draw_list in draw_data.draw_lists() {
                    
                //     let vtx_buffer = VertexBuffer::immutable(&self.ctx, unsafe {
                //         draw_list.transmute_vtx_buffer::<GliumDrawVert>()
                //     })?;
                //     let idx_buffer = IndexBuffer::immutable(
                //         &self.ctx,
                //         PrimitiveType::TrianglesList,
                //         draw_list.idx_buffer(),
                //     )?;

                //     for cmd in draw_list.commands() {
                //         match cmd {
                //             DrawCmd::Elements {
                //                 count,
                //                 cmd_params:
                //                     DrawCmdParams {
                //                         clip_rect,
                //                         texture_id,
                //                         vtx_offset,
                //                         idx_offset,
                //                         ..
                //                     },
                //             } => {
                //                 let clip_rect = [
                //                     (clip_rect[0] - clip_off[0]) * clip_scale[0],
                //                     (clip_rect[1] - clip_off[1]) * clip_scale[1],
                //                     (clip_rect[2] - clip_off[0]) * clip_scale[0],
                //                     (clip_rect[3] - clip_off[1]) * clip_scale[1],
                //                 ];

                //                 if clip_rect[0] < fb_width
                //                     && clip_rect[1] < fb_height
                //                     && clip_rect[2] >= 0.0
                //                     && clip_rect[3] >= 0.0
                //                 {
                //                     let texture = self.lookup_texture(texture_id)?;

                //                     target.draw(
                //                         vtx_buffer
                //                             .slice(vtx_offset..)
                //                             .expect("Invalid vertex buffer range"),
                //                         idx_buffer
                //                             .slice(idx_offset..(idx_offset + count))
                //                             .expect("Invalid index buffer range"),
                //                         &self.program,
                //                         &uniform! {
                //                             matrix: matrix,
                //                             tex: Sampler(texture.texture.as_ref(), texture.sampler)
                //                         },
                //                         &DrawParameters {
                //                             blend: Blend::alpha_blending(),
                //                             scissor: Some(Rect {
                //                                 left: f32::max(0.0, clip_rect[0]).floor() as u32,
                //                                 bottom: f32::max(0.0, fb_height - clip_rect[3]).floor()
                //                                     as u32,
                //                                 width: (clip_rect[2] - clip_rect[0]).abs().ceil() as u32,
                //                                 height: (clip_rect[3] - clip_rect[1]).abs().ceil() as u32,
                //                             }),
                //                             ..DrawParameters::default()
                //                         },
                //                     )?;
                //                 }
                //             }
                //             DrawCmd::ResetRenderState => (), // TODO
                //             DrawCmd::RawCallback { callback, raw_cmd } => unsafe {
                //                 callback(draw_list.raw(), raw_cmd)
                //             },
                //         }
                //     }
                // }
            }
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::Resized(size) => {
                    if size.width != 0 && size.height != 0 {
                        // Some platforms like EGL require resizing GL surface to update the size
                        // Notable platforms here are Wayland and macOS, other don't require it
                        // and the function is no-op, but it's wise to resize it for portability
                        // reasons.

                        // gl_surface.resize(
                        //     gl_context,
                        //     NonZeroU32::new(size.width).unwrap(),
                        //     NonZeroU32::new(size.height).unwrap(),
                        // );
                        // let renderer = renderer.as_ref().unwrap();
                        // renderer.resize(size.width as i32, size.height as i32);
                    }
                },
                WindowEvent::CloseRequested
                | WindowEvent::KeyboardInput {
                    event: KeyEvent { logical_key: Key::Named(NamedKey::Escape), .. },
                    ..
                } => window_target.exit(),
                _ => (),
            },
            Event::AboutToWait => {
                
                
                winit_platform
                    .prepare_frame(imgui_context.io_mut(), &window)
                    .expect("Failed to prepare frame");
                window.request_redraw();

                // gl_surface.swap_buffers(&gl_context).unwrap();
            },
            _ => (),
        }
    }).unwrap();

}