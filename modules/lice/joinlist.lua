-- LiCe5: Join List / Clone Detection
-- Track joins and detect clones (same host joining)

lice5.joinlist = {
    recent = {},  -- {nick, host, channel, time}
    clone_window = 10,  -- seconds to detect clones
}

function lice5.joinlist.track(nick, host, channel)
    local now = os.time()
    table.insert(lice5.joinlist.recent, {
        nick = nick,
        host = host,
        channel = channel,
        time = now,
    })
    -- Clean old entries
    local i = 1
    while i <= #lice5.joinlist.recent do
        if now - lice5.joinlist.recent[i].time > 60 then
            table.remove(lice5.joinlist.recent, i)
        else
            i = i + 1
        end
    end
    -- Check for clones
    for _, entry in ipairs(lice5.joinlist.recent) do
        if entry.host == host and entry.nick ~= nick
           and entry.channel == channel
           and (now - entry.time) < lice5.joinlist.clone_window then
            void.echo("-!- Clone detected: " .. nick .. " and " .. entry.nick .. " (same host: " .. host .. ")")
        end
    end
end

-- Hook: track joins
void.on("JOIN", "lice5_joinlist_track")
function lice5_joinlist_track(args)
    local nick = args[1] or ""
    local channel = args[2] or ""
    -- We don't have host info from the hook, so just track nick/channel
    lice5.joinlist.track(nick, "*", channel)
end

-- Command: /joinlist — show recent joins
void.register_command("JOINLIST", "lice5_cmd_joinlist")
function lice5_cmd_joinlist(args)
    if #lice5.joinlist.recent == 0 then
        void.echo("-!- No recent joins tracked.")
    else
        void.echo("-!- Recent joins:")
        for i, entry in ipairs(lice5.joinlist.recent) do
            local ago = os.time() - entry.time
            void.echo("  " .. i .. ": " .. entry.nick .. " -> " .. entry.channel .. " (" .. ago .. "s ago)")
        end
    end
end
