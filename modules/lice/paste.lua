-- LiCe5: Paste Mode (epic6 style)
-- Multi-line input with bracketed paste support

lice5.paste = {
    enabled = false,
    buffer = {},
    max_lines = 50,
    timeout = 5,  -- seconds of inactivity before auto-send
}

function lice5.paste.start()
    lice5.paste.enabled = true
    lice5.paste.buffer = {}
    void.echo("-!- Paste mode started. Type lines, then /paste send to send, /paste cancel to cancel.")
end

function lice5.paste.add_line(line)
    if not lice5.paste.enabled then
        lice5.paste.start()
    end
    if #lice5.paste.buffer >= lice5.paste.max_lines then
        void.echo("-!- Paste buffer full (" .. lice5.paste.max_lines .. " lines max)")
        return
    end
    table.insert(lice5.paste.buffer, line)
end

function lice5.paste.send()
    if #lice5.paste.buffer == 0 then
        void.echo("-!- Paste buffer is empty.")
        return
    end
    local channel = void.channel()
    if channel == "" then
        void.echo("-!- Not in a channel.")
        return
    end
    void.echo("-!- Sending " .. #lice5.paste.buffer .. " lines...")
    for _, line in ipairs(lice5.paste.buffer) do
        void.msg(channel, line)
    end
    void.echo("-!- Paste sent.")
    lice5.paste.buffer = {}
    lice5.paste.enabled = false
end

function lice5.paste.cancel()
    void.echo("-!- Paste cancelled (" .. #lice5.paste.buffer .. " lines discarded)")
    lice5.paste.buffer = {}
    lice5.paste.enabled = false
end

function lice5.paste.show()
    if #lice5.paste.buffer == 0 then
        void.echo("-!- Paste buffer is empty.")
    else
        void.echo("-!- Paste buffer (" .. #lice5.paste.buffer .. " lines):")
        for i, line in ipairs(lice5.paste.buffer) do
            void.echo("  " .. i .. ": " .. line)
        end
    end
end

-- Command: /paste [send|cancel|show]
void.register_command("PASTE", "lice5_cmd_paste")
function lice5_cmd_paste(args)
    if #args == 0 then
        lice5.paste.start()
    elseif args[1] == "send" then
        lice5.paste.send()
    elseif args[1] == "cancel" then
        lice5.paste.cancel()
    elseif args[1] == "show" then
        lice5.paste.show()
    else
        -- Treat as a line to add
        lice5.paste.add_line(table.concat(args, " "))
    end
end
