-- LiCe5: Defaults — Default settings
-- Display and reset default settings

lice5.defaults = {}

-- Command: /defaults [show|reset]
void.register_command("DEFAULTS", "lice5_cmd_defaults")
function lice5_cmd_defaults(args)
    if #args == 0 or args[1] == "show" then
        void.echo("-!- Default settings:")
        void.echo("  SCROLL_LINES=1, SCROLLBACK=500")
        void.echo("  BEEP_ON_MSG=OFF, CLOCK_24HOUR=ON")
        void.echo("  SHOW_TIMESTAMPS=ON, TIMESTAMP_FORMAT=%H:%M")
        void.echo("  FLOOD_PROTECTION=ON, FLOOD_RATE=4")
        void.echo("  AUTO_RECONNECT=ON, AUTO_RECONNECT_DELAY=15")
        void.echo("  CTCP_REPLY=ON, SSL_VERIFY=OFF")
        return
    end
    if args[1] == "reset" then
        void.echo("-!- Reset to defaults: (use /set to change individual settings)")
    end
end
