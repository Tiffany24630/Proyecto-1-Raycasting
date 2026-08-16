use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink, Source};

pub struct AudioManager {
    _stream: Option<OutputStream>,
    _handle: Option<OutputStreamHandle>,
    music: Option<Sink>,
    enabled: bool,
}

impl AudioManager {
    pub fn new() -> Self {
        match OutputStream::try_default() {
            Ok((stream, handle)) => {
                let sink = Sink::try_new(&handle).ok();
                let mut manager = Self {
                    _stream: Some(stream),
                    _handle: Some(handle),
                    music: sink,
                    enabled: true,
                };
                
                manager.play_music();
                manager
            }
            Err(error) => {
                eprintln!("No se pudo inicializar audio: {}", error);
                Self {
                    _stream: None,
                    _handle: None,
                    music: None,
                    enabled: false,
                }
            }
        }
    }

    fn play_music(&mut self) {
        let Some(sink) = &self.music else { return };

        let paths = [
            Path::new("assets/audio/music.ogg"),
            Path::new("assets/audio/music.wav"),
        ];

        let Some(path) = paths.iter().find(|path| path.exists()) else {
            eprintln!("No se encontró música en assets/audio/music.ogg ni music.wav.");
            self.enabled = false;
            return;
        };

        let Ok(file) = File::open(path) else {
            eprintln!("No se pudo abrir la música {:?}.", path);
            self.enabled = false;
            return;
        };

        let Ok(source) = Decoder::new(BufReader::new(file)) else {
            eprintln!("No se pudo decodificar la música {:?}.", path);
            self.enabled = false;
            return;
        };

        sink.append(source.repeat_infinite());
        sink.set_volume(0.18);
        sink.play();
    }

    pub fn toggle_music(&mut self) {
        let Some(sink) = &self.music else { return };

        if sink.is_paused() {
            sink.play();
            self.enabled = true;
        } else {
            sink.pause();
            self.enabled = false;
        }
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
}
