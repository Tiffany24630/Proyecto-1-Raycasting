mod caster;
mod framebuffer;
mod input;
mod maze;
mod player;
mod raycaster;
mod render;
mod text;

use minifb::{Key, MouseButton, Window, WindowOptions};
use std::time::{Duration, Instant};

use crate::framebuffer::Framebuffer;
use crate::input::{update_mouse_rotation, view_button_down};
use crate::maze::{is_goal, load_maze};
use crate::player::process_events;
use crate::render::{draw_fps, render_2d, render_3d, WINDOW_HEIGHT, WINDOW_WIDTH};

#[derive(Clone, Copy, PartialEq)]
enum ViewMode {
    View3D,
    View2D,
}

fn main() {
    let (maze, mut player) = load_maze("./maze.txt", render::BLOCK_SIZE);
    let mut framebuffer = Framebuffer::new(WINDOW_WIDTH, WINDOW_HEIGHT);
    let mut window = Window::new(
        "Maze Runner - Vista 3D",
        WINDOW_WIDTH,
        WINDOW_HEIGHT,
        WindowOptions::default(),
    ).expect("no se pudo crear la ventana");

    let frame_delay = Duration::from_millis(33);
    let mut view_mode = ViewMode::View3D;
    let mut last_toggle = Instant::now() - Duration::from_secs(1);
    let mut mouse_was_down = false;
    let mut last_mouse_x: Option<f32> = None;
    let mut fps_timer = Instant::now();
    let mut frames = 0u32;
    let mut fps = 0u32;

    window.set_cursor_visibility(true);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let frame_start = Instant::now();
        let mouse_down = window.get_mouse_down(MouseButton::Left);
        let mouse_clicked_button = view_button_down(&window) && !mouse_was_down;
        let key_toggle = window.is_key_down(Key::V) && last_toggle.elapsed() >= Duration::from_millis(250);

        if (key_toggle || mouse_clicked_button) && last_toggle.elapsed() >= Duration::from_millis(250)
        {
            view_mode = match view_mode {
                ViewMode::View3D => ViewMode::View2D,
                ViewMode::View2D => ViewMode::View3D,
            };
            last_toggle = Instant::now();
            last_mouse_x = None;
            window.set_title(match view_mode {
                ViewMode::View3D => "Maze Runner - Vista 3D",
                ViewMode::View2D => "Maze Runner - Vista 2D",
            });
        }

        mouse_was_down = mouse_down;

        process_events(&window, &mut player, &maze, render::BLOCK_SIZE);
        update_mouse_rotation(
            &window,
            &mut player,
            &mut last_mouse_x,
            view_mode == ViewMode::View3D,
        );

        if is_goal(&maze, &player, render::BLOCK_SIZE) {
            println!("¡Meta alcanzada! Fin del juego.");
            break;
        }

        framebuffer.clear();

        match view_mode {
            ViewMode::View3D => render_3d(&mut framebuffer, &maze, &player),
            ViewMode::View2D => render_2d(&mut framebuffer, &maze, &player),
        }

        frames += 1;

        if fps_timer.elapsed() >= Duration::from_secs(1) {
            fps = frames;
            frames = 0;
            fps_timer = Instant::now();
        }

        draw_fps(&mut framebuffer, fps);

        window.update_with_buffer(&framebuffer.buffer, WINDOW_WIDTH, WINDOW_HEIGHT).expect("no se pudo actualizar el framebuffer");

        let elapsed = frame_start.elapsed();

        if elapsed < frame_delay {
            std::thread::sleep(frame_delay - elapsed);
        }
    }
}