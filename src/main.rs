use text_io::read;
use morse::encode;
use std::time::Duration;
use std::fs::File;
use std::io::BufReader;
use rodio::{Decoder, OutputStream, Source};

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
    }

    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
   
    let file = BufReader::new(File::open("sound/sine.ogg").unwrap());

    let source = Decoder::new(file).unwrap();

    stream_handle.play_raw(source.convert_samples());

    std::thread::sleep(std::time::Duration::from_secs(5));
}

