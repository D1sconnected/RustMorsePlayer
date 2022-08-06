use text_io::read;
use morse::encode;

fn main() {

// Get string from user 
let line: String = read!("{}\n");
println!("Your input: {}", line);

// Print String by words
for word in line.split_whitespace()
{
    let morse_code: String = encode::encode(word).unwrap();
    println!("{}", morse_code);
}

}
