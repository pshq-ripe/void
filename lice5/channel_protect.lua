-- LiCe5: Channel Protection
-- Anti-kick, anti-ban, and channel guard features

lice5.protect = {
    channels = {},      -- channels with protection enabled
    anti_deop = true,   -- auto-reop if deoped
    anti_kick = true,   -- auto-rejoin if kicked
    anti_ban = true,    -- auto-unban after timeout
    ban_timeout = 300,  -- seconds before auto-unban
}

function lice5.protect.enable(channel)
    lice5.protect.channels[channel:upper()] = true
    void.echo("-!- Channel protection enabled for " .. channel)
end

function lice5.protect.disable(channel)
    lice5.protect.channels[channel:upper()] = nil
    void.echo("-!- Channel protection disabled for " .. channel)
end

-- Hook: on KICK, auto-rejoin if protected
void.on("KICK", "lice5_on_kick_protect")
function lice5_on_kick_protect(args)
    local channel = args[1] or ""
    local kicked = args[2] or ""
    
    if kicked ~= void.nick() then return end
    if not lice5.protect.channels[channel:upper()] then return end
    if not lice5.protect.anti_kick then return end
    
    void.echo("-!- Protected: rejoining " .. channel)
    void.timer(1, "lice5_protect_rejoin_" .. channel)
end

-- Hook: on MODE -o, auto-reop if protected
void.on("MODE", "lice5_on_mode_protect")
function lice5_on_mode_protect(args)
    local channel = args[1] or ""
    local modes = args[2] or ""
    local target = args[3] or ""
    
    if not lice5.protect.channels[channel:upper()] then return end
    if not lice5.protect.anti_deop then return end
    
    -- Check if we were deoped
    if modes:find("-o") and target == void.nick() then
        void.echo("-!- Protected: deoped in " .. channel .. " (anti-deop active)")
        -- Try to reop via NickServ or channel ops
    end
end

-- Command: /protect [#channel]
void.register_command("PROTECT", "lice5_cmd_protect")
function lice5_cmd_protect(args)
    local channel = args[1] or void.channel()
    if channel == "" then
        void.echo("-!- Usage: /protect [#channel]")
        return
    end
    
    if lice5.protect.channels[channel:upper()] then
        lice5.protect.disable(channel)
    else
        lice5.protect.enable(channel)
    end
end
