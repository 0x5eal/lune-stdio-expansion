use std::sync::Arc;

use mlua::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScreenKind {
    /// Graphics rendering mode.
    Graphics,
    /// Text rendering mode.
    Text,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScreenMode {
    /// Monochrome screen mode.
    Monochrome {
        /// The type of screen rendering.
        screen_kind: ScreenKind,
        /// The dimensions of the screen.
        dims: (usize, usize),
    },

    Color {
        /// The type of screen rendering.
        screen_kind: ScreenKind,
        /// The dimensions of the screen.
        dims: (usize, usize),
        /// The bit depth of the color mode, calculated by `log2(n)`,
        /// where `n` is the number of colors in the graphics mode.
        bit_depth: Option<usize>,
    },

    /// Enables line wrapping.
    EnableWrapping,
}

impl ScreenMode {
    #[rustfmt::skip]
    pub const ALL: [Self; 15] = [
        Self::Monochrome { screen_kind: ScreenKind::Text, dims: (40, 25), },
        Self::Color { screen_kind: ScreenKind::Text, dims: (40, 25), bit_depth: None, },
        Self::Monochrome { screen_kind: ScreenKind::Text, dims: (80, 25), },
        Self::Color { screen_kind: ScreenKind::Text, dims: (80, 25), bit_depth: None, },
        Self::Color { screen_kind: ScreenKind::Graphics, dims: (320, 200), bit_depth: Some(2), },
        Self::Monochrome { screen_kind: ScreenKind::Graphics, dims: (320, 200), },
        Self::Monochrome { screen_kind: ScreenKind::Graphics, dims: (640, 200), },
        Self::EnableWrapping,
        Self::Color { screen_kind: ScreenKind::Graphics, dims: (320, 200), bit_depth: None, },
        Self::Color { screen_kind: ScreenKind::Graphics, dims: (640, 200), bit_depth: Some(4), },
        Self::Monochrome { screen_kind: ScreenKind::Graphics, dims: (640, 350) },
        Self::Color { screen_kind: ScreenKind::Graphics, dims: (640, 350), bit_depth: Some(4) },
        Self::Monochrome { screen_kind: ScreenKind::Graphics, dims: (640, 480) },
        Self::Color { screen_kind: ScreenKind::Graphics, dims: (640, 480), bit_depth: Some(4) },
        Self::Color { screen_kind: ScreenKind::Graphics, dims: (300, 200), bit_depth: Some(8) },
    ];

    pub fn name(self) -> String {
        format!("{self:?}")
    }

    #[rustfmt::skip]
    pub fn ansi_escape_sequence(self) -> LuaResult<&'static str> {
        match self {
            Self::Monochrome { screen_kind: ScreenKind::Text, dims: (40, 25) } => Ok("\x1b[=0h"),
            Self::Color { screen_kind: ScreenKind::Text, dims: (40, 25), bit_depth: None } => Ok("\x1b[=1h"),
            Self::Monochrome { screen_kind: ScreenKind::Text, dims: (80, 25) } => Ok("\x1b[=2h"),
            Self::Color { screen_kind: ScreenKind::Text, dims: (80, 25), bit_depth: None } => Ok("\x1b[=3h"),
            Self::Color { screen_kind: ScreenKind::Graphics, dims: (320, 200), bit_depth: Some(2) } => Ok("\x1b[=4h"),
            Self::Monochrome { screen_kind: ScreenKind::Graphics, dims: (320, 200) } => Ok("\x1b[=5h"),
            Self::Monochrome { screen_kind: ScreenKind::Graphics, dims: (640, 200) } => Ok("\x1b[=6h"),
            Self::EnableWrapping => Ok("\x1b[=7h"),
            Self::Color { screen_kind: ScreenKind::Graphics, dims: (320, 200), bit_depth: None } => Ok("\x1b[=13h"),
            Self::Color { screen_kind: ScreenKind::Graphics, dims: (640, 200), bit_depth: Some(4) } => Ok("\x1b[=14h"),
            Self::Monochrome { screen_kind: ScreenKind::Graphics, dims: (640, 350) } => Ok("\x1b[=15h"),
            Self::Color { screen_kind: ScreenKind::Graphics, dims: (640, 350), bit_depth: Some(4) } => Ok("\x1b[=16h"),
            Self::Monochrome { screen_kind: ScreenKind::Graphics, dims: (640, 480) } => Ok("\x1b[=17h"),
            Self::Color { screen_kind: ScreenKind::Graphics, dims: (640, 480), bit_depth: Some(4) } => Ok("\x1b[=18h"),
            Self::Color { screen_kind: ScreenKind::Graphics, dims: (300, 200), bit_depth: Some(8) } => Ok("\x1b[=19h"),
            _ => Err(LuaError::runtime(format!(
                "Invalid mode configuration, valid configurations are:\n- {}",
                Self::ALL
                    .iter()
                    .map(|kind| kind.name())
                    .collect::<Vec<_>>()
                    .join("\n- ")
            ))),
        }
    }
}

impl<'lua> FromLuaMulti<'lua> for ScreenMode {
    fn from_lua_multi(values: LuaMultiValue<'lua>, _: &'lua Lua) -> LuaResult<Self> {
        // FIXME: Does this start a 1 or 0?
        let get_value = |index: usize, name: &str| -> LuaResult<&LuaValue> {
            values.get(index).ok_or_else(|| LuaError::BadArgument {
                to: None,
                pos: index + 1,
                name: Some(name.to_string()),
                cause: Arc::new(LuaError::runtime(format!(
                    "Expected {name} of mode, got nil"
                ))),
            })
        };

        let lua_color_kind = get_value(0, "ColorKind")?.as_str().ok_or_else(|| {
            LuaError::FromLuaConversionError {
                from: "String",
                to: "ColorKind",
                message: None,
            }
        })?;

        let lua_screen_kind = get_value(1, "ScreenKind")?.as_str().ok_or_else(|| {
            LuaError::FromLuaConversionError {
                from: "String",
                to: "ScreenKind",
                message: None,
            }
        })?;

        let dims_tab = get_value(2, "Dimensions")?.as_table().ok_or_else(|| {
            LuaError::FromLuaConversionError {
                from: "Table",
                to: "Dimensions",
                message: None,
            }
        })?;

        let dims = (
            dims_tab.get::<_, usize>("width")?,
            dims_tab.get::<_, usize>("height")?,
        );

        let bit_depth = values.get(3).and_then(LuaValue::as_usize);

        let screen_kind = match lua_screen_kind.to_lowercase().as_str() {
            "text" => ScreenKind::Text,
            "graphics" => ScreenKind::Graphics,
            _ => {
                return Err(LuaError::FromLuaConversionError {
                    from: "String",
                    to: "ScreenKind",
                    message: None,
                })
            }
        };

        match lua_color_kind.to_lowercase().as_str() {
            "monochrome" => Ok(Self::Monochrome { screen_kind, dims }),
            "color" => Ok(Self::Color {
                screen_kind,
                dims,
                bit_depth,
            }),
            _ => Err(LuaError::FromLuaConversionError {
                from: "String",
                to: "ScreenMode",
                message: None,
            }),
        }
    }
}
