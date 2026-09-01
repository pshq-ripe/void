-- LiCe5: Server Ignore / Silence
-- Server-level ignore (SILENCE command)

lice5.serverignore = {
    patterns = {},
}

function lice5.serverignore.add(pattern)
    table.insert(lice5.serverignore.patterns, pattern)
    void.echo("-!- Server ignore added: " .. pattern)
    -- Send SILENCE command if supported
    void.send("SILENCE +" .. pattern)
end

function lice5.serverignore.remove(pattern)
    for i, p in ipairs(lice5.serverignore.patterns) do
        if p == pattern then
            table.remove(lice5.serverignore.patterns, i)
            void.echo("-!- Server ignore removed: " .. pattern)
            void.send("SILENCE -" .. pattern)
            return true
        end
    end
    return false
end

function lice5.serverignore.list()
    if #lice5.serverignore.patterns == 0 then
        void.echo("-!- No server ignores.")
    else
        void.echo("-!- Server ignores:")
        for i, p in ipairs(lice5.serverignore.patterns) do
            void.echo("  " .. i .. ": " .. p)
        end
    end
end

-- Command: /silence [pattern]
void.register_command("SILENCE", "lice5_cmd_silence")
function lice5_cmd_silence(args)
    if #args == 0 then
        lice5.serverignore.list()
        return
    end
    local pattern = args[1]
    -- Toggle
    if not lice5.serverignore.remove(pattern) then
        lice5.serverignore.add(pattern)
    end
end
