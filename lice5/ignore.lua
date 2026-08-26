-- LiCe5: Enhanced Ignore System
-- Patterns, reasons, timeouts — epic6 style with /ON CONTEXT + /SHH

lice5.ignore = {
    entries = {},  -- {pattern, flags, reason, timeout, created}
}

function lice5.ignore.add(pattern, flags, reason, timeout)
    -- Remove existing entry with same pattern
    lice5.ignore.remove(pattern)
    table.insert(lice5.ignore.entries, {
        pattern = pattern,
        flags = flags or "ALL",
        reason = reason or "",
        timeout = timeout or 0,
        created = os.time(),
    })
    void.echo("-!- Ignore: " .. pattern .. " [" .. flags .. "]" .. (reason ~= "" and (" (" .. reason .. ")") or ""))
end

function lice5.ignore.remove(pattern)
    for i, entry in ipairs(lice5.ignore.entries) do
        if entry.pattern == pattern then
            table.remove(lice5.ignore.entries, i)
            void.echo("-!- Unignored: " .. pattern)
            return true
        end
    end
    return false
end

function lice5.ignore.check(nick, msg_type)
    local now = os.time()
    for i, entry in ipairs(lice5.ignore.entries) do
        -- Check timeout
        if entry.timeout > 0 and (now - entry.created) > entry.timeout then
            table.remove(lice5.ignore.entries, i)
            void.echo("-!- Ignore expired: " .. entry.pattern)
            return false
        end
        -- Pattern match
        if void.match(entry.pattern, nick) then
            if entry.flags == "ALL" then return true end
            if entry.flags:upper():find(msg_type:upper()) then return true end
        end
    end
    return false
end

-- Hook: check ignores on PUBLIC/MSG/NOTICE/CTCP
void.on("PUBLIC", "lice5_ignore_public")
function lice5_ignore_public(args)
    local nick = args[1] or ""
    if lice5.ignore.check(nick, "PUBLIC") then
        return ""  -- suppress
    end
end

void.on("MSG", "lice5_ignore_msg")
function lice5_ignore_msg(args)
    local nick = args[1] or ""
    if lice5.ignore.check(nick, "MSG") then
        return ""
    end
end

-- Command: /ig <pattern> [flags] [reason "text"] [timeout N]
void.register_command("IG", "lice5_cmd_ignore")
void.register_command("IGNORE", "lice5_cmd_ignore")
function lice5_cmd_ignore(args)
    if not args[1] then
        if #lice5.ignore.entries == 0 then
            void.echo("-!- Ignore list is empty.")
        else
            void.echo("-!- Ignore list:")
            for i, entry in ipairs(lice5.ignore.entries) do
                local timeout_str = entry.timeout > 0 and (" timeout:" .. entry.timeout .. "s") or ""
                void.echo("  " .. i .. ": " .. entry.pattern .. " [" .. entry.flags .. "]" .. timeout_str .. (entry.reason ~= "" and (" " .. entry.reason) or ""))
            end
        end
        return
    end

    local pattern = args[1]

    -- Toggle: remove if exists
    if lice5.ignore.remove(pattern) then return end

    local flags = "ALL"
    local reason = ""
    local timeout = 0

    -- Parse flags
    for i = 2, #args do
        local arg = args[i]
        if arg:upper() == "ALL" or arg:upper() == "PUBLIC" or arg:upper() == "MSG" or arg:upper() == "NOTICE" or arg:upper() == "CTCP" then
            flags = arg:upper()
        elseif arg:lower() == "reason" and args[i+1] then
            reason = args[i+1]
        elseif arg:lower() == "timeout" and args[i+1] then
            timeout = tonumber(args[i+1]) or 0
        end
    end

    lice5.ignore.add(pattern, flags, reason, timeout)
end
