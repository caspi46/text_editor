#![warn(clippy::all, clippy::pedantic, clippy::print_stdout)]
mod editor;  // make new module, "editor" 
use editor::Editor; 
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};

fn main() {
    Editor::default().run(); 

}