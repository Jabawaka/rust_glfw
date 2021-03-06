extern crate glfw;
use self::glfw::*;

use cgmath::{Vector2};

use std::mem;

pub enum InputAction {
    Close,
    ToggleWF,
    MoveForward,
    MoveBack,
    MoveLeft,
    MoveRight,
    RotLeft,
    RotRight,
    RotUp,
    RotDown,
    ViewFrontX,
    ViewRearX,
    ViewLeftY,
    ViewRightY,
    ViewTopZ,
    ViewBotZ,

    EndCommand,
    AbortCommand,

    EnterVertex,
    Num0,
    Num1,
    Num2,
    Num3,
    Num4,
    Num5,
    Num6,
    Num7,
    Num8,
    Num9,
    Comma,
    Dot,
    Minus,

    EnterLine,
    EnterFace,
    DeleteVertex,

    Select
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
    pub last_mouse_pos: Vector2::<f32>
}

impl Window {
    pub fn create(glfw: &mut glfw::Glfw, size: (u32, u32), title: &str) -> Window {
        glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
        glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
        glfw.window_hint(glfw::WindowHint::Samples(Some(4)));

        let (mut glfw_window, glfw_events) = glfw.create_window(size.0, size.1, title, glfw::WindowMode::Windowed).expect("Couldn't create window");

        glfw.with_primary_monitor_mut(|_: &mut _, m: Option<&glfw::Monitor>| {
            let monitor = m.unwrap();
        
            let mode: glfw::VidMode = monitor.get_video_mode().unwrap();
        
            println!("Monitor: {}x{} ({}Hz)", mode.width, mode.height, mode.refresh_rate);
            
            glfw_window.set_monitor(glfw::WindowMode::FullScreen(&monitor), 0, 0, mode.width, mode.height, Some(mode.refresh_rate));
        });

        glfw_window.make_current();

        glfw_window.set_key_polling(true);
        glfw_window.set_framebuffer_size_polling(true);

        glfw_window.set_cursor_pos(size.0 as f64 / 2.0, size.1 as f64 / 2.0);
        glfw_window.set_cursor_mode(CursorMode::Normal);
        glfw_window.set_cursor_pos_polling(true);

        let mut window = Window {
            glfw_window,
            glfw_events,
            last_mouse_pos: Vector2::new(0.0, 0.0),
            commands: Vec::<Command>::new()
        };

        window.commands.push(Command {
            key_id: Key::Escape,
            action: InputAction::Close,
            is_down: false,
            was_just_pressed: false
        });
        window.commands.push(Command {
            key_id: Key::Tab,
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
            key_id: Key::Left,
            action: InputAction::RotLeft,
            is_down: false,
            was_just_pressed: false
        });
        window.commands.push(Command {
            key_id: Key::Right,
            action: InputAction::RotRight,
            is_down: false,
            was_just_pressed: false
        });
        window.commands.push(Command {
            key_id: Key::Up,
            action: InputAction::RotUp,
            is_down: false,
            was_just_pressed: false
        });
        window.commands.push(Command {
            key_id: Key::Down,
            action: InputAction::RotDown,
            is_down: false,
            was_just_pressed: false
        });
        window.commands.push(Command {
            key_id: Key::F1,
            action: InputAction::ViewFrontX,
            is_down: false,
            was_just_pressed: false
        });
        window.commands.push(Command {
            key_id: Key::F4,
            action: InputAction::ViewRearX,
            is_down: false,
            was_just_pressed: false
        });
        window.commands.push(Command {
            key_id: Key::F2,
            action: InputAction::ViewLeftY,
            is_down: false,
            was_just_pressed: false
        });
        window.commands.push(Command {
            key_id: Key::F5,
            action: InputAction::ViewRightY,
            is_down: false,
            was_just_pressed: false
        });
        window.commands.push(Command {
            key_id: Key::F3,
            action: InputAction::ViewTopZ,
            is_down: false,
            was_just_pressed: false
        });
        window.commands.push(Command {
            key_id: Key::F6,
            action: InputAction::ViewBotZ,
            is_down: false,
            was_just_pressed: false
        });

        window.commands.push(Command {
            key_id: Key::Enter,
            action: InputAction::EndCommand,
            is_down: false,
            was_just_pressed: false
        });
        window.commands.push(Command {
            key_id: Key::Q,
            action: InputAction::AbortCommand,
            is_down: false,
            was_just_pressed: false
        });

        window.commands.push(Command {
            key_id: Key::V,
            action: InputAction::EnterVertex,
            is_down: false,
            was_just_pressed: false
        });
        window.commands.push(Command {
            key_id: Key::Num0,
            action: InputAction::Num0,
            is_down: false,
            was_just_pressed: false
        });
        window.commands.push(Command {
            key_id: Key::Num1,
            action: InputAction::Num1,
            is_down: false,
            was_just_pressed: false
        });
        window.commands.push(Command {
            key_id: Key::Num2,
            action: InputAction::Num2,
            is_down: false,
            was_just_pressed: false
        });
        window.commands.push(Command {
            key_id: Key::Num3,
            action: InputAction::Num3,
            is_down: false,
            was_just_pressed: false
        });
        window.commands.push(Command {
            key_id: Key::Num4,
            action: InputAction::Num4,
            is_down: false,
            was_just_pressed: false
        });
        window.commands.push(Command {
            key_id: Key::Num5,
            action: InputAction::Num5,
            is_down: false,
            was_just_pressed: false
        });
        window.commands.push(Command {
            key_id: Key::Num6,
            action: InputAction::Num6,
            is_down: false,
            was_just_pressed: false
        });
        window.commands.push(Command {
            key_id: Key::Num7,
            action: InputAction::Num7,
            is_down: false,
            was_just_pressed: false
        });
        window.commands.push(Command {
            key_id: Key::Num8,
            action: InputAction::Num8,
            is_down: false,
            was_just_pressed: false
        });
        window.commands.push(Command {
            key_id: Key::Num9,
            action: InputAction::Num9,
            is_down: false,
            was_just_pressed: false
        });
        window.commands.push(Command {
            key_id: Key::Period,
            action: InputAction::Dot,
            is_down: false,
            was_just_pressed: false
        });
        window.commands.push(Command {
            key_id: Key::Comma,
            action: InputAction::Comma,
            is_down: false,
            was_just_pressed: false
        });
        window.commands.push(Command {
            key_id: Key::Minus,
            action: InputAction::Minus,
            is_down: false,
            was_just_pressed: false
        });
        
        window.commands.push(Command {
            key_id: Key::L,
            action: InputAction::EnterLine,
            is_down: false,
            was_just_pressed: false
        });
        window.commands.push(Command {
            key_id: Key::F,
            action: InputAction::EnterFace,
            is_down: false,
            was_just_pressed: false
        });
        window.commands.push(Command {
            key_id: Key::X,
            action: InputAction::DeleteVertex,
            is_down: false,
            was_just_pressed: false
        });

        window.commands.push(Command {
            key_id: Key::Space,
            action: InputAction::Select,
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

                    self.last_mouse_pos = Vector2::new(x_pos, y_pos);
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
