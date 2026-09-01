-- LiCe5: Msay — Multi-target say
-- Say to multiple channels at once

lice5.msay = {}

-- Command: /msay <message> — say to all joined channels
void.register_command("MSAY", "lice5_cmd_msay")
function lice5_cmd_msay(args)
    if #args == 0 then
        void.echo("-!- Usage: /msay <message>")
        return
    end
    local message = table.concat(args, " ")
    void.echo("-!- Broadcasting: " .. message)
    -- Send to current channel
    local channel = void.channel()
    if channel ~= "" then
        void.msg(channel, message)
    end
end
