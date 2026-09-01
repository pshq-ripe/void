-- LiCe5: Invite Management
-- Track and manage channel invites

lice5.invite = {
    pending = {},  -- {nick, channel, time}
}

function lice5.invite.add(nick, channel)
    table.insert(lice5.invite.pending, {
        nick = nick,
        channel = channel,
        time = os.time(),
    })
    void.echo("-!- Invite from " .. nick .. " to " .. channel)
end

function lice5.invite.accept(channel)
    for i, inv in ipairs(lice5.invite.pending) do
        if inv.channel == channel or not channel then
            void.join(inv.channel)
            void.echo("-!- Accepted invite to " .. inv.channel .. " from " .. inv.nick)
            table.remove(lice5.invite.pending, i)
            return true
        end
    end
    void.echo("-!- No pending invite for " .. (channel or "any channel"))
    return false
end

function lice5.invite.reject(channel)
    for i, inv in ipairs(lice5.invite.pending) do
        if inv.channel == channel or not channel then
            void.echo("-!- Rejected invite to " .. inv.channel .. " from " .. inv.nick)
            table.remove(lice5.invite.pending, i)
            return true
        end
    end
    return false
end

-- Hook: track invites
void.on("INVITE", "lice5_invite_hook")
function lice5_invite_hook(args)
    local nick = args[1] or ""
    local channel = args[2] or ""
    if nick ~= void.nick() then
        lice5.invite.add(nick, channel)
    end
end

-- Command: /invite [accept|reject|list] [#channel]
void.register_command("INVLIST", "lice5_cmd_invlist")
function lice5_cmd_invlist(args)
    if #args == 0 or args[1] == "list" then
        if #lice5.invite.pending == 0 then
            void.echo("-!- No pending invites.")
        else
            void.echo("-!- Pending invites:")
            for i, inv in ipairs(lice5.invite.pending) do
                local ago = os.time() - inv.time
                void.echo("  " .. i .. ": " .. inv.nick .. " -> " .. inv.channel .. " (" .. ago .. "s ago)")
            end
        end
    elseif args[1] == "accept" or args[1] == "a" then
        lice5.invite.accept(args[2])
    elseif args[1] == "reject" or args[1] == "r" then
        lice5.invite.reject(args[2])
    end
end
