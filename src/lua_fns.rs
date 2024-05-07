use crate::box3;
use crate::core::{v3, I, ZERO3};
use crate::functions::*;
use crate::sdf::sd_box;
use glam::{Mat2, Vec2, Vec3};
use mlua::prelude::*;
use mlua::Function;

pub fn lua_functions(lua: &Lua) -> LuaResult<()> {
    let globals = lua.globals();
    let bx3: Function = lua
        .load(
            r#"
        function(x, y, z, a, b, c)
            local b = b or a
            local c = c or a
            return a + b
        end
"#,
        )
        .eval()?;
    let lua_bx3 =
        lua.create_function(|_, (x, y, z, a, b, c): (f32, f32, f32, f32, f32, f32)| {
            Ok(box3!(x, y, z, a, b, c))
        })?;
    globals.set("bx3", lua_bx3)?;
    Ok(())
}
