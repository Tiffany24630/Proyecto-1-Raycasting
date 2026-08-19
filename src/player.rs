use minifb::{Key, Window};
use nalgebra_glm::Vec2;
use std::f32::consts::PI;
use crate::maze::Maze;

pub struct Player {
    pub pos: Vec2,
    pub a: f32,
    pub controller_move: f32,
    pub controller_rotate: f32,
    pub controller_strafe: f32,
    pub controller_forward: bool,
    pub controller_backward: bool,
}

fn is_wall(maze: &Maze, x: f32, y: f32, block_size: usize) -> bool {
    if x < 0.0 || y < 0.0 {
        return true;
    }

    let col = (x / block_size as f32) as usize;
    let row = (y / block_size as f32) as usize;

    match maze.get(row).and_then(|line| line.get(col)) {
        Some(&cell) => matches!(cell, '+' | '-' | '|' | 'b' | 'o'),
        None => true,
    }
}

fn can_move_to(maze: &Maze, position: Vec2, block_size: usize) -> bool {
    const PLAYER_RADIUS: f32 = 5.0;
    let offsets = [
        (-PLAYER_RADIUS, -PLAYER_RADIUS),
        (PLAYER_RADIUS, -PLAYER_RADIUS),
        (-PLAYER_RADIUS, PLAYER_RADIUS),
        (PLAYER_RADIUS, PLAYER_RADIUS),
        (0.0, -PLAYER_RADIUS),
        (0.0, PLAYER_RADIUS),
        (-PLAYER_RADIUS, 0.0),
        (PLAYER_RADIUS, 0.0),
    ];

    offsets.iter().all(|(dx, dy)| !is_wall(maze, position.x + dx, position.y + dy, block_size))
}

pub fn process_events(window: &Window, player: &mut Player, maze: &Maze, block_size: usize) {
    const MOVE_SPEED: f32 = 4.5;
    const ROTATION_SPEED: f32 = PI / 45.0;

    if window.is_key_down(Key::Left) {
        player.a -= ROTATION_SPEED;
    }

    if window.is_key_down(Key::Right) {
        player.a += ROTATION_SPEED;
    }

    player.a += player.controller_rotate * 0.045;

    let mut forward = 0.0;

    if window.is_key_down(Key::W) {
        forward += MOVE_SPEED;
    }

    if window.is_key_down(Key::S) {
        forward -= MOVE_SPEED;
    }

    forward += player.controller_move * MOVE_SPEED;

    let mut strafe = 0.0;

    if window.is_key_down(Key::A) {
        strafe -= MOVE_SPEED;
    }

    if window.is_key_down(Key::D) {
        strafe += MOVE_SPEED;
    }

    strafe += player.controller_strafe * MOVE_SPEED;

    let dx = forward * player.a.cos() - strafe * player.a.sin();
    let dy = forward * player.a.sin() + strafe * player.a.cos();

    if dx != 0.0 || dy != 0.0 {
        let next_x = Vec2::new(player.pos.x + dx, player.pos.y);

        if can_move_to(maze, next_x, block_size) {
            player.pos.x = next_x.x;
        }

        let next_y = Vec2::new(player.pos.x, player.pos.y + dy);
        
        if can_move_to(maze, next_y, block_size) {
            player.pos.y = next_y.y;
        }
    }

    if player.a > PI {
        player.a -= 2.0 * PI;
    }
    
    if player.a < -PI {
        player.a += 2.0 * PI;
    }
}