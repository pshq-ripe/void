-- LiCe5: Rel — Relationship tracking
-- Track relationships with other users

lice5.rel = {
    relationships = {},  -- {nick, type, notes}
    types = {"friend", "enemy", "neutral", "bot", "oper"},
}

function lice5.rel.set(nick, rel_type, notes)
    lice5.rel.relationships[nick:lower()] = {
        nick = nick,
        type = rel_type or "neutral",
        notes = notes or "",
        time = os.time(),
    }
    void.echo("-!- Relationship set: " .. nick .. " = " .. (rel_type or "neutral"))
end

function lice5.rel.get(nick)
    return lice5.rel.relationships[nick:lower()]
end

function lice5.rel.list()
    if next(lice5.rel.relationships) == nil then
        void.echo("-!- No relationships tracked.")
    else
        void.echo("-!- Relationships:")
        for _, rel in pairs(lice5.rel.relationships) do
            void.echo("  " .. rel.nick .. " [" .. rel.type .. "]" .. (rel.notes ~= "" and (" - " .. rel.notes) or ""))
        end
    end
end

-- Command: /rel [nick] [type] [notes]
void.register_command("REL", "lice5_cmd_rel")
function lice5_cmd_rel(args)
    if #args == 0 then
        lice5.rel.list()
        return
    end
    if #args == 1 then
        local rel = lice5.rel.get(args[1])
        if rel then
            void.echo("-!- " .. rel.nick .. " [" .. rel.type .. "] " .. rel.notes)
        else
            void.echo("-!- No relationship with " .. args[1])
        end
        return
    end
    lice5.rel.set(args[1], args[2], #args > 2 and table.concat(args, " ", 3) or "")
end
