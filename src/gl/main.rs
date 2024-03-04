use raw_window_handle::HasRawWindowHandle;
use imgui_winit_support::WinitPlatform;
use imgui_winit_support::winit::event::{Event, KeyEvent, WindowEvent};
use imgui_winit_support::winit::keyboard::{Key, NamedKey};
use imgui_winit_support::winit::window::{WindowBuilder, Window};
use imgui_winit_support::winit::event_loop::{EventLoopBuilder, EventLoop};
use glutin::surface::{WindowSurface, Surface};
use glutin::config::{ConfigTemplateBuilder, Config};
use glutin::context::{ContextApi, ContextAttributesBuilder, PossiblyCurrentContext};
use glutin::display::GetGlDisplay;
use glutin::prelude::*;
use glutin::surface::SurfaceAttributesBuilder;
use glutin_winit::DisplayBuilder;

use takeable_option::Takeable;

use std::ffi::CString;
use std::rc::Rc;
use std::cell::RefCell;
use std::num::NonZeroU32;
use std::os::raw::c_void;

mod renderer;
use renderer::Renderer;

fn gl_config_picker(mut configs: Box<dyn Iterator<Item = Config> + '_>) -> Config {
    // Just use the first configuration since we don't have any special preferences here
    configs.next().unwrap()
}

fn create_window() -> (
    EventLoop<()>,
    Window,
    Surface<WindowSurface>,
    PossiblyCurrentContext,
    Config
) {
    
    // let event_loop = EventLoopBuilder::new()
    //     .build()
    //     .expect("Failed to create EventLoop");
    let event_loop = EventLoop::new().unwrap();
    
    let window_builder = WindowBuilder::new();
    let config_template_builder = ConfigTemplateBuilder::new();
    let display_builder = DisplayBuilder::new()
        .with_window_builder(Some(window_builder));

    // First we create a window
    let (window, gl_config) = display_builder
        .build(&event_loop, config_template_builder, gl_config_picker)
        .unwrap();
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
    let attrs = SurfaceAttributesBuilder::<WindowSurface>::new().build(
        raw_window_handle,
        NonZeroU32::new(width).unwrap(),
        NonZeroU32::new(height).unwrap(),
    );
    // Now we can create our surface, use it to make our context current and finally create our display

    let surface = unsafe { 
        gl_config.display()
            .create_window_surface(&gl_config, &attrs)
            .expect("Failed to create OpenGL surface")
    };
    let context = context
        .make_current(&surface)
        .expect("Failed to make OpenGL context current");

    (event_loop, window, surface, context, gl_config)
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
    let (event_loop, window, gl_surface, gl_context, gl_config) = create_window();
    let (mut winit_platform, mut imgui_context) = imgui_init(&window);

    //let mut state = None;
    let gl_display = gl_config.display();
    // The context needs to be current for the Renderer to set up shaders and
    // buffers. It also performs function loading, which needs a current context on
    // WGL.
    let mut renderer = Renderer::new(&gl_display);
    event_loop.run(move |event, window_target| {
        match event {
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
                // let renderer = renderer.as_ref().unwrap();
                renderer.draw();
                window.request_redraw();

                gl_surface.swap_buffers(&gl_context).unwrap();
            },
            _ => (),
        }
    }).unwrap();

}