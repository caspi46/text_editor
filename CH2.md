Chapter 2: Entering Raw Mode 

use std::io{self, Read}; 

fn main() {
    for b in io::stdin().bytes() {
        let c = b.unwrap() as char; 
        println!("{}", c); 
    }
}

canonical mode / cooked mode: 
- Keyboard input is only setnt to the program when the user presses Enter. 

CTRL-D -> io::stdin().bytes() 
- the file it's supposed to read has ended 

CTRL-C : immediately terminates the program

Entering Raw Mode:
- Raw Mode:
    - crossterm: 
        - get the attribubtes 

- Typewriter concepts:
    - the bell 
    - the turning of the drum 
    - the moving of the carriage 

- Still in computer!

- Error Handling:
    - Result: two variants Ok and Err 