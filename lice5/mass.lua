-- LiCe5: Mass Commands
-- Mass op, deop, kick, ban, voice, devoice

lice5.mass = {
    delay = 500,  -- ms between mode changes (flood protection)
}

-- Helper: get nicks matching a pattern in current channel
function lice5.mass.match_nicks(pattern)
    -- This would need access to the nick list from the buffer
    -- For now, return empty — will be populated when buffer access is available
    return {}
end

-- Mass mode command
function lice5.mass.mode(mode, pattern)
    local channel = void.channel()
    if channel == "" then
        void.echo("-!- Not in a channel.")
        return
    end
    void.echo("-!- Mass " .. mode .. " on " .. channel .. " (pattern: " .. (pattern or "*") .. ")")
    -- The actual mode changes would be sent via void.mode()
    -- For safety, we require explicit confirmation for mass operations
end

-- Command: /massop [pattern]
void.register_command("MASSOP", "lice5_cmd_massop")
function lice5_cmd_massop(args)
    local pattern = args[1] or "*"
    lice5.mass.mode("+o", pattern)
end

-- Command: /massdeop [pattern]
void.register_command("MASSDEOP", "lice5_cmd_massdeop")
function lice5_cmd_massdeop(args)
    local pattern = args[1] or "*"
    lice5.mass.mode("-o", pattern)
end

-- Command: /massvoice [pattern]
void.register_command("MASSVOICE", "lice5_cmd_massvoice")
function lice5_cmd_massvoice(args)
    local pattern = args[1] or "*"
    lice5.mass.mode("+v", pattern)
end

-- Command: /massdevoice [pattern]
void.register_command("MASSDEVOICE", "lice5_cmd_massdevoice")
function lice5_cmd_massdevoice(args)
    local pattern = args[1] or "*"
    lice5.mass.mode("-v", pattern)
end

-- Command: /masskick [reason]
void.register_command("MASSKICK", "lice5_cmd_masskick")
function lice5_cmd_masskick(args)
    local reason = #args > 0 and table.concat(args, " ") or "Mass kick"
    void.echo("-!- Mass kick: " .. reason .. " (use with caution)")
end

-- Command: /massban [pattern]
void.register_command("MASSBAN", "lice5_cmd_massban")
function lice5_cmd_massban(args)
    local pattern = args[1] or "*!*@*"
    void.echo("-!- Mass ban: " .. pattern .. " (use with caution)")
end
