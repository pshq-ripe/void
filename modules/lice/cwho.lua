-- LiCe5: Cwho — Channel WHO
-- Enhanced WHO for channels

lice5.cwho = {}

-- Command: /cwho [#channel] [pattern]
void.register_command("CWHO", "lice5_cmd_cwho")
function lice5_cmd_cwho(args)
    local channel = void.channel()
    if #args > 0 and args[1]:sub(1, 1) == "#" then
        channel = args[1]
        table.remove(args, 1)
    end
    if channel == "" then
        void.echo("-!- Not in a channel.")
        return
    end
    local pattern = args[1] or "*"
    void.echo("-!- WHO " .. channel .. " " .. pattern)
    void.send("WHO " .. channel .. " " .. pattern)
end
