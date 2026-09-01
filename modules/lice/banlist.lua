-- LiCe5: Ban List Management
-- Display and manage channel ban lists

lice5.banlist = {}

function lice5.banlist.show(channel)
    void.echo("-!- Requesting ban list for " .. channel)
    -- The actual ban list is handled by the native RPL_BANLIST handler
    void.mode(channel, "+b")
end

function lice5.banlist.clear(channel)
    void.echo("-!- Clearing ban list for " .. channel)
    void.mode(channel, "-b")
end

-- Command: /banlist [#channel]
void.register_command("BANLIST", "lice5_cmd_banlist")
function lice5_cmd_banlist(args)
    local channel = args[1] or void.channel()
    if channel == "" then
        void.echo("-!- Not in a channel.")
        return
    end
    if args[2] == "clear" then
        lice5.banlist.clear(channel)
    else
        lice5.banlist.show(channel)
    end
end
