mod caster;
mod framebuffer;
mod maze;
mod player;
mod raycaster;

use minifb::{Key, MouseButton, MouseMode, Window, WindowOptions};
use std::f32::consts::PI;
use std::time::{Duration, Instant};

use crate::caster::cast_ray;
use crate::framebuffer::Framebuffer;
use crate::maze::{is_goal, load_maze, Maze};
use crate::raycaster::cast_ray as cast_ray_3d;
use crate::player::{process_events, Player};

const BLOCK_SIZE: usize = 20;
const WINDOW_WIDTH: usize = 1300;
const WINDOW_HEIGHT: usize = 700;
const NUM_2D_RAYS: usize = 15;
const FOV: f32 = PI / 3.0;

#[derive(Clone, Copy, PartialEq)]
enum ViewMode {
    View3D,
    View2D,
}

fn cell_color(cell: char) -> u32 {
    match cell {
        '+' => 0x4afbff,
        '-' => 0x4b0082,
        '|' => 0x4afbff,
        'g' | 'G' => 0x008200,
        _ => 0xFFDDDD,
    }
}

fn darken(color: u32, factor: f32) -> u32 {
    let r = (((color >> 16) & 0xFF) as f32 * factor) as u32;
    let g = (((color >> 8) & 0xFF) as f32 * factor) as u32;
    let b = ((color & 0xFF) as f32 * factor) as u32;

    (r << 16) | (g << 8) | b
}

fn draw_rect(framebuffer: &mut Framebuffer, x0: usize, y0: usize, width: usize, height: usize, color: u32) {
    framebuffer.set_current_color(color);
    
    let x1 = (x0 + width).min(framebuffer.width);
    let y1 = (y0 + height).min(framebuffer.height);

    for y in y0.min(framebuffer.height)..y1 {
        for x in x0.min(framebuffer.width)..x1 {
            framebuffer.point(x, y);
        }
    }
}

fn draw_cell(framebuffer: &mut Framebuffer, xo: usize, yo: usize, cell: char) {
    if cell == ' ' {
        return;
    }
    draw_rect(framebuffer, xo, yo, BLOCK_SIZE, BLOCK_SIZE, cell_color(cell));
}

fn draw_player(framebuffer: &mut Framebuffer, player: &Player, origin_y: usize) {
    const PLAYER_RADIUS: i32 = 6;
    const DIRECTION_LENGTH: i32 = 16;

    let px = player.pos.x.round() as i32;
    let py = player.pos.y.round() as i32 + origin_y as i32;

    framebuffer.set_current_color(0xFFFF00);
    for dx in -PLAYER_RADIUS..=PLAYER_RADIUS {
        for dy in -PLAYER_RADIUS..=PLAYER_RADIUS {
            if dx * dx + dy * dy <= PLAYER_RADIUS * PLAYER_RADIUS {
                let x = px + dx;
                let y = py + dy;

                if x >= 0 && y >= 0 {
                    framebuffer.point(x as usize, y as usize);
                }
            }
        }
    }

    for d in 0..=DIRECTION_LENGTH {
        let x = px + (d as f32 * player.a.cos()).round() as i32;
        let y = py + (d as f32 * player.a.sin()).round() as i32;
        if x >= 0 && y >= 0 {
            framebuffer.point(x as usize, y as usize);
        }
    }
}

fn render_2d(framebuffer: &mut Framebuffer, maze: &Maze, player: &Player) {
    let origin_y = 0;

    for (row, line) in maze.iter().enumerate() {
        for (col, &cell) in line.iter().enumerate() {
            draw_cell(framebuffer, col * BLOCK_SIZE, origin_y + row * BLOCK_SIZE, cell);
        }
    }

    for i in 0..NUM_2D_RAYS {
        let ray_fraction = if NUM_2D_RAYS > 1 {
            i as f32 / (NUM_2D_RAYS - 1) as f32
        } else {
            0.5
        };

        let angle = player.a - FOV / 2.0 + FOV * ray_fraction;

        cast_ray(framebuffer, maze, player, angle, BLOCK_SIZE, origin_y);
    }

    draw_player(framebuffer, player, origin_y);
    draw_rect(framebuffer, 1030, 640, 250, 36, 0x101522);
    draw_text(framebuffer, 1042, 648, "V / CLICK VISTA", 0xFFFFFF, 2);
}

fn wall_color(cell: char, side: bool) -> u32 {
    let base = match cell {
        '+' => 0x4AFBFF,
        '-' => 0x9B59FF,
        '|' => 0x1685FF,
        _ => 0xFFFFFF,
    };
    if side { darken(base, 0.65) } else { base }
}

fn render_3d(framebuffer: &mut Framebuffer, maze: &Maze, player: &Player) {
    draw_rect(framebuffer, 0, 0, WINDOW_WIDTH, WINDOW_HEIGHT / 2, 0x18233D);
    draw_rect(
        framebuffer,
        0,
        WINDOW_HEIGHT / 2,
        WINDOW_WIDTH,
        WINDOW_HEIGHT / 2,
        0x302A2A,
    );

    let projection_distance = (WINDOW_WIDTH as f32 / 2.0) / (FOV / 2.0).tan();

    for column in 0..WINDOW_WIDTH {
        let camera = (2.0 * column as f32 / WINDOW_WIDTH as f32) - 1.0;
        let ray_angle = player.a + (camera * (FOV / 2.0).tan()).atan();

        let Some(hit) = cast_ray_3d(maze, player, ray_angle, BLOCK_SIZE) else {
            continue;
        };

        let perpendicular_distance = (hit.distance * (ray_angle - player.a).cos()).max(0.1);
        let wall_height = (BLOCK_SIZE as f32 * projection_distance / perpendicular_distance).min(WINDOW_HEIGHT as f32 * 2.0);
        let top = ((WINDOW_HEIGHT as f32 - wall_height) / 2.0).max(0.0) as usize;
        let bottom = ((WINDOW_HEIGHT as f32 + wall_height) / 2.0).min(WINDOW_HEIGHT as f32) as usize;

        framebuffer.set_current_color(wall_color(hit.cell, hit.side));
        
        for y in top..bottom.max(top + 1).min(WINDOW_HEIGHT) {
            framebuffer.point(column, y);
        }
    }

    draw_minimap(framebuffer, maze, player);
    draw_rect(framebuffer, 1030, 640, 250, 36, 0x101522);
    draw_text(framebuffer, 1042, 648, "V / CLICK VISTA", 0xFFFFFF, 2);
}

fn main() {
    let (maze, mut player) = load_maze("./maze.txt", BLOCK_SIZE);
    let mut framebuffer = Framebuffer::new(WINDOW_WIDTH, WINDOW_HEIGHT);
    
    framebuffer.set_background_color(0x333355);

    let mut window = Window::new(
        "Maze Runner - Vista del mapa",
        WINDOW_WIDTH,
        WINDOW_HEIGHT,
        WindowOptions::default(),
    )
    .expect("no se pudo crear la ventana");

    let frame_delay = Duration::from_millis(33);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        process_events(&window, &mut player);

        let i = player.pos.x.max(0.0) as usize / BLOCK_SIZE;
        let j = player.pos.y.max(0.0) as usize / BLOCK_SIZE;

        if maze.get(j).and_then(|row| row.get(i)) == Some(&'g') {
            println!("¡Meta alcanzada! Fin del juego.");
            break;
        }

        framebuffer.clear();
        render(&mut framebuffer, &maze, &player);

        window
            .update_with_buffer(&framebuffer.buffer, WINDOW_WIDTH, WINDOW_HEIGHT)
            .expect("no se pudo actualizar el framebuffer");

        std::thread::sleep(frame_delay);
    }
}