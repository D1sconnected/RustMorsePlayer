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

    // Prepare vector for playback
    let wave_table_size = 64;
    let mut wave_table: Vec<i32> = Vec::with_capacity(wave_table_size);

    // Print String by words & Fill wave_table by letters
    for word in line.split_whitespace()
    {
        let morse_code: String = encode::encode(word).unwrap();
        println!("{}", morse_code);

        // Slice by letter and create audio data based on '.' or '-' to play in the end
        for letter in morse_code.chars()
        {
            println!("Input: {}", letter);

            if letter == '.' 
            {
                println!("dot");
                wave_table.push(1);
                
            }
            else
            {
                println!("line");
                wave_table.push(2);
            }
        }
    }

    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    // let mut sine = SineWave::new(880.0);  
    // let _result = stream_handle.play_raw(sine.convert_samples());

    for elem in &wave_table
    {
        println!("{}", elem);

        if *elem == 1
        {
            let mut sine = SineWave::new(880.0);  
            let _result = stream_handle.play_raw(sine.convert_samples());
            std::thread::sleep(std::time::Duration::from_millis(50));
        }
        else
        {
            let mut sine = SineWave::new(220.0);  
            let _result = stream_handle.play_raw(sine.convert_samples());
            std::thread::sleep(std::time::Duration::from_millis(150));
        }
    }


}

