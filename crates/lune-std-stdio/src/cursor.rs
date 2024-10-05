use mlua::prelude::*;

// TODO: ShowCursor and HideCursor

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

macro_rules! define_cursor_op {
($methods:ident,$name:ident,($($arg:ident => $type:ty),*)) => {
        $methods.add_function(stringify!($name), |lua: &Lua, ($($arg,)*): ($($type,)*)| {
            lua.create_string(CursorKind::$name($($arg,)*).ansi_escape_sequence())
        })
    };
}

pub struct Cursor;
impl LuaUserData for Cursor {
    fn add_methods<'lua, M: LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        define_cursor_op!(methods, home, ());
        define_cursor_op!(methods, r#move, (x => usize, y => usize));
        define_cursor_op!(methods, up, (n => usize));
        define_cursor_op!(methods, down, (n => usize));
        define_cursor_op!(methods, left, (n => usize));
        define_cursor_op!(methods, right, (n => usize));
        define_cursor_op!(methods, column, (n => usize));
        define_cursor_op!(methods, save, ());
        define_cursor_op!(methods, restore, ());
    }
}
