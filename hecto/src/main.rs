#![warn(clippy::all, clippy::pedantic)]
mod editor;  // make new module, "editor" 
use editor::Editor; 
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};

fn main() {
    Editor::default().run(); 

}