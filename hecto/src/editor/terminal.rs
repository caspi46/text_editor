use core::fmt::Display;
use crossterm::cursor::{MoveTo, Show, Hide}; 
use crossterm::style::Print;
use crossterm::{Command, queue}; 
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType};
use std::io::{stdout, Error, Write};
use crossterm::terminal::size; 



#[derive(Copy, Clone)]
pub struct Size {
    pub height : usize,
    pub width : usize,
}

#[derive(Copy, Clone, Default)]
pub struct Position {
    pub col : usize,
    pub row : usize,
}

pub struct Terminal;


impl Terminal {
    pub fn terminate() -> Result<(), std::io::Error> {
        Self::execute()?;
        disable_raw_mode()?;
        Ok(())
    }
    pub fn initialize() -> Result<(), std::io::Error> {
        enable_raw_mode()?;
        Self::clear_screen()?; 
        Self::execute()?;
        Ok(())
    }

    pub fn clear_screen() -> Result<(), std::io::Error> {
        Self::queue_command(Clear(ClearType::All))?; 
        Ok(())
    }

    pub fn clear_line() -> Result<(), Error> {
        Self::queue_command(Clear(ClearType::CurrentLine))
    }

    pub fn move_caret_to(position: Position) -> Result<(), std::io::Error> {
        #[allow(clippy::as_conversions, clippy::cast_possible_truncation)]
        Self::queue_command(MoveTo(position.col as u16, position.row as u16))?; 
        Ok(())
    }

    pub fn size() -> Result<Size, Error> {
        let (width_u16, height_u16) = size()?;
        #[allow(clippy::as_conversions)] 
        let height = height_u16 as usize; 
        let width = width_u16 as usize;
        Ok(Size {height, width})
    }

    pub fn print<T: Display>(s : T) -> Result<(), Error> {
        Self::queue_command(Print(s))?;
        Ok(())
    }

    pub fn show_caret()-> Result<(), Error> {
        Self::queue_command(Show)?;
        Ok(())
    }

    pub fn hide_caret() -> Result<(), Error> {
        Self::queue_command(Hide)?;
        Ok(())
    }

    pub fn execute() -> Result<(), Error> {
        stdout().flush()?;
        Ok(())
    }
    
    pub fn queue_command<T:Command>(command: T) -> Result<(), Error> {
        queue!(stdout(), command)?;
        Ok(())
    }
}



