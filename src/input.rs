use gilrs::{Axis, Button, EventType, Gilrs};
use minifb::{MouseButton, MouseMode, Window};
use crate::player::Player;

pub const VIEW_BUTTON_X: f32 = 1030.0;
pub const VIEW_BUTTON_Y: f32 = 640.0;
pub const VIEW_BUTTON_W: f32 = 250.0;
pub const VIEW_BUTTON_H: f32 = 36.0;

pub struct ControllerInput {
    gilrs: Option<Gilrs>,
    active_id: Option<gilrs::GamepadId>,
}

impl ControllerInput {
    pub fn new() -> Self {
        match Gilrs::new() {
            Ok(gilrs) => Self { gilrs: Some(gilrs), active_id: None },
            Err(error) => {
                eprintln!("No se pudo inicializar el control: {}", error);
                Self { gilrs: None, active_id: None }
            }
        }
    }

    pub fn update(&mut self) {
        let Some(gilrs) = &mut self.gilrs else { return };

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
    }

    pub fn apply_to_player(&self, player: &mut Player) {
        let (Some(gilrs), Some(id)) = (&self.gilrs, self.active_id) else { return };
        let gamepad = gilrs.gamepad(id);
        let y = -gamepad.value(Axis::LeftStickY);
        let x = gamepad.value(Axis::LeftStickX);
        let rotate = gamepad.value(Axis::RightStickX);

        if y.abs() > 0.15 {
            player.controller_move = y;
        } else {
            player.controller_move = 0.0;
        }
        
        if rotate.abs() > 0.15 {
            player.controller_rotate = rotate;
        } else {
            player.controller_rotate = 0.0;
        }

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