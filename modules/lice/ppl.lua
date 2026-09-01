-- LiCe5: Ppl — People tracking
-- Track people you've interacted with

lice5.ppl = {
    people = {},
}

function lice5.ppl.add(nick, notes)
    lice5.ppl.people[nick:lower()] = {
        nick = nick,
        notes = notes or "",
        seen = os.time(),
        interactions = 0,
    }
    void.echo("-!- Person added: " .. nick)
end

function lice5.ppl.seen(nick)
    local person = lice5.ppl.people[nick:lower()]
    if person then
        local ago = os.time() - person.seen
        void.echo("-!- " .. person.nick .. ": last seen " .. ago .. "s ago, " .. person.interactions .. " interactions")
        if person.notes ~= "" then
            void.echo("-!- Notes: " .. person.notes)
        end
    else
        void.echo("-!- No record for: " .. nick)
    end
end

function lice5.ppl.list()
    if next(lice5.ppl.people) == nil then
        void.echo("-!- No people tracked.")
    else
        void.echo("-!- People:")
        for _, person in pairs(lice5.ppl.people) do
            void.echo("  " .. person.nick .. " (" .. person.interactions .. " interactions)")
        end
    end
end

-- Command: /ppl [add|seen|list] [nick] [notes]
void.register_command("PPL", "lice5_cmd_ppl")
function lice5_cmd_ppl(args)
    if #args == 0 then
        lice5.ppl.list()
        return
    end
    local action = args[1]:lower()
    if action == "add" then
        lice5.ppl.add(args[2] or "", #args > 2 and table.concat(args, " ", 3) or "")
    elseif action == "seen" then
        lice5.ppl.seen(args[2] or "")
    elseif action == "list" then
        lice5.ppl.list()
    else
        lice5.ppl.add(args[1], #args > 1 and table.concat(args, " ", 2) or "")
    end
end
