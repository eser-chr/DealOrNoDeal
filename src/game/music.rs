use rodio::{Decoder, OutputStream, Sink, Source};
use std::fs::File;
use std::io::BufReader;
use std::sync::Arc;

pub fn load_music(path: &str) -> Arc<dyn Source<Item = f32> + Send + Sync> {
    let file = File::open(path).expect("Failed to open audio file");
    let source = Decoder::new(BufReader::new(file))
        .expect("Failed to decode audio file");

    // Convert the source into a buffered cloneable version and wrap it in Arc
    Arc::new(source.buffered())
}

pub fn play_loaded_music(audio: Arc<dyn Source<Item = f32> + Send + Sync>) {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

    // Clone the buffered source each time you want to play it
    sink.append(audio.clone());
    sink.sleep_until_end();
}
