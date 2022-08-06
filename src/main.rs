use text_io::read;
use morse::encode;
use std::time::Duration;
use std::fs::File;
use std::io::BufReader;
use rodio::{Decoder, OutputStream, Source, source::SineWave, cpal::SampleRate};

fn main() 
{
    // Get string from user 
    let line: String = read!("{}\n");
    println!("Your input: {}", line);

    // Print String by words
    for word in line.split_whitespace()
    {
        let morse_code: String = encode::encode(word).unwrap();
        println!("{}", morse_code);

        // Slice by letter and create audio data based on '.' or '-' to play in the end

    }

    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let mut sine = SineWave::new(880.0);  
    let _result = stream_handle.play_raw(sine.convert_samples());

    std::thread::sleep(std::time::Duration::from_secs(1));
}

