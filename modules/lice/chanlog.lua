-- LiCe5: Channel Log Setup
-- Configure per-channel logging

lice5.chanlog = {
    channels = {},
}

function lice5.chanlog.enable(channel, path)
    lice5.chanlog.channels[channel:upper()] = path or ("logs/" .. channel .. ".log")
    void.echo("-!- Channel log enabled for " .. channel)
end

function lice5.chanlog.disable(channel)
    lice5.chanlog.channels[channel:upper()] = nil
    void.echo("-!- Channel log disabled for " .. channel)
end

function lice5.chanlog.list()
    if next(lice5.chanlog.channels) == nil then
        void.echo("-!- No channel logs configured.")
    else
        void.echo("-!- Channel logs:")
        for ch, path in pairs(lice5.chanlog.channels) do
            void.echo("  " .. ch .. " -> " .. path)
        end
    end
end

-- Command: /chanlog [#channel] [on|off|path]
void.register_command("CHANLOG", "lice5_cmd_chanlog")
function lice5_cmd_chanlog(args)
    local channel = void.channel()
    if #args == 0 then
        lice5.chanlog.list()
        return
    end
    if args[1]:sub(1, 1) == "#" then
        channel = args[1]
        table.remove(args, 1)
    end
    if channel == "" then
        void.echo("-!- Not in a channel.")
        return
    end
    local action = args[1] or "on"
    if action == "on" then
        lice5.chanlog.enable(channel, args[2])
    elseif action == "off" then
        lice5.chanlog.disable(channel)
    else
        lice5.chanlog.enable(channel, action)
    end
end
