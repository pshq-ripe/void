-- LiCe5: Note System
-- Quick notes and reminders

lice5.note = {
    notes = {},
}

function lice5.note.add(text)
    table.insert(lice5.note.notes, {
        text = text,
        time = os.time(),
    })
    void.echo("-!- Note added: " .. text)
end

function lice5.note.list()
    if #lice5.note.notes == 0 then
        void.echo("-!- No notes.")
    else
        void.echo("-!- Notes:")
        for i, note in ipairs(lice5.note.notes) do
            local ago = os.time() - note.time
            void.echo("  " .. i .. ": " .. note.text .. " (" .. ago .. "s ago)")
        end
    end
end

function lice5.note.clear()
    lice5.note.notes = {}
    void.echo("-!- All notes cleared.")
end

-- Command: /note [add|list|clear] [text]
void.register_command("NOTE", "lice5_cmd_note")
function lice5_cmd_note(args)
    if #args == 0 then
        lice5.note.list()
        return
    end
    local action = args[1]:lower()
    if action == "add" then
        lice5.note.add(table.concat(args, " ", 2))
    elseif action == "list" then
        lice5.note.list()
    elseif action == "clear" then
        lice5.note.clear()
    else
        lice5.note.add(table.concat(args, " "))
    end
end
