use std::f32::consts::PI;
use crate::caster::cast_ray;
use crate::framebuffer::Framebuffer;
use crate::maze::Maze;
use crate::player::Player;
use crate::raycaster::cast_ray as cast_ray_3d;
use crate::text::draw_text;
use crate::textures::TextureSet;

pub const BLOCK_SIZE: usize = 20;
pub const WINDOW_WIDTH: usize = 1300;
pub const WINDOW_HEIGHT: usize = 900;
pub const NUM_RAYS: usize = WINDOW_WIDTH;
pub const NUM_2D_RAYS: usize = 5;
pub const FOV: f32 = PI / 3.0;

pub fn render_2d(framebuffer: &mut Framebuffer, maze: &Maze, player: &Player) {
    for (row, line) in maze.iter().enumerate() {
        for (col, &cell) in line.iter().enumerate() {
            draw_cell(framebuffer, col * BLOCK_SIZE, row * BLOCK_SIZE, cell);
        }
    }

    for i in 0..NUM_2D_RAYS {
        let fraction = if NUM_2D_RAYS > 1 {
            i as f32 / (NUM_2D_RAYS - 1) as f32
        } else {
            0.5
        };

        let angle = player.a - FOV / 2.0 + FOV * fraction;

        cast_ray(framebuffer, maze, player, angle, BLOCK_SIZE, 0);
    }

    draw_player_2d(framebuffer, player, 0);
    draw_rect(framebuffer, 0, 420, WINDOW_WIDTH, WINDOW_HEIGHT - 420, 0x0D1220);
    draw_text(framebuffer, 18, 438, "VISTA 2D", 0x4AFBFF, 2);
    draw_text(framebuffer, 18, 474, "W/S AVANZAR-RETROCEDER   A/D GIRAR", 0xFFFFFF, 1);
    draw_text(framebuffer, 18, 498, "V CAMBIAR 2D/3D   T TEXTURAS   M MUSICA", 0xFFFFFF, 1);
    draw_text(framebuffer, 18, 522, "MOUSE: ROTACION HORIZONTAL", 0xAAB7D4, 1);
    draw_text(framebuffer, 18, 546, "CONTROL: STICK IZQ. MOVIMIENTO / STICK DER. ROTACION", 0xAAB7D4, 1);
    draw_text(framebuffer, 18, 570, "A: CONFIRMAR   X: TEXTURAS   Y: 2D/3D   START: MUSICA", 0xAAB7D4, 1);
    draw_view_button(framebuffer);
}

pub fn render_3d(framebuffer: &mut Framebuffer, maze: &Maze, player: &Player, textures: &TextureSet, textures_enabled: bool, music_enabled: bool,) {
    draw_rect(framebuffer, 0, 0, WINDOW_WIDTH, WINDOW_HEIGHT / 2, 0x18233D);
    draw_rect(framebuffer, 0, WINDOW_HEIGHT / 2, WINDOW_WIDTH, WINDOW_HEIGHT / 2, 0x302A2A,);

    let projection_distance = (WINDOW_WIDTH as f32 / 2.0) / (FOV / 2.0).tan();

    for column in 0..NUM_RAYS {
        let camera = (2.0 * column as f32 / NUM_RAYS as f32) - 1.0;
        let ray_angle = player.a + (camera * (FOV / 2.0).tan()).atan();

        let Some(hit) = cast_ray_3d(maze, player, ray_angle, BLOCK_SIZE) else {
            continue;
        };

        let perpendicular_distance = (hit.distance * (ray_angle - player.a).cos()).max(0.1);
        let wall_height = (BLOCK_SIZE as f32 * projection_distance / perpendicular_distance).min(WINDOW_HEIGHT as f32 * 2.0);
        let top = ((WINDOW_HEIGHT as f32 - wall_height) / 2.0).max(0.0) as usize;
        let bottom = ((WINDOW_HEIGHT as f32 + wall_height) / 2.0).min(WINDOW_HEIGHT as f32) as usize;
        let bottom = bottom.max(top + 1).min(WINDOW_HEIGHT);

        if textures_enabled {
            if let Some(texture) = textures.get(hit.cell) {
                let tex_x = ((hit.texture_u.clamp(0.0, 0.9999)) * texture.width as f32) as usize;
                
                for y in top..bottom {
                    let relative = (y - top) as f32 / (bottom - top).max(1) as f32;
                    let tex_y = (relative * texture.height as f32) as usize;
                    let idx = tex_y.min(texture.height - 1) * texture.width + tex_x.min(texture.width - 1);
                    
                    let mut color = texture.pixels[idx];
                    
                    if hit.side {
                        color = darken(color, 0.65);
                    }

                    framebuffer.set_current_color(color);
                    framebuffer.point(column, y);
                }
                continue;
            }
        }
        framebuffer.set_current_color(wall_color(hit.cell, hit.side));

        for y in top..bottom {
            framebuffer.point(column, y);
        }
    }

    draw_minimap(framebuffer, maze, player);
    draw_text(framebuffer, 18, 55, "VISTA 3D - V: 2D/3D", 0xFFFFFF, 2);
    draw_text(framebuffer, 18, 82, if textures_enabled { "T: TEXTURAS ON" } else { "T: TEXTURAS OFF" }, 0xFFFFFF, 1,);
    draw_text(framebuffer, 18, 102, if music_enabled { "M: MUSICA ON" } else { "M: MUSICA OFF" }, 0xFFFFFF, 1,);
    draw_view_button(framebuffer);
    draw_rect(framebuffer, 18, 798, 760, 84, 0x101522);
    draw_text(framebuffer, 30, 806, "TECLADO: W/S MOVER | A/D GIRAR | V 2D/3D | T TEXTURAS | M MUSICA", 0xFFFFFF, 1);
    draw_text(framebuffer, 30, 828, "MOUSE: ROTACION HORIZONTAL", 0xAAB7D4, 1);
    draw_text(framebuffer, 30, 850, "XBOX: STICK IZQ MOVER | STICK DER GIRAR | A ENTER | X TEXTURAS", 0xAAB7D4, 1);
    draw_text(framebuffer, 30, 868, "Y 2D/3D | START MUSICA | D-PAD NIVELES", 0xAAB7D4, 1);
}

pub fn draw_fps(framebuffer: &mut Framebuffer, fps: u32) {
    draw_rect(framebuffer, 12, 12, 130, 30, 0x101522);
    draw_text(framebuffer, 20, 19, &format!("FPS {}", fps), 0xFFFFFF, 2);
}

pub fn draw_centered_panel(framebuffer: &mut Framebuffer, title: &str, lines: &[String]) {
    draw_rect(framebuffer, 260, 220, 780, 460, 0x101522);
    draw_text(framebuffer, 390, 275, title, 0x4AFBFF, 3);

    for (i, line) in lines.iter().enumerate() {
        draw_text(framebuffer, 390, 360 + i * 50, line, 0xFFFFFF, 2);
    }
}

fn draw_view_button(framebuffer: &mut Framebuffer) {
    draw_rect(framebuffer, 1030, 850, 250, 36, 0x101522);
    draw_text(framebuffer, 1042, 858, "V / CLICK VISTA", 0xFFFFFF, 1);
}

fn draw_cell(framebuffer: &mut Framebuffer, x: usize, y: usize, cell: char) {
    if cell != ' ' {
        draw_rect(framebuffer, x, y, BLOCK_SIZE, BLOCK_SIZE, cell_color(cell));
    }
}

fn draw_player_2d(framebuffer: &mut Framebuffer, player: &Player, offset_y: usize) {
    const PLAYER_RADIUS: i32 = 8;
    const DIRECTION_LENGTH: i32 = 20;

    let px = player.pos.x.round() as i32;
    let py = player.pos.y.round() as i32 + offset_y as i32;

    framebuffer.set_current_color(0xFFFF00);

    for dx in -PLAYER_RADIUS..=PLAYER_RADIUS {
        for dy in -PLAYER_RADIUS..=PLAYER_RADIUS {
            if dx * dx + dy * dy <= PLAYER_RADIUS * PLAYER_RADIUS {
                plot(framebuffer, px + dx, py + dy);
            }
        }
    }

    for d in 0..=DIRECTION_LENGTH {
        let x = px + (d as f32 * player.a.cos()).round() as i32;
        let y = py + (d as f32 * player.a.sin()).round() as i32;

        plot(framebuffer, x, y);
    }
}

fn draw_minimap(framebuffer: &mut Framebuffer, maze: &Maze, player: &Player) {
    const MAP_WIDTH: usize = 330;
    const MAP_HEIGHT: usize = 205;
    const PADDING: usize = 12;
    const ORIGIN_X: usize = WINDOW_WIDTH - MAP_WIDTH - PADDING;
    const ORIGIN_Y: usize = PADDING;

    draw_rect(framebuffer, ORIGIN_X, ORIGIN_Y, MAP_WIDTH, MAP_HEIGHT, 0x101522);

    let cols = maze.first().map(|row| row.len()).unwrap_or(1).max(1);
    let rows = maze.len().max(1);
    let scale_x = (MAP_WIDTH - 8) as f32 / cols as f32;
    let scale_y = (MAP_HEIGHT - 8) as f32 / rows as f32;
    let scale = scale_x.min(scale_y);
    let map_w = (cols as f32 * scale) as usize;
    let map_h = (rows as f32 * scale) as usize;
    let ox = ORIGIN_X + (MAP_WIDTH - map_w) / 2;
    let oy = ORIGIN_Y + (MAP_HEIGHT - map_h) / 2;

    for (row, line) in maze.iter().enumerate() {
        for (col, &cell) in line.iter().enumerate() {
            if matches!(cell, '+' | '-' | '|') {
                let x = ox + (col as f32 * scale) as usize;
                let y = oy + (row as f32 * scale) as usize;
                let size = scale.ceil().max(1.0) as usize;

                draw_rect(framebuffer, x, y, size, size, cell_color(cell));
            }
        }
    }

    let px = ox as f32 + (player.pos.x / BLOCK_SIZE as f32) * scale;
    let py = oy as f32 + (player.pos.y / BLOCK_SIZE as f32) * scale;
    let cx = px as i32;
    let cy = py as i32;

    framebuffer.set_current_color(0xAAB7D4);

    for i in 0..NUM_2D_RAYS {
        let fraction = if NUM_2D_RAYS > 1 {
            i as f32 / (NUM_2D_RAYS - 1) as f32
        } else {
            0.5
        };

        let angle = player.a - FOV / 2.0 + FOV * fraction;

        draw_minimap_ray(framebuffer, maze, player, angle, scale, ox, oy);
    }

    for (row, line) in maze.iter().enumerate() {
        for (col, &cell) in line.iter().enumerate() {
            if matches!(cell, 'g' | 'G') {
                let gx = ox + (col as f32 * scale) as usize;
                let gy = oy + (row as f32 * scale) as usize;
                let size = scale.ceil().max(2.0) as usize;
                
                draw_rect(framebuffer, gx, gy, size, size, 0x16C172);
            }
        }
    }

    framebuffer.set_current_color(0xFFFF00);

    for dx in -3..=3 {
        for dy in -3..=3 {
            if dx * dx + dy * dy <= 9 {
                plot(framebuffer, cx + dx, cy + dy);
            }
        }
    }

    for d in 0..10 {
        let x = cx + (d as f32 * player.a.cos() * scale).round() as i32;
        let y = cy + (d as f32 * player.a.sin() * scale).round() as i32;

        plot(framebuffer, x, y);
    }
    draw_text(framebuffer, ORIGIN_X + 10, ORIGIN_Y + MAP_HEIGHT - 25, "MINIMAPA + RAYOS + META", 0xFFFFFF, 1);
}

fn draw_minimap_ray(framebuffer: &mut Framebuffer, maze: &Maze, player: &Player, angle: f32, scale: f32, ox: usize, oy: usize,) {
    let mut distance = 0.0;
    let max_distance = 1500.0;

    while distance < max_distance {
        let world_x = player.pos.x + distance * angle.cos();
        let world_y = player.pos.y + distance * angle.sin();

        if world_x < 0.0 || world_y < 0.0 {
            break;
        }

        let col = (world_x / BLOCK_SIZE as f32) as usize;
        let row = (world_y / BLOCK_SIZE as f32) as usize;

        if row >= maze.len() || col >= maze[row].len() {
            break;
        }

        if matches!(maze[row][col], '+' | '-' | '|') {
            break;
        }

        let x = ox as f32 + (world_x / BLOCK_SIZE as f32) * scale;
        let y = oy as f32 + (world_y / BLOCK_SIZE as f32) * scale;

        plot(framebuffer, x.round() as i32, y.round() as i32);
        distance += 2.0;
    }
}

fn cell_color(cell: char) -> u32 {
    match cell {
        '+' => 0xB9B0A1, //yeso viejo
        '-' => 0x6E4A32, //madera
        '|' => 0x7B7060, //papel tapiz
        'b' => 0x8A7564, //ladrillo/piedra
        'g' | 'G' => 0xC48A3A, //puerta cerrada
        'o' | 'O' => 0x3D6B55, //puerta abierta
        _ => 0x202840,
    }
}

fn wall_color(cell: char, side: bool) -> u32 {
    let base = cell_color(cell);
    if side { darken(base, 0.65) } else { base }
}

fn darken(color: u32, factor: f32) -> u32 {
    let r = (((color >> 16) & 0xFF) as f32 * factor) as u32;
    let g = (((color >> 8) & 0xFF) as f32 * factor) as u32;
    let b = ((color & 0xFF) as f32 * factor) as u32;
    (r << 16) | (g << 8) | b
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

fn plot(framebuffer: &mut Framebuffer, x: i32, y: i32) {
    if x >= 0 && y >= 0 {
        framebuffer.point(x as usize, y as usize);
    }
}