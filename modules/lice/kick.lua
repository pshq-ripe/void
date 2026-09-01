-- LiCe5: Enhanced Kick/Kickban with Random Reasons

lice5.kick = {
    reasons = {},
    kickban_reasons = {},
}

function lice5.kick.load_reasons()
    -- Load kick reasons
    local f = io.open("modules/kick.reasons", "r")
    if f then
        for line in f:lines() do
            line = line:match("^%s*(.-)%s*$")
            if line and line ~= "" and not line:match("^#") then
                table.insert(lice5.kick.reasons, line)
            end
        end
        f:close()
    end
    -- Fallback
    if #lice5.kick.reasons == 0 then
        lice5.kick.reasons = {
            "Requested",
            "Bye bye",
            "Get out",
            "You're not welcome",
            "Channel rules violation",
            "Spam",
            "Flood",
            "Harassment",
        }
    end
end

function lice5.kick.random_reason(list)
    if #list == 0 then return "Kicked" end
    return list[math.random(1, #list)]
end

-- Initialize
lice5.kick.load_reasons()

-- Command: /k <nick> [reason] — kick with optional random reason
void.register_command("K", "lice5_cmd_kick_enhanced")
function lice5_cmd_kick_enhanced(args)
    if #args == 0 then
        void.echo("-!- Usage: /k <nick> [reason]")
        return
    end
    local nick = args[1]
    local reason = #args > 1 and table.concat(args, " ", 2) or lice5.kick.random_reason(lice5.kick.reasons)
    local channel = void.channel()
    if channel == "" then
        void.echo("-!- Not in a channel.")
        return
    end
    void.kick(channel, nick, reason)
end

-- Command: /kb <nick> [reason] — kickban with optional random reason
void.register_command("KB", "lice5_cmd_kickban_enhanced")
function lice5_cmd_kickban_enhanced(args)
    if #args == 0 then
        void.echo("-!- Usage: /kb <nick> [reason]")
        return
    end
    local nick = args[1]
    local reason = #args > 1 and table.concat(args, " ", 2) or lice5.kick.random_reason(lice5.kick.reasons)
    local channel = void.channel()
    if channel == "" then
        void.echo("-!- Not in a channel.")
        return
    end
    -- Ban first, then kick
    void.ban(channel, nick)
    void.kick(channel, nick, reason)
end

-- Command: /rk <nick> — random kick reason
void.register_command("RK", "lice5_cmd_random_kick")
function lice5_cmd_random_kick(args)
    if #args == 0 then
        void.echo("-!- Usage: /rk <nick>")
        return
    end
    local nick = args[1]
    local channel = void.channel()
    if channel == "" then
        void.echo("-!- Not in a channel.")
        return
    end
    void.kick(channel, nick, lice5.kick.random_reason(lice5.kick.reasons))
end
