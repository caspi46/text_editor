use std::io::{self, Read}; 
use crossterm::terminal::{enable_raw_mode, disable_raw_mode}; 


fn main() {
    enable_raw_mode().unwrap(); // if can't turn on Raw Mode, no point to continue and nothing further we can do 
    for b in io::stdin().bytes() {
        match b {
            Ok(b) => { // not occurs error 
                let c = b as char; 
                if c.is_control() { // if c is a control character : something we wouldn't want to print out (ex: Backspace)
                    println!("Binary: {0:08b} ASCII: {0:#03} \r", b); 
                } else {
                    println!("Binary: {0:08b} ASCII: {0:#03} Character: {1:#?}\r", b, c);
                }
                if c == 'q' { // 'q' : quit
                    disable_raw_mode().unwrap();
                    break; 
                }
            },
            Err(err) => println!("Error: {}", err), 
        }
    }
    disable_raw_mode().unwrap();
}