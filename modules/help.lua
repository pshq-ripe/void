-- LiCe5: Help System
-- Enhanced help with categories and module info

lice5.help = {
    categories = {},
    cmd_help = {},
}

function lice5.help.register(category, commands)
    lice5.help.categories[category] = commands
end

function lice5.help.add_cmd_help(cmd, desc)
    lice5.help.cmd_help[cmd:lower()] = desc
end

-- Register help categories
lice5.help.register("Channel", {"join", "part", "topic", "names", "kick", "ban", "unban", "op", "deop", "voice", "devoice", "mode", "invite", "cycle", "list", "knock"})
lice5.help.register("Message", {"msg", "me", "notice", "say", "query", "ctcp", "wallops", "ping"})
lice5.help.register("Server", {"server", "disconnect", "reconnect", "quit", "nick", "away", "whois", "who", "userhost", "lusers", "admin", "info", "motd", "stats", "links", "map", "trace"})
lice5.help.register("Window", {"window", "clear", "lastlog", "scroll", "repaint"})
lice5.help.register("Config", {"set", "alias", "unalias", "highlight", "bind", "format", "save", "load", "reload", "charset"})
lice5.help.register("System", {"help", "raw", "echo", "exec", "log", "eval", "dcc", "timer", "notify", "ignore", "debug", "cd", "pwd", "shh"})
lice5.help.register("LiCe5", {"gone", "back", "autoaway", "ig", "k", "kb", "rk", "ul", "alarm", "paste", "logman", "protect", "autovoice", "antiflood", "nickserv", "ns", "massop", "massdeop", "massvoice", "massdevoice", "memo", "note", "dns", "finger", "wall", "signoff", "party", "sensors", "invlist", "module", "theme"})
lice5.help.register("Bouncer", {"bouncer"})
lice5.help.register("IRCv3", {"caplist", "starttls", "chathistory", "rawlog"})

-- Command help entries
lice5.help.add_cmd_help("join", "/join <#channel> [key] — Join a channel")
lice5.help.add_cmd_help("part", "/part [#channel] [reason] — Leave a channel")
lice5.help.add_cmd_help("msg", "/msg <target> <text> — Send private message")
lice5.help.add_cmd_help("me", "/me <action> — Send action")
lice5.help.add_cmd_help("nick", "/nick <newnick> — Change nickname")
lice5.help.add_cmd_help("away", "/away [message] — Set/unset away")
lice5.help.add_cmd_help("whois", "/whois <nick> — Query user info")
lice5.help.add_cmd_help("mode", "/mode <target> <modes> — Set modes")
lice5.help.add_cmd_help("kick", "/kick <nick> [reason] — Kick user")
lice5.help.add_cmd_help("ban", "/ban <nick|mask> — Ban user")
lice5.help.add_cmd_help("op", "/op <nick> — Give operator")
lice5.help.add_cmd_help("voice", "/voice <nick> — Give voice")
lice5.help.add_cmd_help("topic", "/topic [text] — View/set topic")
lice5.help.add_cmd_help("set", "/set [variable] [value] — View/change settings")
lice5.help.add_cmd_help("alias", "/alias [name] [body] — Define/show alias")
lice5.help.add_cmd_help("save", "/save — Save settings to SQLite + void.conf")
lice5.help.add_cmd_help("theme", "/theme [name|list|info|random] — Change theme")
lice5.help.add_cmd_help("module", "/module [name] — Show module info")
lice5.help.add_cmd_help("bouncer", "/bouncer [start|stop|status] [port] [password] — IRC bouncer")
lice5.help.add_cmd_help("debug", "/debug [on|off|raw show|save] — Debug mode")
lice5.help.add_cmd_help("log", "/log [on|off] — Toggle logging")
lice5.help.add_cmd_help("window", "/window [next|prev|goto|list|last|split|unsplit] — Window management")
lice5.help.add_cmd_help("gone", "/gone [message] — Set away with random reason")
lice5.help.add_cmd_help("back", "/back — Return from away")
lice5.help.add_cmd_help("ig", "/ig <pattern> [flags] — Add/toggle ignore")
lice5.help.add_cmd_help("k", "/k <nick> [reason] — Kick with random reason")
lice5.help.add_cmd_help("kb", "/kb <nick> [reason] — Kickban with random reason")
lice5.help.add_cmd_help("ul", "/ul [add|del|list] — Userlist management")
lice5.help.add_cmd_help("alarm", "/alarm [name] <seconds> <command> — Set alarm")
lice5.help.add_cmd_help("paste", "/paste [send|cancel|show] — Paste mode")
lice5.help.add_cmd_help("memo", "/memo [send|check|list] — Offline memos")
lice5.help.add_cmd_help("note", "/note [add|list|clear] — Quick notes")
lice5.help.add_cmd_help("party", "/party [on|off] — Party mode")
lice5.help.add_cmd_help("disco", "/disco <text> — Disco colored text")
lice5.help.add_cmd_help("dance", "/dance — Send random dance move")
lice5.help.add_cmd_help("sensors", "/sensors [on|off|report] — Channel monitoring")
lice5.help.add_cmd_help("protect", "/protect [#channel] — Channel protection")
lice5.help.add_cmd_help("autovoice", "/autovoice [#channel] — Auto-voice on join")
lice5.help.add_cmd_help("antiflood", "/antiflood [on|off] [threshold] — Flood protection")
lice5.help.add_cmd_help("nickserv", "/ns <password> [nick] — NickServ identify")
lice5.help.add_cmd_help("dns", "/dns <nick|host> — DNS lookup")
lice5.help.add_cmd_help("finger", "/finger <nick> — User info")
lice5.help.add_cmd_help("wall", "/wall <message> — Broadcast to channels")
lice5.help.add_cmd_help("signoff", "/signoff [reason] — Random quit message")
lice5.help.add_cmd_help("invlist", "/invlist [accept|reject|list] — Invite management")
lice5.help.add_cmd_help("rawlog", "/rawlog [on|off|show|save] — Raw IRC log")
lice5.help.add_cmd_help("chathistory", "/chathistory <before|after|latest> <target> <limit> — Message history")
lice5.help.add_cmd_help("caplist", "/caplist — List active IRCv3 capabilities")
lice5.help.add_cmd_help("starttls", "/starttls — Upgrade to TLS")
lice5.help.add_cmd_help("charset", "/charset [encoding] — Set character encoding")
lice5.help.add_cmd_help("bouncer", "/bouncer [start|stop|status] [port] [password] — IRC bouncer")

-- Command: /help [category|command]
void.register_command("LICE_HELP", "lice5_cmd_help")
function lice5_cmd_help(args)
    if #args == 0 then
        void.echo("-!- Help categories:")
        for cat, cmds in pairs(lice5.help.categories) do
            void.echo("  " .. cat .. ": " .. #cmds .. " commands")
        end
        void.echo("-!- Type /help <category> or /help <command>")
        void.echo("-!- Type /module <name> for module details")
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
    -- Check command help
    local help_text = lice5.help.cmd_help[query]
    if help_text then
        void.echo("-!- " .. help_text)
        return
    end
    void.echo("-!- No help for: " .. query)
end
