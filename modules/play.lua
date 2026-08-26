-- LiCe5: Play — Log Replay
-- Replay log files into the current buffer

lice5.play = {}

function lice5.play.replay(filename, delay)
    void.echo("-!- Replaying: " .. filename)
    local content = void.file_read(filename)
    if content:sub(1, 5) == "Error" then
        void.echo("-!- " .. content)
        return
    end
    local count = 0
    for line in content:gmatch("[^\n]+") do
        line = line:match("^%s*(.-)%s*$")
        if line and line ~= "" then
            void.echo(line)
            count = count + 1
        end
    end
    void.echo("-!- Replayed " .. count .. " lines from " .. filename)
end

-- Command: /play <filename>
void.register_command("PLAY", "lice5_cmd_play")
function lice5_cmd_play(args)
    if #args == 0 then
        void.echo("-!- Usage: /play <logfile>")
        return
    end
    lice5.play.replay(args[1])
end
