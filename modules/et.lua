-- LiCe5: Et — Enhanced topic
-- Enhanced topic management

lice5.et = {}

-- Command: /et [text] — enhanced topic
void.register_command("ET", "lice5_cmd_et")
function lice5_cmd_et(args)
    local channel = void.channel()
    if channel == "" then
        void.echo("-!- Not in a channel.")
        return
    end
    if #args == 0 then
        void.echo("-!- Current topic: (use /topic)")
    else
        local topic = table.concat(args, " ")
        void.topic(channel, topic)
    end
end
