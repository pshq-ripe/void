-- LiCe5: Floodlist — Flood protection list
-- Manage flood protection exceptions

lice5.floodlist = {
    exceptions = {},
}

function lice5.floodlist.add(nick)
    table.insert(lice5.floodlist.exceptions, nick:lower())
    void.echo("-!- Flood exception added: " .. nick)
end

function lice5.floodlist.remove(nick)
    for i, n in ipairs(lice5.floodlist.exceptions) do
        if n == nick:lower() then
            table.remove(lice5.floodlist.exceptions, i)
            void.echo("-!- Flood exception removed: " .. nick)
            return true
        end
    end
    return false
end

function lice5.floodlist.list()
    if #lice5.floodlist.exceptions == 0 then
        void.echo("-!- No flood exceptions.")
    else
        void.echo("-!- Flood exceptions:")
        for i, nick in ipairs(lice5.floodlist.exceptions) do
            void.echo("  " .. i .. ": " .. nick)
        end
    end
end

-- Command: /floodlist [nick]
void.register_command("FLOODLIST", "lice5_cmd_floodlist")
function lice5_cmd_floodlist(args)
    if #args == 0 then
        lice5.floodlist.list()
        return
    end
    if not lice5.floodlist.remove(args[1]) then
        lice5.floodlist.add(args[1])
    end
end
