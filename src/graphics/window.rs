extern crate glfw;
use self::glfw::{Context, Key, Action, CursorMode};

use std::mem;

pub enum InputAction {
    Close,
    ToggleWF,
    MoveForward,
    MoveBack,
    MoveLeft,
    MoveRight,
    MoveAround,
    Rotate,
    RotLeft,
    RotRight,
    ViewX,
    ViewY,
    ViewZ
}

struct Command {
    key_id: glfw::Key,
    action: InputAction,
    is_down: bool,
    was_just_pressed: bool
}

pub struct Window {
    pub glfw_window: glfw::Window,
    glfw_events: std::sync::mpsc::Receiver<(f64, glfw::WindowEvent)>,
    commands: Vec<Command>,
    last_mouse_pos: (f32, f32),
    pub last_mouse_disp: (f32, f32)
}

impl Window {
    pub fn create(glfw: &mut glfw::Glfw, size: (u32, u32), title: &str) -> Window {
        glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
        glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
        let (mut glfw_new_window, glfw_new_events) =
            glfw.create_window(size.0, size.1, title, glfw::WindowMode::Windowed).
            expect("Failed to create GLFW window");

        glfw_new_window.make_current();

        glfw_new_window.set_key_polling(true);
        glfw_new_window.set_framebuffer_size_polling(true);

        glfw_new_window.set_cursor_pos(size.0 as f64 / 2.0, size.1 as f64 / 2.0);
        glfw_new_window.set_cursor_mode(CursorMode::Hidden);
        glfw_new_window.set_cursor_pos_polling(true);

        let mut window = Window {
            glfw_window: glfw_new_window,
            glfw_events: glfw_new_events,
            last_mouse_pos: (0.0, 0.0),
            last_mouse_disp: (0.0, 0.0),
            commands: Vec::<Command>::new()
        };

        window.commands.push(Command {
            key_id: Key::Escape,
            action: InputAction::Close,
            is_down: false,
            was_just_pressed: false
        });
        window.commands.push(Command {
            key_id: Key::Space,
            action: InputAction::ToggleWF,
            is_down: false,
            was_just_pressed: false
        });
        window.commands.push(Command {
            key_id: Key::W,
            action: InputAction::MoveForward,
            is_down: false,
            was_just_pressed: false
        });
        window.commands.push(Command {
            key_id: Key::S,
            action: InputAction::MoveBack,
            is_down: false,
            was_just_pressed: false
        });
        window.commands.push(Command {
            key_id: Key::A,
            action: InputAction::MoveLeft,
            is_down: false,
            was_just_pressed: false
        });
        window.commands.push(Command {
            key_id: Key::D,
            action: InputAction::MoveRight,
            is_down: false,
            was_just_pressed: false
        });
        window.commands.push(Command {
            key_id: Key::LeftShift,
            action: InputAction::MoveAround,
            is_down: false,
            was_just_pressed: false
        });
        window.commands.push(Command {
            key_id: Key::LeftControl,
            action: InputAction::Rotate,
            is_down: false,
            was_just_pressed: false
        });
        window.commands.push(Command {
            key_id: Key::Q,
            action: InputAction::RotLeft,
            is_down: false,
            was_just_pressed: false
        });
        window.commands.push(Command {
            key_id: Key::E,
            action: InputAction::RotRight,
            is_down: false,
            was_just_pressed: false
        });
        window.commands.push(Command {
            key_id: Key::Num1,
            action: InputAction::ViewX,
            is_down: false,
            was_just_pressed: false
        });
        window.commands.push(Command {
            key_id: Key::Num2,
            action: InputAction::ViewY,
            is_down: false,
            was_just_pressed: false
        });
        window.commands.push(Command {
            key_id: Key::Num3,
            action: InputAction::ViewZ,
            is_down: false,
            was_just_pressed: false
        });

        window
    }

    pub fn process_input(&mut self) {
        // Iterate through commands setting their state
        for command in self.commands.iter_mut() {
            command.was_just_pressed = false;

            // Check the key presses
            if self.glfw_window.get_key(command.key_id) == Action::Press {
                if command.is_down == false {
                    command.was_just_pressed = true;
                }
                command.is_down = true;
            }
            if self.glfw_window.get_key(command.key_id) == Action::Release {
                command.is_down = false;
            }
        }

        // Register mouse displacement for this frame
        for (_, event) in glfw::flush_messages(&self.glfw_events) {
            match event {
                glfw::WindowEvent::CursorPos(x_pos, y_pos) => {
                    let (x_pos, y_pos) = (x_pos as f32, y_pos as f32);
                    self.last_mouse_disp =
                       (self.last_mouse_pos.0 - x_pos,
                        self.last_mouse_pos.1 - y_pos);
                    
                    if self.last_mouse_disp.0.abs() < 1.0 {
                        self.last_mouse_disp.0 = 0.0;
                    }
                    if self.last_mouse_disp.1.abs() < 1.0 {
                        self.last_mouse_disp.1 = 0.0;
                    }

                    self.last_mouse_pos = (x_pos, y_pos);
                }
                _ =>{}
            }
        }
    }

    pub fn was_input_pressed(&self, action: InputAction) -> bool {
        let mut was_pressed = false;

        for command in self.commands.iter() {
            if mem::discriminant(&command.action) == mem::discriminant(&action) {
                was_pressed = command.was_just_pressed;
            }
        }

        was_pressed
    }

    pub fn is_input_down(&self, action: InputAction) -> bool {
        let mut is_down = false;

        for command in self.commands.iter() {
            if mem::discriminant(&command.action) == mem::discriminant(&action) {
                is_down = command.is_down;
            }
        }

        is_down
    }
}