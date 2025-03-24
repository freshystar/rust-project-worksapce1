use audio::AudioPlayer;
use traits::{audioplayable::AudioPlayable, videoplayable::VideoPlayable};
use video::VideoPlayer;



fn main() {
   let audio = AudioPlayer::new();
   let video = VideoPlayer::new();

   println!("audio: {:?} ", audio.audio());
   println!("video: {:?} ", video.video());
}


mod audio;
mod traits;
mod video;