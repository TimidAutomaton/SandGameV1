use winit::{dpi::PhysicalPosition, event::{ElementState, KeyEvent, MouseButton, WindowEvent}, keyboard::{KeyCode, PhysicalKey}};

#[derive(Debug)]
pub struct InputBuffer {
    pub is_space_pressed: bool,
    pub is_left_clicked: bool,
    pub mouse_pos: PhysicalPosition<f64>,
    debouncing: bool,
    pub is_1_pressed: bool,
    pub is_2_pressed: bool,
    pub is_3_pressed: bool,
    pub is_4_pressed: bool,
    pub is_5_pressed: bool,
    pub is_up_pressed: bool,
    pub is_down_pressed: bool,
    pub is_q_pressed: bool,
    pub is_a_pressed: bool,
    pub is_r_pressed: bool,
    pub is_z_pressed: bool,
    pub is_x_pressed: bool,
}


impl InputBuffer {
    pub fn new() -> Self {
        let mouse_pos = PhysicalPosition {
            x: 0.0,
            y: 0.0,
        };

        Self {
            is_space_pressed: false,
            debouncing: false,
            is_left_clicked: false,
            mouse_pos,
            is_1_pressed: false,
            is_2_pressed: false,
            is_3_pressed: false,
            is_4_pressed: false,
            is_5_pressed: false,
            is_up_pressed: false,
            is_down_pressed: false,
            is_q_pressed: false,
            is_a_pressed: false,
            is_r_pressed: false,
            is_z_pressed: false,
            is_x_pressed: false,
        }
    }


    pub fn reset_input(&mut self) {
        self.is_space_pressed = false;
        // self.is_left_clicked = false;
        self.debouncing = false;
        self.is_1_pressed = false;
        self.is_2_pressed = false;
        self.is_3_pressed = false;
        self.is_4_pressed = false;
        self.is_5_pressed = false;
        self.is_up_pressed = false;
        self.is_down_pressed = false;
        self.is_q_pressed = false;
        self.is_a_pressed = false;
        self.is_r_pressed = false;
        self.is_r_pressed = false;
        self.is_z_pressed = false;
        self.is_x_pressed = false;
    }


    pub fn get_input(&mut self, event: &WindowEvent) -> bool {
        //println!("get input: {:?}", event);
        match event {
            WindowEvent::KeyboardInput { event:
                KeyEvent {
                    state: ElementState::Pressed,
                    physical_key: PhysicalKey::Code(KeyCode::Space),
                    ..
                },
            .. 
            } => {
                self.is_space_pressed = true;
                return true
            },
            WindowEvent::KeyboardInput { event:
                KeyEvent {
                    state: ElementState::Pressed,
                    physical_key: PhysicalKey::Code(KeyCode::Digit1),
                    ..
                },
            .. 
            } => {
                self.is_1_pressed = true;
                return true
            },
            WindowEvent::KeyboardInput { event:
                KeyEvent {
                    state: ElementState::Pressed,
                    physical_key: PhysicalKey::Code(KeyCode::Digit2),
                    ..
                },
            .. 
            } => {
                self.is_2_pressed = true;
                return true
            },
            WindowEvent::KeyboardInput { event:
                KeyEvent {
                    state: ElementState::Pressed,
                    physical_key: PhysicalKey::Code(KeyCode::Digit3),
                    ..
                },
            .. 
            } => {
                self.is_3_pressed = true;
                return true
            },
            WindowEvent::KeyboardInput { event:
                KeyEvent {
                    state: ElementState::Pressed,
                    physical_key: PhysicalKey::Code(KeyCode::Digit4),
                    ..
                },
            .. 
            } => {
                self.is_4_pressed = true;
                return true
            },
            WindowEvent::KeyboardInput { event:
                KeyEvent {
                    state: ElementState::Pressed,
                    physical_key: PhysicalKey::Code(KeyCode::Digit5),
                    ..
                },
            .. 
            } => {
                self.is_5_pressed = true;
                return true
            },
            WindowEvent::KeyboardInput { event:
                KeyEvent {
                    state: ElementState::Pressed,
                    physical_key: PhysicalKey::Code(KeyCode::ArrowUp),
                    ..
                },
            .. 
            } => {
                self.is_up_pressed = true;
                return true
            },
            WindowEvent::KeyboardInput { event:
                KeyEvent {
                    state: ElementState::Pressed,
                    physical_key: PhysicalKey::Code(KeyCode::ArrowDown),
                    ..
                },
            .. 
            } => {
                self.is_down_pressed = true;
                return true
            },
            WindowEvent::KeyboardInput { event:
                KeyEvent {
                    state: ElementState::Pressed,
                    physical_key: PhysicalKey::Code(KeyCode::KeyQ),
                    ..
                },
            .. 
            } => {
                self.is_q_pressed = true;
                return true
            },
            WindowEvent::KeyboardInput { event:
                KeyEvent {
                    state: ElementState::Pressed,
                    physical_key: PhysicalKey::Code(KeyCode::KeyA),
                    ..
                },
            .. 
            } => {
                self.is_a_pressed = true;
                return true
            },
            WindowEvent::KeyboardInput { event:
                KeyEvent {
                    state: ElementState::Pressed,
                    physical_key: PhysicalKey::Code(KeyCode::KeyW),
                    ..
                },
            .. 
            } => {
                self.is_up_pressed = true;
                return true
            },
            WindowEvent::KeyboardInput { event:
                KeyEvent {
                    state: ElementState::Pressed,
                    physical_key: PhysicalKey::Code(KeyCode::KeyR),
                    ..
                },
            .. 
            } => {
                self.is_r_pressed = true;
                return true
            },
            WindowEvent::KeyboardInput { event:
                KeyEvent {
                    state: ElementState::Pressed,
                    physical_key: PhysicalKey::Code(KeyCode::KeyZ),
                    ..
                },
            .. 
            } => {
                self.is_z_pressed = true;
                return true
            },
            WindowEvent::KeyboardInput { event:
                KeyEvent {
                    state: ElementState::Pressed,
                    physical_key: PhysicalKey::Code(KeyCode::KeyX),
                    ..
                },
            .. 
            } => {
                self.is_x_pressed = true;
                return true
            },
            WindowEvent::KeyboardInput { event:
                KeyEvent {
                    state: ElementState::Pressed,
                    physical_key: PhysicalKey::Code(KeyCode::KeyS),
                    ..
                },
            .. 
            } => {
                self.is_down_pressed = true;
                return true
            },
            WindowEvent::MouseInput {state: ElementState::Pressed, button: MouseButton::Left, ..} => {
                self.is_left_clicked = true;
                return true
            },
            WindowEvent::MouseInput {state: ElementState::Released, button: MouseButton::Left, ..} => {
                self.is_left_clicked = false;
                return true
            },
            WindowEvent::CursorMoved {position, ..} => {
                self.mouse_pos = *position;
                return false
            },
            _ => {
                return false
            }
        }
    }
}


