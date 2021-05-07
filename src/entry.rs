use std::io::{Write, stdout};

use crossterm::{cursor::{MoveDown, MoveToPreviousLine, RestorePosition, SavePosition}, execute, style::{Attribute, Color, Print, ResetColor, SetAttribute, SetBackgroundColor, SetForegroundColor}};

#[derive(Debug)]
pub enum Error {
    Display,
    Logic
}

pub type Result<T> = std::result::Result<T, Error>;

pub struct Entry {
    pub name: String,
    pub color: Color,
}

impl Entry {
    pub fn new(name: String, color: Color) -> Entry {
        return Entry {
            name: format!("{}\n", &name),
            color,
        };
    }

    pub fn print(&self) -> Result<()> {
        execute!(
            stdout(),
            SetForegroundColor(self.color),
            SetBackgroundColor(Color::Black),
            Print(" "),
            Print(self.name.clone()),
            ResetColor,
        ).map_err(|_| Error::Display)?;
        Ok(())
    }
}

pub fn print_entries(entries: &Vec<Entry>) -> Result<()> {

    for entry in entries {
        &entry.print()?;
    }
    Ok(())
}

pub fn print_selected_entry(entry: &Entry, i: usize, total: usize) -> Result<()> {
    execute!(
        stdout(),
        SavePosition,
        MoveToPreviousLine(total as u16),
        MoveDown((i % total) as u16),
        SetForegroundColor(entry.color),
        Print(">"),
        SetAttribute(Attribute::Underlined),
        Print(&entry.name),
        RestorePosition,
    ).map_err(|_| Error::Display)?;
    Ok(())
}

pub fn print_clear_entry(entry: &Entry, i: usize, total: usize) -> Result<()> {
    execute!(
        stdout(),
        SavePosition,
        MoveToPreviousLine(total as u16),
        MoveDown((i % total) as u16),
        SetForegroundColor(entry.color),
        Print(" "),
        SetAttribute(Attribute::NoUnderline),
        Print(&entry.name),
        RestorePosition,
    ).map_err(|_| Error::Display)?;
    Ok(())
}
