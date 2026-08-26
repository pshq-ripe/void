-- LiCe5: Status Bar Enhancements
-- Custom status bar with dynamic information

lice5.statusbar = {
    format = " [$nick] [$channel] [$server] ",
    show_ops = true,
    show_users = true,
}

-- Update status bar with current info
function lice5.statusbar.update()
    -- The status bar is rendered by the Rust side
    -- This module provides helper functions for status info
end

-- Command: /sb [format]
void.register_command("SB", "lice5_cmd_statusbar")
void.register_command("STATUSBAR", "lice5_cmd_statusbar")
function lice5_cmd_statusbar(args)
    if args[1] then
        lice5.statusbar.format = table.concat(args, " ")
        void.echo("-!- Status bar format: " .. lice5.statusbar.format)
    else
        void.echo("-!- Status bar: " .. lice5.statusbar.format)
        void.echo("-!- Variables: $nick $channel $server $topic $users $ops")
    end
end
