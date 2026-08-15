use std::f32::consts::PI;
use std::fs::File;
use std::io::{BufRead, BufReader};
use nalgebra_glm::Vec2;
use crate::player::Player;
pub type Maze = Vec<Vec<char>>;

pub fn load_maze(filename: &str, block_size: usize) -> (Maze, Player) {
    let file = File::open(filename).expect("no se pudo abrir el archivo del laberinto");
    let reader = BufReader::new(file);
    
    let mut maze: Maze = Vec::new();
    let mut player_pos: Option<Vec2> = None;
    let mut expected_width: Option<usize> = None;

    for (row, line) in reader.lines().enumerate() {
        let line = line.expect("no se pudo leer una línea del laberinto");

        let mut cells: Vec<char> = Vec::new();

        if let Some(width) = expected_width {
            if cells.len() != width {
                panic!(
                    "el mapa debe ser rectangular: la fila {} tiene {} columnas y se esperaban {}",
                    row,
                    cells.len(),
                    width
                );
            }
        } else {
            expected_width = Some(cells.len());
        }

        for (col, character) in line.chars().enumerate() {
            if character == 'p' {
                let x = col * block_size + block_size / 2;
                let y = row * block_size + block_size / 2;

                player_pos = Some(Vec2::new(x as f32, y as f32));
                *character = ' ';
            }
        }

        maze.push(cells);
    }

    let player = Player {
        pos: player_pos.unwrap_or_else(|| Vec2::new(block_size as f32 * 1.5, block_size as f32 * 1.5)),
        a: PI / 3.0,
        controller_move: 0.0,
        controller_rotate: 0.0,
        controller_forward: false,
        controller_backward: false,
    };

    (maze, player)
}

pub fn is_goal(maze: &Maze, player: &Player, block_size: usize) -> bool {
    if player.pos.x < 0.0 || player.pos.y < 0.0 {
        return false;
    }

    let col = (player.pos.x / block_size as f32) as usize;
    let row = (player.pos.y / block_size as f32) as usize;
    
    matches!(maze.get(row).and_then(|line| line.get(col)), Some('g'))
}