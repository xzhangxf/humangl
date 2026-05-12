use glfw::{Action, Context, Key,OpenGlProfileHint,WindowHint};
mod math;
mod shader;
mod cube;
mod stack;
mod human;
use math::{Mat4, Vec3};

//source strings to compile vertex and fragment shader programmes
const VS_SRC: &str = r#"
#version 410 core

layout (location = 0) in vec3 aPos;
uniform mat4 u_mvp;

void main() {
    gl_Position = u_mvp * vec4(aPos, 1.0);
}
"#;

const FS_SRC: &str = r#"
#version 410 core

out vec4 FragColor;
uniform vec3 u_colour;

void main() {
    FragColor = vec4(u_colour,1.0);
}
"#;


fn main() {
    //initialise GLFW
    let mut glfw = glfw::init(glfw::fail_on_errors).expect("Failed to initialise GLFW");

    //set window hints
    glfw.window_hint(WindowHint::ContextVersionMajor(4));
    glfw.window_hint(WindowHint::ContextVersionMinor(1));
    glfw.window_hint(WindowHint::OpenGlProfile(OpenGlProfileHint::Core));
    glfw.window_hint(WindowHint::OpenGlForwardCompat(true));
//create window
    let (mut window, events) = glfw.create_window(1024, 768, "HumanGL", glfw::WindowMode::Windowed).expect("Failed to create GLFW window");

    window.make_current();
    window.set_key_polling(true);
    //load OpenGL functions
    gl::load_with(|s| {
        window
            .get_proc_address(s)
            .map(|p| p as *const _)
            .unwrap_or(std::ptr::null())
    });
//compile and link shader programmes
    let _program = shader::compile_program(VS_SRC, FS_SRC);
    println!("Shader programme compiled + linked");
    //create cube
    let cube = cube::UnitCube::new(_program);
    //set up OpenGL state
    unsafe {
        let version = gl::GetString(gl::VERSION);
        let renderer = gl::GetString(gl::RENDERER);
        if !version.is_null() {
            println!("OpenGL VERSION: {}", cstr_to_string(version));
        }
        if !renderer.is_null() {
            println!("OpenGL RENDERER: {}", cstr_to_string(renderer));
        }
        gl::Viewport(0,0, 1024, 768);
        gl::Enable(gl::DEPTH_TEST);
    }
  
  let mut human = human::HumanAnimation::new();
  let mut debug_spin = false;
  let mut t_prev = glfw.get_time() as f32;
    //render loop
    while !window.should_close() {
        //poll for events
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            match event {
                glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => window.set_should_close(true),
                glfw::WindowEvent::Key(Key::Num1, _, Action::Press, _) => human.motion = human::Motion::Idle,
                glfw::WindowEvent::Key(Key::Num2, _, Action::Press, _) => human.motion = human::Motion::Walk,
                glfw::WindowEvent::Key(Key::Num3, _, Action::Press, _) => {
                    human.start_jump();
                }
                glfw::WindowEvent::Key(Key::R, _, Action::Press, _) => debug_spin = !debug_spin,
                _ => {}
            }
        }
        //time
        let t_now = glfw.get_time() as f32;
        let dt = (t_now - t_prev).max(0.0);
        t_prev = t_now;
        //clear colour and depth
        unsafe {
            gl::ClearColor(0.08, 0.10, 0.14, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        //camera setup
        let (fb_w, fb_h) = window.get_framebuffer_size();
        //match framebuffer size to viewport
        unsafe { gl::Viewport(0, 0, fb_w, fb_h); }
        let aspect = (fb_w as f32) / (fb_h as f32).max(1.0);
//set projection matrix (camera lens)
        let proj = Mat4::perspective(45.0f32.to_radians(), aspect, 0.1, 100.0);

        let eye = Vec3::new(0.0, 1.0, 8.0);
        let centre = Vec3::new(0.0, 0.50, 0.0);
        let up = Vec3::new(0.0, 1.0, 0.0);
        let view = Mat4::look_at(eye, centre, up);

        let pose = human.update(dt);
        human::draw_human(&cube, proj, view, debug_spin, t_now, pose);
    
        window.swap_buffers();
    }
}

fn cstr_to_string(ptr: *const u8) -> String {
    unsafe { std::ffi::CStr::from_ptr(ptr as *const i8) }
        .to_string_lossy()
        .into_owned()
}
