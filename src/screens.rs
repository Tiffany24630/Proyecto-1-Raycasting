use minifb::{Key, KeyRepeat, Window};
use crate::framebuffer::Framebuffer;
use crate::game::level_name;
use crate::input::ControllerInput;
use crate::render::draw_centered_panel;
use crate::text::draw_text;

pub fn render_welcome(framebuffer: &mut Framebuffer) {
    draw_rect(framebuffer, 0, 0, framebuffer.width, framebuffer.height, 0x080B14);
    draw_text(framebuffer, 365, 190, "LABERINTO LA CASA ABANDONADA", 0x4AFBFF, 4);
    draw_text(framebuffer, 410, 300, "ENCUENTRA LA SALIDA", 0xFFFFFF, 3);
    draw_text(framebuffer, 390, 425, "ENTER / A PARA CONTINUAR", 0xFFFFFF, 2);
    draw_text(framebuffer, 365, 485, "TECLADO: W/S MOVER | A/D GIRAR | Q/E O FLECHAS LATERALES", 0xAAB7D4, 2);
    draw_text(framebuffer, 340, 530, "CONTROL: STICK IZQ MOVER/LATERAL | STICK DER GIRAR", 0xAAB7D4, 2);
    draw_text(framebuffer, 360, 600, "V/Y: 2D-3D   T/X: TEXTURAS   M/START: MUSICA", 0xFFFFFF, 1);
    draw_text(framebuffer, 390, 630, "ESC: SALIR", 0xAAB7D4, 1);
}

pub fn render_level_select(framebuffer: &mut Framebuffer, selected: usize) {
    draw_rect(framebuffer, 0, 0, framebuffer.width, framebuffer.height, 0x080B14);
    draw_text(framebuffer, 420, 90, "SELECCIONAR NIVEL", 0x4AFBFF, 3);

    for i in 0..3 {
        let y = 220 + i * 105;

        if i == selected {
            draw_rect(framebuffer, 300, y - 18, 700, 62, 0x14263D);
        }

        let selected_color = if i == selected { 0x4AFBFF } else { 0xFFFFFF };
        
        draw_text(framebuffer, 350, y, &format!("{} - {}", i + 1, level_name(i)), selected_color, 2);
    }

    draw_text(framebuffer, 360, 590, "TECLADO: 1/2/3   ARRIBA/ABAJO: D-PAD", 0xFFFFFF, 1);
    draw_text(framebuffer, 360, 625, "ENTER / A: CONFIRMAR   B: VOLVER   ESC: SALIR", 0xFFFFFF, 1);
    draw_text(framebuffer, 360, 665, "NIVEL 1: FACIL   NIVEL 2: MEDIO   NIVEL 3: DIFICIL", 0xAAB7D4, 1);
}

pub fn render_success(framebuffer: &mut Framebuffer, level: usize) {
    draw_rect(framebuffer, 0, 0, framebuffer.width, framebuffer.height, 0x07120E);
    draw_centered_panel(
        framebuffer,
        "NIVEL COMPLETADO",
        &[
            format!("{} SUPERADO", level_name(level)),
            "ENTER / A - SIGUIENTE NIVEL".to_string(),
        ],
    );
}

pub fn update_level_selection(window: &Window, controller: &ControllerInput, selected: &mut usize,) {
    for (key, index) in [
        (Key::Key1, 0),
        (Key::Key2, 1),
        (Key::Key3, 2),
    ] {
        if window.is_key_pressed(key, KeyRepeat::No) {
            *selected = index;
        }
    }

    if window.is_key_pressed(Key::Up, KeyRepeat::No) || controller.level_up_pressed() {
        *selected = if *selected == 0 { 2 } else { *selected - 1 };
    }

    if window.is_key_pressed(Key::Down, KeyRepeat::No) || controller.level_down_pressed() {
        *selected = (*selected + 1) % 3;
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