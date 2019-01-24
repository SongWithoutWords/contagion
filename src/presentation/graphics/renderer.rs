extern crate glfw;

use glfw::{Action, Context, Key};

pub fn create_window() {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    // Create window and OpenGL context
    let (mut window, events) = glfw.create_window(1000, 800, "Contagion", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window!");

    // Set window's context current
    window.make_current();
    window.set_key_polling(true);

    // Loop until user closes window
    while !window.should_close() {
        window.swap_buffers();

        // Poll and process events
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            println!("{:?}", event);
            handle_window_event(&mut window, event);
        }
    }
}

fn handle_window_event(window: &mut glfw::Window, event: glfw::WindowEvent) {
    match event {
        glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
            window.set_should_close(true)
        },
        _ => {},
    }
}