use std::fs::File;
use std::io::BufReader;
use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink, Source};

pub struct AudioManager {
    _stream: Option<OutputStream>,
    _handle: Option<OutputStreamHandle>,
    music: Option<Sink>,
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
                }
            }
        }
    }

    fn play_music(&mut self) {
        let Some(sink) = &self.music else { return };
        let file = File::open("assets/audio/music.wav").or_else(|_| File::open("assets/audio/music.ogg"));
        
        let Ok(file) = file else {
            eprintln!("No se encontró assets/audio/music.ogg ni music.wav; el juego continuará sin música.");
            return;
        };

        let Ok(source) = Decoder::new(BufReader::new(file)) else {
            eprintln!("No se pudo decodificar la música de fondo.");
            return;
        };

        sink.append(source.repeat_infinite());
        sink.set_volume(0.18);
        sink.play();
    }
}
