use crate::maze::Maze;
use crate::player::Player;

#[derive(Clone, Copy, Debug)]
pub struct RayHit {
    pub distance: f32,
    pub cell: char,
    pub side: bool,
    pub texture_u: f32,
}

pub fn cast_ray(maze: &Maze, player: &Player, ray_angle: f32, block_size: usize,) -> Option<RayHit> {
    let block = block_size as f32;
    let ray_dir_x = ray_angle.cos();
    let ray_dir_y = ray_angle.sin();

    let mut map_x = (player.pos.x / block).floor() as i32;
    let mut map_y = (player.pos.y / block).floor() as i32;

    if map_x < 0 || map_y < 0 {
        return None;
    }

    let delta_dist_x = if ray_dir_x.abs() < 0.000001 {
        f32::MAX
    } else {
        1.0 / ray_dir_x.abs()
    };

    let delta_dist_y = if ray_dir_y.abs() < 0.000001 {
        f32::MAX
    } else {
        1.0 / ray_dir_y.abs()
    };

    let (step_x, mut side_dist_x) = if ray_dir_x < 0.0 {
        (-1, (player.pos.x / block - map_x as f32) * delta_dist_x)
    } else {
        (1, (map_x as f32 + 1.0 - player.pos.x / block) * delta_dist_x)
    };

    let (step_y, mut side_dist_y) = if ray_dir_y < 0.0 {
        (-1, (player.pos.y / block - map_y as f32) * delta_dist_y)
    } else {
        (1, (map_y as f32 + 1.0 - player.pos.y / block) * delta_dist_y)
    };

    for _ in 0..512 {
        let side;
        let distance_cells;

        if side_dist_x < side_dist_y {
            distance_cells = side_dist_x;
            side_dist_x += delta_dist_x;
            map_x += step_x;
            side = false;
        } else {
            distance_cells = side_dist_y;
            side_dist_y += delta_dist_y;
            map_y += step_y;
            side = true;
        }

        if map_x < 0
            || map_y < 0
            || map_y as usize >= maze.len()
            || map_x as usize >= maze[map_y as usize].len()
        {
            return None;
        }

        let cell = maze[map_y as usize][map_x as usize];

        if matches!(cell, '+' | '-' | '|' | 'b' | 'g' | 'o') {
            let distance = (distance_cells * block).max(0.001);
            let hit_x = player.pos.x + ray_dir_x * distance;
            let hit_y = player.pos.y + ray_dir_y * distance;
            
            let texture_u = if side {
                (hit_x / block).fract()
            } else {
                (hit_y / block).fract()
            };

            return Some(RayHit {
                distance,
                cell,
                side,
                texture_u: if texture_u < 0.0 { texture_u + 1.0 } else { texture_u },
            });
        }
    }
    None
}