use text_io::read;
use morse::encode;
use std::time::Duration;
use std::fs::File;
use std::io::BufReader;
use rodio::{Decoder, OutputStream, Source, source::SineWave, cpal::SampleRate};

mod oscillator;

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

    //oscillator::run();

    let wave_table_size = 64;
    let mut wave_table: Vec<f32> = Vec::with_capacity(wave_table_size);

    for n in 0..wave_table_size {
        wave_table.push((2.0 * std::f32::consts::PI * n as f32 / wave_table_size as f32).sin());
    }
    
    let mut osc = oscillator::WavetableOscillator::new(48000, wave_table);

    osc.set_frequency(880.0);
    
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    
    let _result = stream_handle.play_raw(osc.convert_samples());

    std::thread::sleep(std::time::Duration::from_secs(1));

    //let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    //let mut sine = SineWave::new(880.0);  
    //let _result = stream_handle.play_raw(sine.convert_samples());

    //std::thread::sleep(std::time::Duration::from_secs(1));
}

