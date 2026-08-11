mod caster;
mod framebuffer;
mod maze;
mod player;

use minifb::{Key, Window, WindowOptions};
use std::f32::consts::PI;
use std::time::Duration;
use crate::caster::cast_ray;
use crate::framebuffer::Framebuffer;
use crate::maze::{load_maze, Maze};
use crate::player::{process_events, Player};

const BLOCK_SIZE: usize = 20;
const WINDOW_WIDTH: usize = 1300;
const WINDOW_HEIGHT: usize = 500;
const NUM_RAYS: usize = 5;
const FOV: f32 = PI / 3.0;

fn cell_color(cell: char) -> u32 {
    match cell {
        '+' => 0x4afbff,
        '-' => 0x4b0082,
        '|' => 0x4afbff,
        'g' | 'G' => 0x008200,
        _ => 0xFFDDDD,
    }
}

fn draw_cell(framebuffer: &mut Framebuffer, xo: usize, yo: usize, cell: char) {
    if cell == ' ' {
        return;
    }

    framebuffer.set_current_color(cell_color(cell));

    for x in xo..xo + BLOCK_SIZE {
        for y in yo..yo + BLOCK_SIZE {
            framebuffer.point(x, y);
        }
    }
}

fn draw_player(framebuffer: &mut Framebuffer, player: &Player) {
    const PLAYER_RADIUS: i32 = 6;
    const DIRECTION_LENGTH: i32 = 14;

    let px = player.pos.x.round() as i32;
    let py = player.pos.y.round() as i32;

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

fn render(framebuffer: &mut Framebuffer, maze: &Maze, player: &Player) {
    for (row, line) in maze.iter().enumerate() {
        for (col, &cell) in line.iter().enumerate() {
            draw_cell(framebuffer, col * BLOCK_SIZE, row * BLOCK_SIZE, cell);
        }
    }

    for i in 0..NUM_RAYS {
        let ray_fraction = if NUM_RAYS > 1 {
            i as f32 / (NUM_RAYS - 1) as f32
        } else {
            0.5
        };
        let angle = player.a - FOV / 2.0 + FOV * ray_fraction;
        cast_ray(framebuffer, maze, player, angle, BLOCK_SIZE);
    }

    draw_player(framebuffer, player);
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