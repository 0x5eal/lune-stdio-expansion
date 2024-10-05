use lune_utils::TableBuilder;
use mlua::prelude::*;

use erase::EraseKind;
use screen::ScreenMode;

mod erase;
mod screen;

pub fn create(lua: &Lua) -> LuaResult<LuaTable> {
    TableBuilder::new(lua)?
        .with_metatable(
            TableBuilder::new(lua)?
                .with_function(
                    LuaMetaMethod::Index.name(),
                    |lua: &Lua, (_, kind): (LuaTable, EraseKind)| {
                        lua.create_function(move |lua: &Lua, (): ()| {
                            lua.create_string(kind.ansi_escape_sequence())
                        })
                    },
                )?
                .build()?,
        )?
        .with_function("setMode", |_: &Lua, mode: ScreenMode| {
            mode.ansi_escape_sequence()
        })?
        .build()
}
