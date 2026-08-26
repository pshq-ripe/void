-- LiCe5: Exception List Management
-- Display and manage channel ban exceptions (+e)

lice5.exclist = {}

function lice5.exclist.show(channel)
    void.echo("-!- Requesting exception list for " .. channel)
    void.mode(channel, "+e")
end

function lice5.exclist.clear(channel)
    void.echo("-!- Clearing exception list for " .. channel)
    void.mode(channel, "-e")
end

-- Command: /exclist [#channel]
void.register_command("EXCLIST", "lice5_cmd_exclist")
function lice5_cmd_exclist(args)
    local channel = args[1] or void.channel()
    if channel == "" then
        void.echo("-!- Not in a channel.")
        return
    end
    if args[2] == "clear" then
        lice5.exclist.clear(channel)
    else
        lice5.exclist.show(channel)
    end
end
