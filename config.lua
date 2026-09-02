-- ==============================================================================
-- VOID IRC CLIENT — Configuration
-- ==============================================================================

-- 1. GŁÓWNA KONFIGURACJA
config = {
    nickname = "void_" .. tostring(math.random(100, 999)),
    altnick = "void_" .. tostring(math.random(1000, 9999)),
    server = "irc.spadhausen.com",
    port = 6697,
    tls = true,
    sasl = "",
    realname = "Void IRC Client",
    channels = {},
}

-- 2. WYBÓR AKTYWNEGO MODUŁU (zmień na inny folder, gdy zajdzie potrzeba)
-- Wpisz "lice" aby załadować modules/lice/
-- Wpisz "dupa" aby załadować modules/dupa/
-- Wpisz "" aby nie ładować żadnego modułu
load_module = "lice"

-- 3. SILNIK ROUTINGU ŚCIEŻEK (obsługa podfolderów modułów)
local home = os.getenv("HOME") or os.getenv("USERPROFILE")

-- Szukaj modułów w ~/.void/modules/ lub ./modules/
local base_dir = nil
local candidates = {
    home .. "/.void/modules/" .. load_module .. "/",
    "modules/" .. load_module .. "/",
}
for _, dir in ipairs(candidates) do
    local f = io.open(dir .. "init.lua", "r")
    if f then
        f:close()
        base_dir = dir
        break
    end
end

if not base_dir then
    base_dir = home .. "/.void/modules/" .. load_module .. "/"
end

-- Funkcja pomocnicza przekierowująca "modules/" do wybranego load_module
local function route_to_module(path)
    if path:sub(1, 8) == "modules/" then
        return base_dir .. path:sub(9)
    end
    return path
end

-- Nadpisanie globalnej funkcji dofile
local original_dofile = dofile
dofile = function(path)
    return original_dofile(route_to_module(path))
end

-- Nadpisanie globalnej funkcji loadfile
local original_loadfile = loadfile
loadfile = function(path)
    return original_loadfile(route_to_module(path))
end

-- 4. ŁADOWANIE MODUŁU (jeśli wybrany)
if load_module and load_module ~= "" then
    local init_path = base_dir .. "init.lua"
    local f = io.open(init_path, "r")
    if f then
        f:close()
        dofile(init_path)
    else
        void.echo("-!- Module not found: " .. load_module .. " (" .. init_path .. ")")
    end
end
