use crate::framebuffer::Framebuffer;
use crate::maze::Maze;
use crate::player::Player;

pub fn cast_ray(framebuffer: &mut Framebuffer, maze: &Maze, player: &Player, a: f32, block_size: usize, offset_y: usize,) {
    let mut d = 0.0;

    framebuffer.set_current_color(0xFFDDDD);

    loop {
        let xf = player.pos.x + d * a.cos();
        let yf = player.pos.y + d * a.sin();

        if xf < 0.0 || yf < 0.0 {
            return;
        }

        let x = xf as usize;
        let y = yf as usize;
        let i = x / block_size;
        let j = y / block_size;

        if j >= maze.len() || i >= maze[j].len() {
            return;
        }

        let cell = maze[j][i];
        
        if matches!(cell, '+' | '-' | '|') {
            return;
        }

        framebuffer.point(x, y + offset_y);
        d += 1.0;
    }
}
