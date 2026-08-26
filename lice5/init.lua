-- LiCe5 Compatibility Layer for Void IRC Client
-- Main initialization script
-- 
-- Usage: /load lice5/init.lua
-- Or add to config.lua: dofile("lice5/init.lua")

lice5 = {
    version = "5.0.0-void",
    loaded = {},
}

-- Helper: load a lice5 module
function lice5.load(name)
    local path = "lice5/" .. name .. ".lua"
    local f, err = loadfile(path)
    if f then
        f()
        lice5.loaded[name] = true
        void.echo("-!- LiCe5: loaded " .. name)
    else
        void.echo("-!- LiCe5 error loading " .. name .. ": " .. (err or "unknown"))
    end
end

-- Load all modules
lice5.load("autovoice")
lice5.load("anti_flood")
lice5.load("highlight")
lice5.load("ctcp")
lice5.load("away")
lice5.load("nickserv")
lice5.load("channel_protect")
lice5.load("statusbar")
lice5.load("keybinds")

void.echo("-!- LiCe5 v" .. lice5.version .. " loaded (" .. #lice5.loaded .. " modules)")
