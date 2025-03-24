use crate::traits::audioplayable::AudioPlayable;

#[derive(Debug)]

pub struct AudioPlayer {
    audio: String
}
impl AudioPlayable for AudioPlayer {
    fn audio(&self) {
        
    }
}

impl AudioPlayer {
   pub fn new() -> Self {
        Self { audio: String::from("Despasito")}
    }
 }