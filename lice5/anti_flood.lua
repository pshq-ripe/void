-- LiCe5: Anti-Flood Protection
-- Detects and mitigates flood attacks

lice5.antiflood = {
    enabled = true,
    threshold = 5,      -- messages per window
    window = 3,         -- seconds
    action = "IGNORE",  -- IGNORE or KICK
    tracking = {},      -- nick -> {count, first_seen}
}

void.on("PUBLIC", "lice5_on_flood_check")
function lice5_on_flood_check(args)
    if not lice5.antiflood.enabled then return end
    
    local nick = args[1] or ""
    local now = os.time()
    
    if not lice5.antiflood.tracking[nick] then
        lice5.antiflood.tracking[nick] = { count = 1, first = now }
        return
    end
    
    local t = lice5.antiflood.tracking[nick]
    if now - t.first > lice5.antiflood.window then
        t.count = 1
        t.first = now
        return
    end
    
    t.count = t.count + 1
    
    if t.count > lice5.antiflood.threshold then
        void.echo("-!- Anti-flood: " .. nick .. " is flooding (" .. t.count .. " msgs)")
        if lice5.antiflood.action == "IGNORE" then
            -- The ignore will be handled by the main client
            void.echo("-!- Use /ignore " .. nick .. " to suppress")
        end
        t.count = 0
        t.first = now
    end
end

-- Command: /antiflood [on|off] [threshold]
void.register_command("ANTIFLOOD", "lice5_cmd_antiflood")
function lice5_cmd_antiflood(args)
    if args[1] == "on" then
        lice5.antiflood.enabled = true
        void.echo("-!- Anti-flood: ON")
    elseif args[1] == "off" then
        lice5.antiflood.enabled = false
        void.echo("-!- Anti-flood: OFF")
    elseif args[1] then
        lice5.antiflood.threshold = tonumber(args[1]) or 5
        void.echo("-!- Anti-flood threshold: " .. lice5.antiflood.threshold)
    else
        local status = lice5.antiflood.enabled and "ON" or "OFF"
        void.echo("-!- Anti-flood: " .. status .. " (threshold: " .. lice5.antiflood.threshold .. ")")
    end
end
