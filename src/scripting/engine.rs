use mlua::{Lua, Result as LuaResult};

/// Inicjalizacja silnika Lua
pub fn init_lua() -> LuaResult<Lua> {
    let lua = Lua::new();
    lua.globals().set("client_name", "void")?;
    lua.globals().set("client_version", "0.3.0")?;
    Ok(lua)
}

/// Załaduj config.lua i skrypty (wywoływane PO register_api)
pub fn load_scripts(lua: &Lua) {
    let home = std::env::var("HOME").unwrap_or_default();
    let void_dir = std::path::PathBuf::from(&home).join(".void");

    // Wczytaj config.lua z CWD lub ~/.void/
    // Config.lua zawiera routing modułów (load_module = "lice")
    if let Ok(script) = std::fs::read_to_string("config.lua") {
        if let Err(e) = lua.load(&script).exec() {
            eprintln!("Error loading config.lua: {}", e);
        }
    } else if let Ok(script) = std::fs::read_to_string(void_dir.join("config.lua")) {
        if let Err(e) = lua.load(&script).exec() {
            eprintln!("Error loading ~/.void/config.lua: {}", e);
        }
    }

    // Wczytaj skrypty z katalogu scripts/ lub ~/.void/scripts/
    load_scripts_dir(lua, "scripts");
    load_scripts_dir(lua, &void_dir.join("scripts").to_string_lossy());

    // Moduły są ładowane przez config.lua (routing: load_module = "lice")
    // Nie ładujemy automatycznie z modules/ — to robi config.lua
}

/// Wczytaj wszystkie .lua z katalogu
fn load_scripts_dir(lua: &Lua, dir: &str) {
    if let Ok(entries) = std::fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().map_or(false, |e| e == "lua") {
                if let Ok(script) = std::fs::read_to_string(&path) {
                    if let Err(e) = lua.load(&script).exec() {
                        eprintln!("Error loading {}: {}", path.display(), e);
                    }
                }
            }
        }
    }
}

/// Pobierz string z Lua config
pub fn get_config_string(lua: &Lua, table: &str, key: &str) -> Option<String> {
    lua.globals()
        .get::<mlua::Table>(table)
        .ok()
        .and_then(|t| t.get::<String>(key).ok())
}

/// Pobierz listę stringów z Lua config
pub fn get_config_vec(lua: &Lua, table: &str, key: &str) -> Option<Vec<String>> {
    lua.globals()
        .get::<mlua::Table>(table)
        .ok()
        .and_then(|t| t.get::<Vec<String>>(key).ok())
}

/// Wywołaj funkcję Lua i zwróć wynik jako String
pub fn call_lua_fn(lua: &Lua, fn_name: &str) -> Option<String> {
    lua.globals()
        .get::<mlua::Function>(fn_name)
        .ok()
        .and_then(|f| f.call::<String>(()).ok())
}
