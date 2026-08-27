-- LiCe5 Compatibility Layer for Void IRC Client
-- Main initialization script
-- 
-- Usage: /load modules/init.lua
-- Or add to config.lua: dofile("modules/init.lua")

lice5 = {
    version = "5.2.0-void",
    loaded = {},
}

-- Helper: load a module
function lice5.load(name)
    local path = "modules/" .. name .. ".lua"
    local f, err = loadfile(path)
    if f then
        f()
        lice5.loaded[name] = true
        void.echo("-!- Module loaded: " .. name)
    else
        void.echo("-!- Module error: " .. name .. ": " .. (err or "unknown"))
    end
end

-- Load themes
lice5.load("themes/init")
lice5.load("themes/catppuccin")
lice5.load("themes/catppuccin_latte")
lice5.load("themes/dracula")
lice5.load("themes/nord")
lice5.load("themes/gruvbox")
lice5.load("themes/gruvbox_light")
lice5.load("themes/solarized")
lice5.load("themes/solarized_light")
lice5.load("themes/tokyonight")
lice5.load("themes/matrix")
lice5.load("themes/cyberpunk")
lice5.load("themes/monokai")
lice5.load("themes/onedark")
lice5.load("themes/rosepine")
lice5.load("themes/irssi")
lice5.load("themes/bitchx")

-- Load all modules (order matters — dependencies first)
lice5.load("ignore")          -- Enhanced ignore system
lice5.load("gone")            -- Away/back with random reasons + auto-away
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
lice5.load("nickserv")        -- NickServ auto-identify + ghost
lice5.load("channel_protect") -- Anti-kick, anti-deop
lice5.load("invite")          -- Invite management
lice5.load("dns")             -- DNS lookup
lice5.load("signoff")         -- Random quit messages
lice5.load("wall")            -- Broadcast to channels
lice5.load("finger")          -- User info lookup
lice5.load("memo")            -- Offline memo system
lice5.load("note")            -- Quick notes
lice5.load("party")           -- Party mode
lice5.load("sensors")         -- Channel activity monitoring
lice5.load("help")            -- Enhanced help system
lice5.load("banlist")         -- Ban list management
lice5.load("exclist")         -- Exception list management
lice5.load("invlist")         -- Invite exception list management
lice5.load("joinlist")        -- Join tracking / clone detection
lice5.load("serverignore")    -- Server-level ignore (SILENCE)
lice5.load("play")            -- Log replay
lice5.load("chanlog")         -- Per-channel logging setup
lice5.load("news")            -- News system
lice5.load("update")          -- Update checker
lice5.load("oops")            -- Quick fix last message
lice5.load("splitlist")       -- Netsplit tracking
lice5.load("show_list")       -- Unified list display
lice5.load("remove_list")     -- Unified list removal
lice5.load("refriend")        -- Quick friend management
lice5.load("rel")             -- Relationship tracking
lice5.load("noig")            -- No-ignore whitelist
lice5.load("pager")           -- In-client file pager
lice5.load("wget")            -- URL fetch
lice5.load("trans")           -- Translation helper
lice5.load("define")          -- Dictionary lookup
lice5.load("sc")              -- Screen/tmux integration
lice5.load("mk")              -- File creation helper
lice5.load("mme")             -- Mass message to targets
lice5.load("msay")            -- Multi-target say
lice5.load("mtog")            -- Message toggle
lice5.load("ctog")            -- Channel feature toggle
lice5.load("dtog")            -- Display feature toggle
lice5.load("wtog")            -- Window feature toggle
lice5.load("tog")             -- Generic toggle
lice5.load("dom")             -- Domain operations
lice5.load("dump")            -- Debug dump
lice5.load("ul_save")         -- Userlist save/load
lice5.load("ulw")             -- Userlist window commands
lice5.load("tab_comp")        -- Tab completion enhancement
lice5.load("bword")           -- Word manipulation
lice5.load("binds")           -- Key binding management
lice5.load("defaults")        -- Default settings
lice5.load("imail")           -- Internal mail system
lice5.load("floodlist")       -- Flood protection exceptions
lice5.load("looplist")        -- Loop through lists
lice5.load("pic")             -- ASCII art pictures
lice5.load("ppl")             -- People tracking
lice5.load("chanst")          -- Channel status
lice5.load("cwho")            -- Channel WHO
lice5.load("et")              -- Enhanced topic
lice5.load("db")              -- Key-value database
lice5.load("fkeys")           -- Function key bindings
lice5.load("boot")            -- Boot sequence
lice5.load("stubs")           -- Stub functions

-- Count loaded modules
local count = 0
for _ in pairs(lice5.loaded) do count = count + 1 end
void.echo("-!- LiCe5 v" .. lice5.version .. " loaded (" .. count .. " modules)")
