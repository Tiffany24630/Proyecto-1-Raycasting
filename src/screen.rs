use minifb::{Key, Window};
use crate::framebuffer::Framebuffer;
use crate::game::level_name;
use crate::render::draw_centered_panel;
use crate::text::draw_text;

pub fn render_welcome(framebuffer: &mut Framebuffer) {
    draw_rect(framebuffer, 0, 0, framebuffer.width, framebuffer.height, 0x080B14);
    draw_text(framebuffer, 365, 245, "MAZE RUNNER", 0x4AFBFF, 4);
    draw_text(framebuffer, 410, 355, "RAY CASTING", 0xFFFFFF, 3);
    draw_text(framebuffer, 390, 485, "ENTER PARA CONTINUAR", 0xFFFFFF, 2);
    draw_text(framebuffer, 425, 535, "WASD / MOUSE / CONTROL", 0xAAB7D4, 2);
}

pub fn render_level_select(framebuffer: &mut Framebuffer, selected: usize) {
    draw_rect(framebuffer, 0, 0, framebuffer.width, framebuffer.height, 0x080B14);
    draw_text(framebuffer, 420, 100, "SELECCIONAR NIVEL", 0x4AFBFF, 3);

    for i in 0..7 {
        let y = 210 + i * 70;
        let selected_color = if i == selected { 0x4AFBFF } else { 0xFFFFFF };
        draw_text(framebuffer, 430, y, &format!("{} - {}", i + 1, level_name(i)), selected_color, 2);
    }

    draw_text(framebuffer, 385, 730, "1-7 PARA ELEGIR  |  ENTER PARA JUGAR", 0xAAB7D4, 1);
    draw_text(framebuffer, 520, 765, "ESC PARA SALIR", 0xAAB7D4, 1);
}

pub fn render_success(framebuffer: &mut Framebuffer, level: usize) {
    draw_rect(framebuffer, 0, 0, framebuffer.width, framebuffer.height, 0x07120E);
    draw_centered_panel(
        framebuffer,
        "NIVEL COMPLETADO",
        &[&format!("{} SUPERADO", level_name(level)), "ENTER - SIGUIENTE NIVEL", "ESC - SALIR"],
    );
}

pub fn update_level_selection(window: &Window, selected: &mut usize) {
    for (key, index) in [
        (Key::Key1, 0),
        (Key::Key2, 1),
        (Key::Key3, 2),
    ] {
        if window.is_key_pressed(key, minifb::KeyRepeat::No) {
            *selected = index;
        }
    }
}

fn draw_rect(framebuffer: &mut Framebuffer, x0: usize, y0: usize, width: usize, height: usize, color: u32,) {
    framebuffer.set_current_color(color);
    let x1 = x0.saturating_add(width).min(framebuffer.width);
    let y1 = y0.saturating_add(height).min(framebuffer.height);
    for y in y0.min(framebuffer.height)..y1 {
        for x in x0.min(framebuffer.width)..x1 {
            framebuffer.point(x, y);
        }
    }
}
