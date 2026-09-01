-- LiCe5: Stubs — DCC advanced commands
-- These are DCC-related commands that extend the basic DCC functionality

lice5.stubs = {}

-- ADCC: Advanced DCC — initiate DCC with specific options
void.register_command("ADCC", "lice5_cmd_adcc")
function lice5_cmd_adcc(args)
    if #args < 2 then
        void.echo("-!- Usage: /adcc <nick> <file> [options]")
        void.echo("-!- Options: passive, turbo, resume")
        return
    end
    local nick = args[1]
    local file = args[2]
    local opts = #args > 2 and table.concat(args, " ", 3) or ""
    void.echo("-!- ADCC: sending " .. file .. " to " .. nick .. " (" .. opts .. ")")
    -- Use native DCC send
    void.msg(nick, "\001DCC SEND " .. file .. " 0 0\001")
end

-- DCCLIST: List all DCC sessions
void.register_command("DCCLIST", "lice5_cmd_dcclist")
function lice5_cmd_dcclist(args)
    void.echo("-!- DCC sessions: (use /dcc list for native list)")
end

-- RDCC: Reverse DCC — request sender to connect to us
void.register_command("RDCC", "lice5_cmd_rdcc")
function lice5_cmd_rdcc(args)
    if #args < 2 then
        void.echo("-!- Usage: /rdcc <nick> <file>")
        return
    end
    local nick = args[1]
    local file = args[2]
    void.echo("-!- RDCC: requesting " .. nick .. " to send " .. file)
    void.msg(nick, "\001DCC SEND " .. file .. " 0 0\001")
end

-- REDCC: Reverse DCC resume
void.register_command("REDCC", "lice5_cmd_redcc")
function lice5_cmd_redcc(args)
    if #args < 2 then
        void.echo("-!- Usage: /redcc <nick> <file>")
        return
    end
    void.echo("-!- REDCC: reverse DCC resume not yet fully implemented")
end
