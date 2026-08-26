-- LiCe5: Help System
-- Enhanced help with categories

lice5.help = {
    categories = {},
}

function lice5.help.register(category, commands)
    lice5.help.categories[category] = commands
end

-- Register help categories
lice5.help.register("Channel", {"join", "part", "topic", "names", "kick", "ban", "unban", "op", "deop", "voice", "devoice", "mode", "invite", "cycle"})
lice5.help.register("Message", {"msg", "me", "notice", "say", "query", "ctcp", "wallops", "ping"})
lice5.help.register("Server", {"server", "disconnect", "reconnect", "quit", "nick", "away", "whois", "who", "userhost"})
lice5.help.register("Window", {"window", "clear", "lastlog", "scroll"})
lice5.help.register("Config", {"set", "alias", "unalias", "highlight", "bind", "format", "save"})
lice5.help.register("System", {"help", "raw", "echo", "exec", "log", "eval", "dcc", "timer", "notify", "ignore"})
lice5.help.register("LiCe5", {"gone", "back", "autoaway", "ig", "k", "kb", "rk", "ul", "alarm", "paste", "logman", "protect", "autovoice", "antiflood", "nickserv", "ns", "massop", "massdeop", "massvoice", "massdevoice", "memo", "note", "dns", "finger", "wall", "signoff", "party", "sensors", "invlist"})

-- Command: /help [category|command]
void.register_command("LICE_HELP", "lice5_cmd_help")
function lice5_cmd_help(args)
    if #args == 0 then
        void.echo("-!- Help categories:")
        for cat, cmds in pairs(lice5.help.categories) do
            void.echo("  " .. cat .. ": " .. #cmds .. " commands")
        end
        void.echo("-!- Type /help <category> or /help <command>")
        return
    end
    local query = args[1]:lower()
    -- Check categories
    for cat, cmds in pairs(lice5.help.categories) do
        if cat:lower() == query then
            void.echo("-!- " .. cat .. " commands:")
            void.echo("  " .. table.concat(cmds, ", "))
            return
        end
    end
    void.echo("-!- No help for: " .. query)
end
