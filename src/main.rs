mod audio;
mod caster;
mod framebuffer;
mod game;
mod input;
mod maze;
mod player;
mod raycaster;
mod render;
mod screens;
mod text;
mod textures;

use minifb::{Key, KeyRepeat, MouseButton, Window, WindowOptions};
use std::time::{Duration, Instant};
use crate::audio::AudioManager;
use crate::framebuffer::Framebuffer;
use crate::game::{GameState, LEVELS};
use crate::input::{update_mouse_rotation, view_button_down, ControllerInput};
use crate::maze::{is_goal, load_maze};
use crate::player::process_events;
use crate::render::{draw_fps, render_2d, render_3d, BLOCK_SIZE, WINDOW_HEIGHT, WINDOW_WIDTH};
use crate::screens::{render_level_select, render_success, render_welcome, update_level_selection};
use crate::textures::TextureSet;

fn main() {
    let mut framebuffer = Framebuffer::new(WINDOW_WIDTH, WINDOW_HEIGHT);
    let mut window = Window::new("Maze Runner - Bienvenida", WINDOW_WIDTH, WINDOW_HEIGHT, WindowOptions::default(),).expect("no se pudo crear la ventana");

    let textures = TextureSet::load();
    
    let mut audio = AudioManager::new();
    let mut textures_enabled = true;
    let mut controller = ControllerInput::new();
    let mut state = GameState::Welcome;
    let mut selected_level = 0usize;
    let mut maze = Vec::new();
    let mut player = None;
    let mut view_mode = ViewMode::View3D;
    let mut last_toggle = Instant::now() - Duration::from_secs(1);
    let mut mouse_was_down = false;
    let mut last_mouse_x: Option<f32> = None;

    let frame_delay = Duration::from_millis(80);

    let mut fps_timer = Instant::now();
    let mut frames = 0u32;
    let mut fps = 0u32;

    window.set_cursor_visibility(true);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let frame_start = Instant::now();

        controller.update();

        if controller.back_pressed() {
            break;
        }

        framebuffer.clear();

        match state {
            GameState::Welcome => {
                render_welcome(&mut framebuffer);

                if window.is_key_pressed(Key::Enter, KeyRepeat::No) || controller.confirm_pressed() {
                    state = GameState::LevelSelect;

                    window.set_title("Maze Runner - Selección de nivel");
                }
            }

            GameState::LevelSelect => {
                update_level_selection(&window, &controller, &mut selected_level);
                render_level_select(&mut framebuffer, selected_level);

                if window.is_key_pressed(Key::Enter, KeyRepeat::No) || controller.confirm_pressed() {
                    let (new_maze, new_player) = load_maze(LEVELS[selected_level], BLOCK_SIZE);
                    
                    maze = new_maze;
                    player = Some(new_player);
                    view_mode = ViewMode::View3D;
                    last_mouse_x = None;
                    state = GameState::Playing;
                    window.set_title(&format!("Maze Runner - Nivel {}", selected_level + 1));
                }
            }

            GameState::Playing => {
                let current_player = player.as_mut().expect("jugador no inicializado");
                let mouse_down = window.get_mouse_down(MouseButton::Left);
                let mouse_clicked_button = view_button_down(&window) && !mouse_was_down;
                let key_toggle = (window.is_key_pressed(Key::V, KeyRepeat::No) || controller.toggle_view_pressed()) && last_toggle.elapsed() >= Duration::from_millis(250);

                if (key_toggle || mouse_clicked_button) && last_toggle.elapsed() >= Duration::from_millis(250){
                    view_mode = match view_mode {
                        ViewMode::View3D => ViewMode::View2D,
                        ViewMode::View2D => ViewMode::View3D,
                    };

                    last_toggle = Instant::now();
                    last_mouse_x = None;
                }
                
                mouse_was_down = mouse_down;

                controller.apply_to_player(current_player);

                if window.is_key_pressed(Key::T, KeyRepeat::No) || controller.toggle_textures_pressed() {
                    textures_enabled = !textures_enabled;
                }

                if window.is_key_pressed(Key::M, KeyRepeat::No) || controller.toggle_music_pressed() {
                    audio.toggle_music();
                }

                process_events(&window, current_player, &maze, BLOCK_SIZE);
                update_mouse_rotation(&window, current_player, &mut last_mouse_x, view_mode == ViewMode::View3D,);

                match view_mode {
                    ViewMode::View3D => render_3d(&mut framebuffer, &maze, current_player, &textures, textures_enabled, audio.is_enabled()),
                    ViewMode::View2D => render_2d(&mut framebuffer, &maze, current_player),
                }

                if is_goal(&maze, current_player, BLOCK_SIZE) {
                    state = GameState::Success;
                    last_mouse_x = None;

                    window.set_title("Maze Runner - ¡Nivel completado!");
                }
            }

            GameState::Success => {
                render_success(&mut framebuffer, selected_level);

                if window.is_key_pressed(Key::Enter, KeyRepeat::No) || controller.confirm_pressed() {
                    selected_level = (selected_level + 1) % LEVELS.len();
                    
                    let (new_maze, new_player) = load_maze(LEVELS[selected_level], BLOCK_SIZE);
                    
                    maze = new_maze;
                    player = Some(new_player);
                    view_mode = ViewMode::View3D;
                    state = GameState::Playing;
                    window.set_title(&format!("Maze Runner - Nivel {}", selected_level + 1));
                }
            }
        }

        frames += 1;

        if fps_timer.elapsed() >= Duration::from_secs(1) {
            fps = frames;
            frames = 0;
            fps_timer = Instant::now();
        }

        if matches!(state, GameState::Playing) {
            draw_fps(&mut framebuffer, fps);
        }

        window.update_with_buffer(&framebuffer.buffer, WINDOW_WIDTH, WINDOW_HEIGHT).expect("no se pudo actualizar el framebuffer");

        let elapsed = frame_start.elapsed();

        if elapsed < frame_delay {
            std::thread::sleep(frame_delay - elapsed);
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
enum ViewMode {
    View3D,
    View2D,
}