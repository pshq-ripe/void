-- LiCe5: Tog — Generic toggle
-- Toggle various features

lice5.tog = {}

-- Command: /tog <feature> [on|off]
void.register_command("TOG", "lice5_cmd_tog")
function lice5_cmd_tog(args)
    if #args == 0 then
        void.echo("-!- Usage: /tog <feature> [on|off]")
        void.echo("-!- Features: timestamps, colors, nicks, modes, joins, parts, quits")
        return
    end
    local feature = args[1]:lower()
    void.echo("-!- Toggle: " .. feature .. " (use /ctog, /dtog, /wtog for specific toggles)")
end
