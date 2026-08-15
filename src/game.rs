#[derive(Clone, Copy, PartialEq)]
pub enum GameState {
    Welcome,
    LevelSelect,
    Playing,
    Success,
}

pub const LEVELS: [&str; 7] = [
    "./maze1.txt",
    "./maze2.txt",
    "./maze3.txt",
    "./maze4.txt",
    "./maze5.txt",
    "./maze6.txt",
    "./maze7.txt",
];

pub fn level_name(index: usize) -> String {
    format!("NIVEL {}", index + 1)
}
