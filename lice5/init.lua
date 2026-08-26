-- LiCe5 Compatibility Layer for Void IRC Client
-- Main initialization script
-- 
-- Usage: /load lice5/init.lua
-- Or add to config.lua: dofile("lice5/init.lua")

lice5 = {
    version = "5.1.0-void",
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

-- Load all modules (order matters — dependencies first)
lice5.load("ignore")          -- Enhanced ignore system
lice5.load("gone")            -- Away/back with random reasons
lice5.load("kick")            -- Enhanced kick/kickban with random reasons
lice5.load("mass")            -- Mass op/deop/voice/kick/ban
lice5.load("userlist")        -- Bot-style auto-op/voice
lice5.load("alarm")           -- Timer/reminder system
lice5.load("reconnect")       -- Auto-reconnect with channel rejoin
lice5.load("paste")           -- Multi-line paste mode (epic6)
lice5.load("logman")          -- Per-channel log management (epic6)
lice5.load("autovoice")       -- Auto-voice on join
lice5.load("anti_flood")      -- Anti-flood protection
lice5.load("highlight")       -- Nick/pattern highlight
lice5.load("ctcp")            -- Enhanced CTCP replies
lice5.load("away")            -- Away system with auto-away
lice5.load("nickserv")        -- NickServ auto-identify + ghost
lice5.load("channel_protect") -- Anti-kick, anti-deop
lice5.load("statusbar")       -- Status bar enhancements
lice5.load("keybinds")        -- Custom key bindings

-- Count loaded modules
local count = 0
for _ in pairs(lice5.loaded) do count = count + 1 end
void.echo("-!- LiCe5 v" .. lice5.version .. " loaded (" .. count .. " modules)")
