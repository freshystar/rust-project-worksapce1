use crate::traits::{audioplayable::AudioPlayable, videoplayable::VideoPlayable};

#[derive(Debug)]
pub struct VideoPlayer {
    audio: String,
    video: String
}
impl VideoPlayable for VideoPlayer {
    fn video(&self) {
        
    }
}
impl AudioPlayable for VideoPlayer {
    fn audio(&self) {
        println!("");
    }
}

impl VideoPlayer {
    pub fn new()-> Self {
        Self { audio: String::from("Lolita"), video: String::from("Cat drinking milk") }

    }
}