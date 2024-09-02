use mlua::prelude::*;

use crate::define_ansi_op;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CursorKind {
    /// Moves the cursor to `(0, 0)`
    Home,
    /// Moves the cursor to the coordinates `(x, y)`
    Move(usize, usize),
    /// Moves the cursor up by a specified number of lines
    Up(usize),
    /// Moves the cursor down by a specified number of lines
    Down(usize),
    /// Moves the cursor left by a specified number of lines
    Left(usize),
    /// Moves the cursor right by a specified number of lines
    Right(usize),
    /// Moves the cursor to the specified column
    Column(usize),
    /// Saves the cursor position
    Save,
    /// Restores the cursor position
    Restore,
}

impl CursorKind {
    pub fn home() -> Self {
        Self::Home
    }

    pub fn r#move(x: usize, y: usize) -> Self {
        Self::Move(x, y)
    }

    pub fn up(n: usize) -> Self {
        Self::Up(n)
    }

    pub fn down(n: usize) -> Self {
        Self::Down(n)
    }

    pub fn left(n: usize) -> Self {
        Self::Left(n)
    }

    pub fn right(n: usize) -> Self {
        Self::Right(n)
    }

    pub fn column(n: usize) -> Self {
        Self::Column(n)
    }

    pub fn save() -> Self {
        Self::Save
    }

    pub fn restore() -> Self {
        Self::Restore
    }

    pub fn ansi_escape_sequence(self) -> String {
        match self {
            Self::Home => "\x1b[H".to_string(),
            Self::Move(x, y) => format!("\x1b[{{{x}}};{{{y}}}"),
            Self::Up(n) => format!("\x1b[{n}A"),
            Self::Down(n) => format!("\x1b[{n}B"),
            Self::Left(n) => format!("\x1b[{n}D"),
            Self::Right(n) => format!("\x1b[{n}C"),
            Self::Column(n) => format!("\x1b[{n}G"),

            // FIXME: These two might not be correct, actually confirm them
            Self::Save => "\x1b 7".to_string(),
            Self::Restore => "\x1b 8".to_string(),
        }
    }
}

pub struct Cursor;

impl LuaUserData for Cursor {
    fn add_methods<'lua, M: LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        define_ansi_op!(methods, CursorKind::home, ());
        define_ansi_op!(methods, CursorKind::r#move, (x => usize, y => usize));
        define_ansi_op!(methods, CursorKind::up, (n => usize));
        define_ansi_op!(methods, CursorKind::down, (n => usize));
        define_ansi_op!(methods, CursorKind::left, (n => usize));
        define_ansi_op!(methods, CursorKind::right, (n => usize));
        define_ansi_op!(methods, CursorKind::column, (n => usize));
        define_ansi_op!(methods, CursorKind::save, ());
        define_ansi_op!(methods, CursorKind::restore, ());
    }
}
