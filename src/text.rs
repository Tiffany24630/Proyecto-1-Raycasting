use font8x8::UnicodeFonts;
use crate::framebuffer::Framebuffer;

pub fn draw_text(framebuffer: &mut Framebuffer, x: usize, y: usize, text: &str, color: u32, scale: usize,) {
    if scale == 0 {
        return;
    }

    let mut cursor_x = x;

    for ch in text.chars() {
        let ch = ch.to_ascii_uppercase();

        if let Some(glyph) = font8x8::BASIC_FONTS.get(ch) {
            for (row, bits) in glyph.iter().enumerate() {
                for col in 0..8 {
                    if bits & (1 << col) != 0 {
                        draw_scaled_pixel(
                            framebuffer,
                            cursor_x + col * scale,
                            y + row * scale,
                            scale,
                            color,
                        );
                    }
                }
            }
        }
        cursor_x += 9 * scale;
    }
}

fn draw_scaled_pixel(framebuffer: &mut Framebuffer, x: usize, y: usize, scale: usize, color: u32,) {
    framebuffer.set_current_color(color);
    
    for py in y..y.saturating_add(scale) {
        for px in x..x.saturating_add(scale) {
            framebuffer.point(px, py);
        }
    }
}
