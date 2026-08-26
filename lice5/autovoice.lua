-- LiCe5: Auto-Voice on Join
-- Automatically gives voice to users when they join a channel

lice5.autovoice = {
    channels = {},  -- channels where auto-voice is enabled
}

function lice5.autovoice.enable(channel)
    lice5.autovoice.channels[channel:upper()] = true
    void.echo("-!- Auto-voice enabled for " .. channel)
end

function lice5.autovoice.disable(channel)
    lice5.autovoice.channels[channel:upper()] = nil
    void.echo("-!- Auto-voice disabled for " .. channel)
end

-- Hook: on JOIN, auto-voice if enabled for that channel
void.on("JOIN", "lice5_on_join_voice")
function lice5_on_join_voice(args)
    local nick = args[1] or ""
    local channel = args[2] or ""
    
    -- Don't voice ourselves
    if nick == void.nick() then return end
    
    if lice5.autovoice.channels[channel:upper()] then
        void.voice(channel, nick)
    end
end

-- Command: /autovoice [#channel]
void.register_command("AUTOVOICE", "lice5_cmd_autovoice")
function lice5_cmd_autovoice(args)
    local channel = args[1] or void.channel()
    if channel == "" then
        void.echo("-!- Usage: /autovoice [#channel]")
        return
    end
    if lice5.autovoice.channels[channel:upper()] then
        lice5.autovoice.disable(channel)
    else
        lice5.autovoice.enable(channel)
    end
end
