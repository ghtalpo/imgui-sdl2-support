use imgui::Context;
use imgui_glow_renderer::{
    glow::{self, HasContext},
    AutoRenderer,
};
use imgui_sdl3_support::SdlPlatform;
use sdl3::{
    event::Event,
    video::{GLProfile, Window},
};

// Create a new glow context.
fn glow_context(window: &Window) -> glow::Context {
    unsafe {
        // glow::Context::from_loader_function(|s| window.subsystem().gl_get_proc_address(s) as _)
        glow::Context::from_loader_function(|s| {
            let addr = window.subsystem().gl_get_proc_address(s);
            if addr.is_some() {
                addr.unwrap() as _
            } else {
                std::ptr::null() as _ // ugly but working
            }
        })
    }
}

fn main() {
    /* initialize SDL and its video subsystem */
    let mut sdl = sdl3::init().unwrap();
    let video_subsystem = sdl.video().unwrap();

    /* hint SDL to initialize an OpenGL 3.3 core profile context */
    let gl_attr = video_subsystem.gl_attr();

    gl_attr.set_context_version(3, 3);
    gl_attr.set_context_profile(GLProfile::Core);

    /* create a new window, be sure to call opengl method on the builder when using glow! */
    let window = video_subsystem
        .window("Hello imgui-rs!", 1280, 720)
        // .allow_highdpi()
        .opengl()
        .position_centered()
        .resizable()
        .build()
        .unwrap();

    /* create a new OpenGL context and make it current */
    let gl_context = window.gl_create_context().unwrap();
    window.gl_make_current(&gl_context).unwrap();

    /* enable vsync to cap framerate */
    window.subsystem().gl_set_swap_interval(1).unwrap();

    /* create new glow and imgui contexts */
    let gl = glow_context(&window);

    /* create context */
    let mut imgui = Context::create();

    /* disable creation of files on disc */
    imgui.set_ini_filename(None);
    imgui.set_log_filename(None);

    /* setup platform and renderer, and fonts to imgui */
    imgui
        .fonts()
        .add_font(&[imgui::FontSource::DefaultFontData { config: None }]);

    /* create platform and renderer */
    let mut platform = SdlPlatform::new(&mut imgui);
    let mut renderer = AutoRenderer::new(gl, &mut imgui).unwrap();

    /* start main loop */
    let mut event_pump = sdl.event_pump().unwrap();

    'main: loop {
        for event in event_pump.poll_iter() {
            /* pass all events to imgui platfrom */
            platform.handle_event(&mut imgui, &event);

            if let Event::Quit { .. } = event {
                break 'main;
            }
        }

        /* call prepare_frame before calling imgui.new_frame() */
        platform.prepare_frame(&mut sdl, &mut imgui, &window, &event_pump);

        let ui = imgui.new_frame();
        /* create imgui UI here */
        ui.show_demo_window(&mut true);

        /* render */
        let draw_data = imgui.render();

        unsafe { renderer.gl_context().clear(glow::COLOR_BUFFER_BIT) };
        renderer.render(draw_data).unwrap();

        window.gl_swap_window();
    }
}
