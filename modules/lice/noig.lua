-- LiCe5: Noig — No-ignore management
-- Prevent specific nicks from being ignored

lice5.noig = {
    whitelist = {},
}

function lice5.noig.add(nick)
    table.insert(lice5.noig.whitelist, nick:lower())
    void.echo("-!- No-ignore added: " .. nick)
end

function lice5.noig.remove(nick)
    for i, n in ipairs(lice5.noig.whitelist) do
        if n == nick:lower() then
            table.remove(lice5.noig.whitelist, i)
            void.echo("-!- No-ignore removed: " .. nick)
            return true
        end
    end
    return false
end

function lice5.noig.check(nick)
    for _, n in ipairs(lice5.noig.whitelist) do
        if n == nick:lower() then
            return true
        end
    end
    return false
end

function lice5.noig.list()
    if #lice5.noig.whitelist == 0 then
        void.echo("-!- No-ignore list is empty.")
    else
        void.echo("-!- No-ignore list:")
        for i, nick in ipairs(lice5.noig.whitelist) do
            void.echo("  " .. i .. ": " .. nick)
        end
    end
end

-- Command: /noig [nick]
void.register_command("NOIG", "lice5_cmd_noig")
function lice5_cmd_noig(args)
    if #args == 0 then
        lice5.noig.list()
        return
    end
    local nick = args[1]
    if not lice5.noig.remove(nick) then
        lice5.noig.add(nick)
    end
end
