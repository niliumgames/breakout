use glfw::Context;

fn main() {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).expect("initialize glfw");

    let (mut window, events) = glfw
        .create_window(800, 600, "Breakout", glfw::WindowMode::Windowed)
        .expect("window created");

    window.make_current();
    window.set_key_polling(true);

    while !window.should_close() {
        window.swap_buffers();

        glfw.poll_events();
        for (_, ref event) in glfw::flush_messages(&events) {
            dbg!(event);
            match event {
                glfw::WindowEvent::Key(glfw::Key::Escape, _, glfw::Action::Press, _) => {
                    window.set_should_close(true)
                }
                glfw::WindowEvent::MouseButton(button, action, modifiers) => {
                    dbg!(button, action, modifiers);
                }
                _ => {}
            }
        }
    }
}
