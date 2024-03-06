#![allow(unused_imports)]

use std::num::NonZeroU32;

use glium::Display;
use glium::Surface;
// use glium::glutin::surface::{SurfaceAttributesBuilder, WindowSurface};

use glutin::config::{ConfigTemplateBuilder, Config};
use glutin::context::{ContextAttributesBuilder, PossiblyCurrentContext, NotCurrentGlContext};
use glutin::display::{GetGlDisplay, GlDisplay};
use glutin::surface::{SurfaceAttributesBuilder, WindowSurface};
use glutin_winit::DisplayBuilder;

use imgui::{DrawCmd, DrawCmdParams};
// use imgui_glium_renderer::Renderer;
// use imgui_glium_renderer::glium::backend::Context;

use imgui_winit_support::WinitPlatform;
use imgui_winit_support::winit::event::{Event, KeyEvent, WindowEvent};
use imgui_winit_support::winit::keyboard::{Key, NamedKey};
use imgui_winit_support::winit::window::{WindowBuilder, Window};
use imgui_winit_support::winit::event_loop::{EventLoopBuilder, EventLoop};
use raw_window_handle::HasRawWindowHandle;

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
    Display<WindowSurface>
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

    let display = Display::from_context_surface(context, surface)
        .expect("Failed to create glium Display");

    (event_loop, window, display)
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
        display) = create_window();
    let (mut winit_platform, mut imgui_context) = imgui_init(&window);

    let mut renderer = imgui_glium_renderer::Renderer::init(&mut imgui_context, &display)
        .expect("Failed to initialize renderer");

    // Timer for FPS calculation
    let mut last_frame = std::time::Instant::now();

    event_loop.run(move |event, window_target| {
        // match event {
        //     Event::NewEvents(_) => {
        //         let now = std::time::Instant::now();
        //         imgui_context.io_mut().update_delta_time(now - last_frame);
        //         last_frame = now;
        //     }
        //     Event::WindowEvent { event, .. } => match event {
        //         WindowEvent::RedrawRequested => {
        //             let ui = imgui_context.frame();
        //             // Draw our example content
        //             ui.show_demo_window(&mut true);
                    
        //             let mut target = display.draw();
        //             target.clear_color_srgb(1.0, 1.0, 1.0, 1.0);
        //             winit_platform.prepare_render(ui, &window);
        //             let draw_data = imgui_context.render();   
        //             renderer
        //                 .render(&mut target, draw_data)
        //                 .expect("Rendering failed");
        //             target.finish().expect("Failed to swap buffers");
        //         }
        //         WindowEvent::Resized(new_size) => {
        //             if new_size.width > 0 && new_size.height > 0 {
        //                 display.resize((new_size.width, new_size.height));
        //             }
        //             winit_platform.handle_window_event(imgui_context.io_mut(), &window, &event);
        //         },
        //         WindowEvent::CloseRequested
        //         | WindowEvent::KeyboardInput {
        //             event: KeyEvent { logical_key: Key::Named(NamedKey::Escape), .. },
        //             ..
        //         } => window_target.exit(),
        //         _ => (),
        //     }
        //     Event::AboutToWait => {
        //         winit_platform
        //             .prepare_frame(imgui_context.io_mut(), &window)
        //             .expect("Failed to prepare frame");
        //         window.request_redraw();

        //         // gl_surface.swap_buffers(&gl_context).unwrap();
        //     }
        //     event => {
        //         winit_platform.handle_event(imgui_context.io_mut(), &window, &event);
        //     }
        // }
        match event {
            Event::NewEvents(_) => {
                let now = std::time::Instant::now();
                imgui_context.io_mut().update_delta_time(now - last_frame);
                last_frame = now;
            }
            Event::AboutToWait => {
                winit_platform
                    .prepare_frame(imgui_context.io_mut(), &window)
                    .expect("Failed to prepare frame");
                window.request_redraw();
            }
            Event::WindowEvent {
                event: WindowEvent::RedrawRequested,
                ..
            } => {
                let ui = imgui_context.frame();

                ui.show_demo_window(&mut true);

                let mut target = display.draw();
                target.clear_color_srgb(1.0, 1.0, 1.0, 1.0);
                winit_platform.prepare_render(ui, &window);
                let draw_data = imgui_context.render();
                renderer
                    .render(&mut target, draw_data)
                    .expect("Rendering failed");
                target.finish().expect("Failed to swap buffers");
            }
            Event::WindowEvent {
                event: WindowEvent::Resized(new_size),
                ..
            } => {
                if new_size.width > 0 && new_size.height > 0 {
                    display.resize((new_size.width, new_size.height));
                }
                winit_platform.handle_event(imgui_context.io_mut(), &window, &event);
            }
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => window_target.exit(),
            event => {
                winit_platform.handle_event(imgui_context.io_mut(), &window, &event);
            }
        } 
    }).unwrap();

}
