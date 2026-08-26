-- LiCe5: Chanst — Channel status
-- Display channel status information

lice5.chanst = {}

-- Command: /chanst [#channel]
void.register_command("CHANST", "lice5_cmd_chanst")
function lice5_cmd_chanst(args)
    local channel = args[1] or void.channel()
    if channel == "" then
        void.echo("-!- Not in a channel.")
        return
    end
    void.echo("-!- Channel status: " .. channel)
    void.echo("-!- (Use /names for user list, /mode for modes)")
end
