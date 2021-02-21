use std::thread::{self, sleep};
use std::time;

fn main() {
    let bpm = 120;
    let time_signature = 4;
    let beat_player = get_beat_player(BeatPlayerSelection::Audio);

    println!(
        "Starting metronome at {} bpm and {} beats per bar",
        bpm, time_signature
    );

    thread::spawn(move || loop {
        let mut beat_counter = 0;
        for _i in 0..time_signature {
            sleep(time::Duration::from_millis(bpm_to_ms(bpm)));
            beat_counter += 1;
            beat_player.play(beat_counter);
        }
    });

    loop {}
}

fn bpm_to_ms(bpm: u64) -> u64 {
    //  1 min => 60 seconds => 60000 ms
    60000 / bpm
}

enum BeatPlayerSelection {
    Audio,
    Text,
}

pub trait BeatPlayer {
    fn play(&self, beat_counter: u64);
}

pub struct TextBeatPlayer;

impl BeatPlayer for TextBeatPlayer {
    fn play(&self, beat_counter: u64) {
        println!("{}", beat_counter);
    }
}
pub struct AudioBeatPlayer;

impl BeatPlayer for AudioBeatPlayer {
    fn play(&self, beat_counter: u64) {
        println!("->{}", beat_counter);
        let beat_freq = if beat_counter == 1 { 640 } else { 440 };

        let (_stream, stream_handle) = rodio::OutputStream::try_default().unwrap();
        let sink = rodio::Sink::try_new(&stream_handle).unwrap();
        let source = rodio::source::SineWave::new(beat_freq);
        sink.append(source);

        let (_stream, stream_handle) = rodio::OutputStream::try_default().unwrap();
        let sink = rodio::Sink::try_new(&stream_handle).unwrap();
        let source = rodio::source::SineWave::new(beat_freq);
        sink.append(source);

        // use rodio::Source;
        // use std::fs::File;
        // use std::io::BufReader;

        // let (_stream, stream_handle) = rodio::OutputStream::try_default().unwrap();
        // // Load a sound from a file, using a path relative to Cargo.toml
        // let file = File::open("src/assets/sounds/mixkit-typewriter-hit-1362.wav").unwrap();
        // let source = rodio::Decoder::new(BufReader::new(file)).unwrap();
        // stream_handle.play_raw(source.convert_samples()).unwrap();

        // let (_stream, stream_handle) = rodio::OutputStream::try_default().unwrap();
        // // Load a sound from a file, using a path relative to Cargo.toml
        // let file = File::open("src/assets/sounds/mixkit-typewriter-hit-1362.wav").unwrap();
        // let source = rodio::Decoder::new(BufReader::new(file)).unwrap();
        // stream_handle.play_raw(source.convert_samples()).unwrap();
    }
}

fn get_beat_player(audio: BeatPlayerSelection) -> Box<dyn BeatPlayer + Send + Sync> {
    match audio {
        BeatPlayerSelection::Audio => Box::new(AudioBeatPlayer {}),
        BeatPlayerSelection::Text => Box::new(TextBeatPlayer {}),
    }
}
