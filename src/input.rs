use gilrs::{Axis, Button, EventType, Gilrs};
use minifb::{MouseButton, MouseMode, Window};
use crate::player::Player;

pub const VIEW_BUTTON_X: f32 = 1030.0;
pub const VIEW_BUTTON_Y: f32 = 850.0;
pub const VIEW_BUTTON_W: f32 = 250.0;
pub const VIEW_BUTTON_H: f32 = 36.0;

#[derive(Clone, Copy, Default)]
struct ButtonSnapshot {
    south: bool,
    east: bool,
    west: bool,
    north: bool,
    start: bool,
    select: bool,
    dpad_up: bool,
    dpad_down: bool,
}

pub struct ControllerInput {
    gilrs: Option<Gilrs>,
    active_id: Option<gilrs::GamepadId>,
    previous: ButtonSnapshot,
    current: ButtonSnapshot,
}

impl ControllerInput {
    pub fn new() -> Self {
        match Gilrs::new() {
            Ok(gilrs) => Self {
                gilrs: Some(gilrs),
                active_id: None,
                previous: ButtonSnapshot::default(),
                current: ButtonSnapshot::default(),
            },

            Err(error) => {
                eprintln!("No se pudo inicializar el control: {}", error);
                Self {
                    gilrs: None,
                    active_id: None,
                    previous: ButtonSnapshot::default(),
                    current: ButtonSnapshot::default(),
                }
            }
        }
    }

    pub fn update(&mut self) {
        let Some(gilrs) = &mut self.gilrs else {
            self.previous = self.current;
            self.current = ButtonSnapshot::default();
            return;
        };

        while let Some(event) = gilrs.next_event() {
            match event.event {
                EventType::Connected => self.active_id = Some(event.id),
                EventType::Disconnected => {
                    if self.active_id == Some(event.id) {
                        self.active_id = None;
                    }
                }
                _ => {}
            }
        }

        if self.active_id.is_none() {
            self.active_id = gilrs.gamepads().next().map(|(id, _)| id);
        }

        self.previous = self.current;
        self.current = self.sample_buttons();
    }

    fn sample_buttons(&self) -> ButtonSnapshot {
        let (Some(gilrs), Some(id)) = (&self.gilrs, self.active_id) else {
            return ButtonSnapshot::default();
        };

        let gamepad = gilrs.gamepad(id);
        ButtonSnapshot {
            south: gamepad.is_pressed(Button::South),
            east: gamepad.is_pressed(Button::East),
            west: gamepad.is_pressed(Button::West),
            north: gamepad.is_pressed(Button::North),
            start: gamepad.is_pressed(Button::Start),
            select: gamepad.is_pressed(Button::Select),
            dpad_up: gamepad.is_pressed(Button::DPadUp),
            dpad_down: gamepad.is_pressed(Button::DPadDown),
        }
    }

    fn just_pressed(&self, current: bool, previous: bool) -> bool {
        current && !previous
    }

    pub fn confirm_pressed(&self) -> bool {
        self.just_pressed(self.current.south || self.current.start, self.previous.south || self.previous.start)
    }

    pub fn back_pressed(&self) -> bool {
        self.just_pressed(self.current.east || self.current.select, self.previous.east || self.previous.select)
    }

    pub fn toggle_view_pressed(&self) -> bool {
        self.just_pressed(self.current.north, self.previous.north)
    }

    pub fn toggle_textures_pressed(&self) -> bool {
        self.just_pressed(self.current.west, self.previous.west)
    }

    pub fn toggle_music_pressed(&self) -> bool {
        self.just_pressed(self.current.start, self.previous.start)
    }

    pub fn level_up_pressed(&self) -> bool {
        self.just_pressed(self.current.dpad_up, self.previous.dpad_up)
    }

    pub fn level_down_pressed(&self) -> bool {
        self.just_pressed(self.current.dpad_down, self.previous.dpad_down)
    }

    pub fn apply_to_player(&self, player: &mut Player) {
        let (Some(gilrs), Some(id)) = (&self.gilrs, self.active_id) else {
            player.controller_move = 0.0;
            player.controller_rotate = 0.0;
            player.controller_strafe = 0.0;
            player.controller_forward = false;
            player.controller_backward = false;
            return;
        };

        let gamepad = gilrs.gamepad(id);
        let y = gamepad.value(Axis::LeftStickY);
        let x = gamepad.value(Axis::LeftStickX);
        let rotate = gamepad.value(Axis::RightStickX);

        player.controller_move = if y.abs() > 0.15 { -y } else { 0.0 };
        player.controller_strafe = if x.abs() > 0.15 { x } else { 0.0 };
        player.controller_rotate = if rotate.abs() > 0.15 { rotate } else { 0.0 };
        player.controller_forward = gamepad.is_pressed(Button::South);
        player.controller_backward = gamepad.is_pressed(Button::East);
    }
}

pub fn view_button_down(window: &Window) -> bool {
    if let Some((mx, my)) = window.get_mouse_pos(MouseMode::Discard) {
        mx >= VIEW_BUTTON_X
            && mx < VIEW_BUTTON_X + VIEW_BUTTON_W
            && my >= VIEW_BUTTON_Y
            && my < VIEW_BUTTON_Y + VIEW_BUTTON_H
            && window.get_mouse_down(MouseButton::Left)
    } else {
        false
    }
}

pub fn update_mouse_rotation(window: &Window, player: &mut Player, previous_x: &mut Option<f32>, enabled: bool,) {
    if !enabled {
        *previous_x = None;
        return;
    }

    if let Some((mouse_x, _)) = window.get_mouse_pos(MouseMode::Pass) {
        if let Some(previous) = *previous_x {
            player.a += (mouse_x - previous) * 0.004;
        }

        *previous_x = Some(mouse_x);
    }
}