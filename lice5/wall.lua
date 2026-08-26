-- LiCe5: Wall (Broadcast)
-- Send message to all channels

lice5.wall = {}

function lice5.wall.send(message)
    if not message or message == "" then
        void.echo("-!- Usage: /wall <message>")
        return
    end
    void.echo("-!- Wall: " .. message)
    -- Send to current channel as a notice
    local channel = void.channel()
    if channel ~= "" then
        void.notice(channel, "*** WALL: " .. message .. " ***")
    end
end

-- Command: /wall <message>
void.register_command("WALL", "lice5_cmd_wall")
function lice5_cmd_wall(args)
    if #args == 0 then
        void.echo("-!- Usage: /wall <message>")
        return
    end
    lice5.wall.send(table.concat(args, " "))
end
