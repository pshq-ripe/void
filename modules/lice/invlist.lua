-- LiCe5: Invite Exception List Management
-- Display and manage channel invite exceptions (+I)

lice5.invexlist = {}

function lice5.invexlist.show(channel)
    void.echo("-!- Requesting invite exception list for " .. channel)
    void.mode(channel, "+I")
end

function lice5.invexlist.clear(channel)
    void.echo("-!- Clearing invite exception list for " .. channel)
    void.mode(channel, "-I")
end

-- Command: /invexlist [#channel]
void.register_command("INVEXLIST", "lice5_cmd_invexlist")
function lice5_cmd_invexlist(args)
    local channel = args[1] or void.channel()
    if channel == "" then
        void.echo("-!- Not in a channel.")
        return
    end
    if args[2] == "clear" then
        lice5.invexlist.clear(channel)
    else
        lice5.invexlist.show(channel)
    end
end
