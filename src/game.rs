#[derive(Clone, Copy, PartialEq)]
pub enum GameState {
    Welcome,
    LevelSelect,
    Playing,
    Success,
}

pub const LEVELS: [&str; 3] = [
    "levels/recibidor.txt",
    "levels/pasillos.txt",
    "levels/atico.txt",
];

pub fn level_name(index: usize) -> String {
    match index {
        0 => "EL RECIBIDOR".to_string(),
        1 => "LOS PASILLOS".to_string(),
        2 => "EL ATICO".to_string(),
        _ => format!("NIVEL {}", index + 1),
    }
}