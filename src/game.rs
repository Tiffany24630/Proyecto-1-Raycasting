#[derive(Clone, Copy, PartialEq)]
pub enum GameState {
    Welcome,
    LevelSelect,
    Playing,
    Success,
}

pub const LEVELS: [&str; 3] = [
    "levels/level1.txt",
    "levels/level2.txt",
    "levels/level3.txt",
];

pub fn level_name(index: usize) -> String {
    match index {
        0 => "Fácil".to_string(),
        1 => "Intermedio".to_string(),
        2 => "Difícil".to_string(),
        _ => format!("NIVEL {}", index + 1),
    }
}