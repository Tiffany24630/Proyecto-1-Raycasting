use std::collections::HashMap;
use std::path::Path;
use image::io::Reader as ImageReader;

pub struct Texture {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<u32>,
}

pub struct TextureSet {
    textures: HashMap<char, Texture>,
}

impl TextureSet {
    pub fn load() -> Self {
        let mut textures = HashMap::new();

        textures.insert('+', load_texture("assets/textures/wall_cyan.png"));
        textures.insert('-', load_texture("assets/textures/wall_purple.png"));
        textures.insert('|', load_texture("assets/textures/wall_blue.png"));

        Self { textures }
    }

    pub fn get(&self, cell: char) -> Option<&Texture> {
        self.textures.get(&cell)
    }
}

fn load_texture<P: AsRef<Path>>(path: P) -> Texture {
    let path_ref = path.as_ref();
    let image = ImageReader::open(path_ref).unwrap_or_else(|e| panic!("no se pudo abrir la textura {:?}: {}", path_ref, e)).decode().unwrap_or_else(|e| panic!("no se pudo decodificar la textura {:?}: {}", path_ref, e)).to_rgb8();
    let (width, height) = image.dimensions();
    let pixels = image.pixels().map(|p| ((p[0] as u32) << 16) | ((p[1] as u32) << 8) | p[2] as u32).collect();

    Texture {
        width: width as usize,
        height: height as usize,
        pixels,
    }
}
