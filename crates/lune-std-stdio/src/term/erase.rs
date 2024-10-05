use std::str::FromStr;

use mlua::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EraseKind {
    /// Clears the entire screen.
    Screen,
    /// Clears from the cursor to the end of the screen.
    ScreenEnd,
    /// Clears from the cursor to the start of the screen.
    ScreenStart,
    /// Clears the current line.
    Line,
    /// Clears from the cursor to the end of the line.
    LineEnd,
    /// Clears from the cursor to the start of the line.
    LineStart,
    /// Clears the saved lines.
    Saved,
}

impl EraseKind {
    pub const ALL: [Self; 7] = [
        Self::Screen,
        Self::ScreenEnd,
        Self::ScreenStart,
        Self::Line,
        Self::LineStart,
        Self::LineEnd,
        Self::Saved,
    ];

    pub fn name(self) -> &'static str {
        match self {
            Self::Screen => "clear",
            Self::ScreenEnd => "clearEnd",
            Self::ScreenStart => "clearStart",
            Self::Line => "clearLine",
            Self::LineEnd => "clearLineEnd",
            Self::LineStart => "clearLineStart",
            Self::Saved => "clearSaved",
        }
    }

    pub fn ansi_escape_sequence(self) -> &'static str {
        match self {
            Self::Screen => "\x1b[2J",
            Self::ScreenEnd => "\x1b[0J",
            Self::ScreenStart => "\x1b[1J",
            Self::Line => "\x1b[2K",
            Self::LineEnd => "\x1b[0K",
            Self::LineStart => "\x1b[1K",
            Self::Saved => "\x1b[3J",
        }
    }
}

impl FromStr for EraseKind {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "clear" => Ok(Self::Screen),
            "clearEnd" => Ok(Self::ScreenEnd),
            "clearStart" => Ok(Self::ScreenStart),
            "clearLine" => Ok(Self::Line),
            "clearLineEnd" => Ok(Self::LineEnd),
            "clearLineStart" => Ok(Self::LineStart),
            "clearSaved" => Ok(Self::Saved),
            _ => Err(()),
        }
    }
}

impl<'lua> FromLua<'lua> for EraseKind {
    fn from_lua(value: LuaValue<'lua>, _: &'lua Lua) -> LuaResult<Self> {
        if let LuaValue::String(s) = value {
            let str = s.to_str()?;

            str.parse().map_err(|(): ()| {
                LuaError::runtime(format!(
                    "Method {str} not found on Terminal, valid methods are {}",
                    Self::ALL
                        .iter()
                        .map(|kind| kind.name())
                        .collect::<Vec<_>>()
                        .join(", ")
                ))
            })
        } else {
            Err(LuaError::FromLuaConversionError {
                from: value.type_name(),
                to: "EraseKind",
                message: None,
            })
        }
    }
}
