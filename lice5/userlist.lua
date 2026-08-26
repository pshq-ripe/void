-- LiCe5: Userlist — Bot-style Auto-op/Voice/Kickban
-- Persistent user database with access levels

lice5.userlist = {
    users = {},  -- {nick, host, level, channels, added_by, added_at}
    levels = {
        OWNER = 100,
        ADMIN = 90,
        OP = 80,
        HALFOP = 70,
        VOICE = 60,
        FRIEND = 50,
        NONE = 0,
    },
}

function lice5.userlist.add(nick, host, level, channels)
    -- Remove existing entry
    lice5.userlist.remove(nick)
    table.insert(lice5.userlist.users, {
        nick = nick,
        host = host or "*!*@*",
        level = level or "FRIEND",
        channels = channels or "*",
        added_by = void.nick(),
        added_at = os.time(),
    })
    void.echo("-!- Userlist: added " .. nick .. " (" .. host .. ") level:" .. level)
end

function lice5.userlist.remove(nick)
    for i, user in ipairs(lice5.userlist.users) do
        if user.nick:lower() == nick:lower() then
            table.remove(lice5.userlist.users, i)
            void.echo("-!- Userlist: removed " .. nick)
            return true
        end
    end
    return false
end

function lice5.userlist.find(nick, host)
    for _, user in ipairs(lice5.userlist.users) do
        if user.nick:lower() == nick:lower() then
            return user
        end
        if host and void.match(user.host, host) then
            return user
        end
    end
    return nil
end

function lice5.userlist.get_level(nick, host)
    local user = lice5.userlist.find(nick, host)
    if user then
        return lice5.userlist.levels[user.level] or 0
    end
    return 0
end

-- Hook: auto-op on join (if user is in userlist with OP+ level)
void.on("JOIN", "lice5_userlist_autoop")
function lice5_userlist_autoop(args)
    local nick = args[1] or ""
    local channel = args[2] or ""
    if nick == void.nick() then return end

    local level = lice5.userlist.get_level(nick)
    if level >= lice5.userlist.levels.OP then
        void.op(channel, nick)
        void.echo("-!- Auto-op: " .. nick .. " on " .. channel)
    elseif level >= lice5.userlist.levels.VOICE then
        void.voice(channel, nick)
        void.echo("-!- Auto-voice: " .. nick .. " on " .. channel)
    end
end

-- Command: /userlist [add|del|list] [nick] [host] [level]
void.register_command("USERLIST", "lice5_cmd_userlist")
void.register_command("UL", "lice5_cmd_userlist")
function lice5_cmd_userlist(args)
    if #args == 0 or args[1] == "list" then
        if #lice5.userlist.users == 0 then
            void.echo("-!- Userlist is empty.")
        else
            void.echo("-!- Userlist:")
            for i, user in ipairs(lice5.userlist.users) do
                void.echo("  " .. i .. ": " .. user.nick .. " (" .. user.host .. ") level:" .. user.level .. " channels:" .. user.channels)
            end
        end
        return
    end

    local action = args[1]:lower()

    if action == "add" then
        if #args < 2 then
            void.echo("-!- Usage: /userlist add <nick> [host] [level]")
            return
        end
        local nick = args[2]
        local host = args[3] or "*!*@*"
        local level = args[4] or "FRIEND"
        lice5.userlist.add(nick, host, level)
    elseif action == "del" or action == "remove" then
        if #args < 2 then
            void.echo("-!- Usage: /userlist del <nick>")
            return
        end
        lice5.userlist.remove(args[2])
    elseif action == "op" then
        if #args < 2 then
            void.echo("-!- Usage: /userlist op <nick>")
            return
        end
        local user = lice5.userlist.find(args[2])
        if user then
            user.level = "OP"
            void.echo("-!- " .. args[2] .. " set to OP level")
        else
            void.echo("-!- User not found: " .. args[2])
        end
    elseif action == "voice" then
        if #args < 2 then
            void.echo("-!- Usage: /userlist voice <nick>")
            return
        end
        local user = lice5.userlist.find(args[2])
        if user then
            user.level = "VOICE"
            void.echo("-!- " .. args[2] .. " set to VOICE level")
        else
            void.echo("-!- User not found: " .. args[2])
        end
    else
        void.echo("-!- Usage: /userlist [add|del|list|op|voice] [nick] [host] [level]")
    end
end
