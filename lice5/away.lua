-- LiCe5: Away System
-- Enhanced away management with auto-away and away messages

lice5.away = {
    message = "",
    since = nil,
    auto_away = false,
    auto_away_time = 600,  -- seconds of idle before auto-away
    last_activity = os.time(),
}

function lice5.away.set(message)
    lice5.away.message = message or "Away"
    lice5.away.since = os.time()
    void.away(lice5.away.message)
    void.echo("-!- You are now away: " .. lice5.away.message)
end

function lice5.away.unset()
    lice5.away.message = ""
    lice5.away.since = nil
    void.away(nil)
    void.echo("-!- You are no longer away.")
end

function lice5.away.check_auto()
    if not lice5.away.auto_away then return end
    if lice5.away.since then return end  -- already away
    
    local idle = os.time() - lice5.away.last_activity
    if idle > lice5.away.auto_away_time then
        lice5.away.set("Auto-away (idle " .. math.floor(idle/60) .. "m)")
    end
end

-- Update activity on any input
void.on("PUBLIC", "lice5_away_activity")
void.on("MSG", "lice5_away_activity")
function lice5_away_activity(args)
    lice5.away.last_activity = os.time()
end

-- Command: /away [message]
void.register_command("LICE_AWAY", "lice5_cmd_away")
function lice5_cmd_away(args)
    if not args[1] then
        if lice5.away.since then
            local duration = os.time() - lice5.away.since
            local mins = math.floor(duration / 60)
            void.echo("-!- You are away: " .. lice5.away.message .. " (since " .. mins .. "m ago)")
        else
            void.echo("-!- You are not away.")
        end
        return
    end
    
    if args[1] == "off" or args[1] == "back" then
        lice5.away.unset()
    else
        lice5.away.set(table.concat(args, " "))
    end
end

-- Command: /autoaway [seconds]
void.register_command("AUTOAWAY", "lice5_cmd_autoaway")
function lice5_cmd_autoaway(args)
    if args[1] then
        lice5.away.auto_away_time = tonumber(args[1]) or 600
        lice5.away.auto_away = true
        void.echo("-!- Auto-away set to " .. lice5.away.auto_away_time .. " seconds")
    else
        local status = lice5.away.auto_away and "ON" or "OFF"
        void.echo("-!- Auto-away: " .. status .. " (" .. lice5.away.auto_away_time .. "s)")
    end
end
