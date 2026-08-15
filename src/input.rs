use minifb::{MouseButton, MouseMode, Window};
use crate::player::Player;

pub const VIEW_BUTTON_X: f32 = 1030.0;
pub const VIEW_BUTTON_Y: f32 = 640.0;
pub const VIEW_BUTTON_W: f32 = 250.0;
pub const VIEW_BUTTON_H: f32 = 36.0;

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