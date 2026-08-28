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

-- Module info registry
lice5.module_info = {
    ignore = {desc = "Enhanced ignore with patterns, timeouts, /ON hooks", cmds = "/ig, /ignore"},
    gone = {desc = "Away/back system with random reasons from files", cmds = "/gone, /back, /autoaway"},
    kick = {desc = "Enhanced kick/kickban with random reasons", cmds = "/k, /kb, /rk"},
    mass = {desc = "Mass op/deop/voice/kick/ban commands", cmds = "/massop, /massdeop, /massvoice, /massdevoice, /masskick, /massban"},
    userlist = {desc = "Bot-style auto-op/voice with access levels", cmds = "/ul, /userlist"},
    alarm = {desc = "Timer/reminder system with named alarms", cmds = "/alarm"},
    reconnect = {desc = "Auto-reconnect with channel rejoin tracking", cmds = "/reconnect, /rejoin"},
    paste = {desc = "Multi-line paste mode", cmds = "/paste"},
    logman = {desc = "Per-channel log management", cmds = "/logman"},
    autovoice = {desc = "Auto-voice on join", cmds = "/autovoice"},
    anti_flood = {desc = "Anti-flood protection", cmds = "/antiflood"},
    highlight = {desc = "Nick/pattern highlight with colors", cmds = "/lice_highlight"},
    ctcp = {desc = "Enhanced CTCP replies (VERSION, PING, TIME, CLIENTINFO)", cmds = "(hooks)"},
    nickserv = {desc = "NickServ auto-identify + ghost + recover", cmds = "/ns, /nickserv"},
    channel_protect = {desc = "Anti-kick, anti-deop protection", cmds = "/protect"},
    invite = {desc = "Invite tracking, accept/reject", cmds = "/invlist"},
    dns = {desc = "DNS lookup command", cmds = "/dns"},
    signoff = {desc = "Random quit messages from quit.reasons", cmds = "/signoff"},
    wall = {desc = "Broadcast to channels", cmds = "/wall"},
    finger = {desc = "User info lookup", cmds = "/finger"},
    memo = {desc = "Offline memo system", cmds = "/memo"},
    note = {desc = "Quick notes and reminders", cmds = "/note"},
    party = {desc = "Party mode with disco colors and dance moves", cmds = "/party, /disco, /dance"},
    sensors = {desc = "Channel activity monitoring", cmds = "/sensors"},
    help = {desc = "Enhanced help with categories", cmds = "/lice_help"},
    banlist = {desc = "Ban list management", cmds = "/banlist"},
    exclist = {desc = "Exception list management", cmds = "/exclist"},
    invlist = {desc = "Invite exception list", cmds = "/invexlist"},
    joinlist = {desc = "Join tracking / clone detection", cmds = "/joinlist"},
    serverignore = {desc = "Server-level ignore (SILENCE)", cmds = "/silence"},
    play = {desc = "Log replay", cmds = "/play"},
    chanlog = {desc = "Per-channel logging setup", cmds = "/chanlog"},
    news = {desc = "News system", cmds = "/news"},
    update = {desc = "Update checker", cmds = "/update"},
    oops = {desc = "Quick fix last message", cmds = "/oops"},
    splitlist = {desc = "Netsplit tracking", cmds = "/splitlist"},
    show_list = {desc = "Unified list display", cmds = "/showlist"},
    remove_list = {desc = "Unified list removal", cmds = "/rmlist"},
    refriend = {desc = "Quick friend management", cmds = "/refriend"},
    rel = {desc = "Relationship tracking", cmds = "/rel"},
    noig = {desc = "No-ignore whitelist", cmds = "/noig"},
    pager = {desc = "In-client file pager", cmds = "/pager"},
    wget = {desc = "URL fetch", cmds = "/wget"},
    trans = {desc = "Translation helper", cmds = "/trans"},
    define = {desc = "Dictionary lookup", cmds = "/define"},
    sc = {desc = "Screen/tmux integration", cmds = "/sc"},
    mk = {desc = "File creation helper", cmds = "/mk"},
    mme = {desc = "Mass message to targets", cmds = "/mme"},
    msay = {desc = "Multi-target say", cmds = "/msay"},
    mtog = {desc = "Message toggle", cmds = "/mtog"},
    ctog = {desc = "Channel feature toggle", cmds = "/ctog"},
    dtog = {desc = "Display feature toggle", cmds = "/dtog"},
    wtog = {desc = "Window feature toggle", cmds = "/wtog"},
    tog = {desc = "Generic toggle", cmds = "/tog"},
    dom = {desc = "Domain operations", cmds = "/dom"},
    dump = {desc = "Debug dump", cmds = "/dump"},
    ul_save = {desc = "Userlist save/load", cmds = "/ulsave"},
    ulw = {desc = "Userlist window commands", cmds = "/ulw_*"},
    tab_comp = {desc = "Tab completion enhancement", cmds = "/tabcomp"},
    bword = {desc = "Word manipulation utilities", cmds = "/bword"},
    binds = {desc = "Key binding management", cmds = "/binds"},
    defaults = {desc = "Default settings display", cmds = "/defaults"},
    imail = {desc = "Internal mail system", cmds = "/imail"},
    floodlist = {desc = "Flood protection exceptions", cmds = "/floodlist"},
    looplist = {desc = "Loop through lists", cmds = "/looplist"},
    pic = {desc = "ASCII art pictures", cmds = "/pic"},
    ppl = {desc = "People tracking", cmds = "/ppl"},
    chanst = {desc = "Channel status", cmds = "/chanst"},
    cwho = {desc = "Channel WHO", cmds = "/cwho"},
    et = {desc = "Enhanced topic", cmds = "/et"},
    db = {desc = "Key-value database", cmds = "/db"},
    fkeys = {desc = "Function key bindings", cmds = "/fkey"},
    boot = {desc = "Boot sequence", cmds = "/boot"},
    stubs = {desc = "DCC stubs (adcc, dcclist, rdcc, redcc)", cmds = "/adcc, /dcclist, /rdcc, /redcc"},
}

-- /module command — show module info
void.register_command("MODULE", "lice5_cmd_module")
function lice5_cmd_module(args)
    if #args == 0 then
        void.echo("-!- Loaded modules:")
        for name, _ in pairs(lice5.loaded) do
            local info = lice5.module_info[name]
            if info then
                void.echo("  " .. name .. " — " .. info.desc)
            else
                void.echo("  " .. name)
            end
        end
        void.echo("-!- Use /module <name> for details")
        return
    end
    local name = args[1]:lower()
    local info = lice5.module_info[name]
    if info then
        void.echo("-!- Module: " .. name)
        void.echo("  Description: " .. info.desc)
        void.echo("  Commands: " .. info.cmds)
    else
        void.echo("-!- Unknown module: " .. name)
    end
end
