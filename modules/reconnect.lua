-- LiCe5: Reconnect with Channel Rejoin
-- Auto-reconnect and channel recovery

lice5.reconnect = {
    enabled = true,
    delay = 10,         -- seconds before reconnect
    max_attempts = 5,   -- max reconnect attempts
    channels = {},      -- channels to rejoin after reconnect
    attempt = 0,
}

-- Save current channels before disconnect
function lice5.reconnect.save_channels()
    lice5.reconnect.channels = {}
    -- Would need buffer enumeration from Rust side
    -- For now, track via JOIN events
end

-- Hook: track joined channels
void.on("JOIN", "lice5_reconnect_track_join")
function lice5_reconnect_track_join(args)
    local nick = args[1] or ""
    local channel = args[2] or ""
    if nick == void.nick() and channel ~= "" then
        -- Add to channel list if not present
        local found = false
        for _, ch in ipairs(lice5.reconnect.channels) do
            if ch == channel then found = true; break end
        end
        if not found then
            table.insert(lice5.reconnect.channels, channel)
        end
    end
end

-- Hook: track parted channels
void.on("PART", "lice5_reconnect_track_part")
function lice5_reconnect_track_part(args)
    local nick = args[1] or ""
    local channel = args[2] or ""
    if nick == void.nick() then
        for i, ch in ipairs(lice5.reconnect.channels) do
            if ch == channel then
                table.remove(lice5.reconnect.channels, i)
                break
            end
        end
    end
end

-- Rejoin saved channels after reconnect
function lice5.reconnect.rejoin()
    if #lice5.reconnect.channels > 0 then
        void.echo("-!- Rejoining " .. #lice5.reconnect.channels .. " channels...")
        for _, channel in ipairs(lice5.reconnect.channels) do
            void.join(channel)
        end
    end
end

-- Command: /reconnect
void.register_command("RECONNECT", "lice5_cmd_reconnect")
function lice5_cmd_reconnect(args)
    lice5.reconnect.save_channels()
    void.echo("-!- Reconnecting (will rejoin " .. #lice5.reconnect.channels .. " channels)...")
    -- The actual reconnect is handled by the Rust side
    -- This just saves the channel list
end

-- Command: /rejoin — rejoin saved channels
void.register_command("REJOIN", "lice5_cmd_rejoin")
function lice5_cmd_rejoin(args)
    lice5.reconnect.rejoin()
end
