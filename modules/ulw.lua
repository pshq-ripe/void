-- LiCe5: Ulw_* — Userlist window commands
-- Quick userlist operations from window

lice5.ulw = {}

-- Command: /ulw_chat <nick> — open chat with userlist user
void.register_command("ULW_CHAT", "lice5_cmd_ulw_chat")
function lice5_cmd_ulw_chat(args)
    if #args == 0 then
        void.echo("-!- Usage: /ulw_chat <nick>")
        return
    end
    void.query(args[1])
end

-- Command: /ulw_help — userlist help
void.register_command("ULW_HELP", "lice5_cmd_ulw_help")
function lice5_cmd_ulw_help(args)
    void.echo("-!- Userlist commands:")
    void.echo("  /ul add <nick> [host] [level] — Add user")
    void.echo("  /ul del <nick> — Remove user")
    void.echo("  /ul list — List users")
    void.echo("  /ul op <nick> — Set OP level")
    void.echo("  /ul voice <nick> — Set VOICE level")
    void.echo("  /ulsave save — Save to file")
    void.echo("  /ulsave load — Load from file")
end

-- Command: /ulw_ident <nick> — identify user
void.register_command("ULW_IDENT", "lice5_cmd_ulw_ident")
function lice5_cmd_ulw_ident(args)
    if #args == 0 then
        void.echo("-!- Usage: /ulw_ident <nick>")
        return
    end
    void.whois(args[1])
end

-- Command: /ulw_invite <nick> — invite user
void.register_command("ULW_INVITE", "lice5_cmd_ulw_invite")
function lice5_cmd_ulw_invite(args)
    if #args == 0 then
        void.echo("-!- Usage: /ulw_invite <nick>")
        return
    end
    local channel = void.channel()
    if channel == "" then
        void.echo("-!- Not in a channel.")
        return
    end
    void.invite(args[1], channel)
end

-- Command: /ulw_op <nick> — op user
void.register_command("ULW_OP", "lice5_cmd_ulw_op")
function lice5_cmd_ulw_op(args)
    if #args == 0 then
        void.echo("-!- Usage: /ulw_op <nick>")
        return
    end
    local channel = void.channel()
    if channel == "" then
        void.echo("-!- Not in a channel.")
        return
    end
    void.op(channel, args[1])
end

-- Command: /ulw_voice <nick> — voice user
void.register_command("ULW_VOICE", "lice5_cmd_ulw_voice")
function lice5_cmd_ulw_voice(args)
    if #args == 0 then
        void.echo("-!- Usage: /ulw_voice <nick>")
        return
    end
    local channel = void.channel()
    if channel == "" then
        void.echo("-!- Not in a channel.")
        return
    end
    void.voice(channel, args[1])
end

-- Command: /ulw_unban <mask> — unban user
void.register_command("ULW_UNBAN", "lice5_cmd_ulw_unban")
function lice5_cmd_ulw_unban(args)
    if #args == 0 then
        void.echo("-!- Usage: /ulw_unban <mask>")
        return
    end
    local channel = void.channel()
    if channel == "" then
        void.echo("-!- Not in a channel.")
        return
    end
    void.unban(channel, args[1])
end

-- Command: /ulw_whoami — show your info
void.register_command("ULW_WHOAMI", "lice5_cmd_ulw_whoami")
function lice5_cmd_ulw_whoami(args)
    void.echo("-!- Nick: " .. void.nick())
    void.echo("-!- Channel: " .. void.channel())
    void.echo("-!- Server: " .. void.server())
    void.echo("-!- Connected: " .. tostring(void.connected()))
end

-- Command: /ulw_pass <nick> <password> — NickServ identify for user
void.register_command("ULW_PASS", "lice5_cmd_ulw_pass")
function lice5_cmd_ulw_pass(args)
    if #args < 2 then
        void.echo("-!- Usage: /ulw_pass <nick> <password>")
        return
    end
    void.msg("NickServ", "IDENTIFY " .. args[1] .. " " .. args[2])
    void.echo("-!- Sent IDENTIFY for " .. args[1])
end
