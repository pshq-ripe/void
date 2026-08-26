-- LiCe5: Remove List — Remove entries from various lists
-- Unified list removal command

lice5.remove_list = {}

-- Command: /rmlist [bans|exceptions|invites] [pattern|all]
void.register_command("RMLIST", "lice5_cmd_rmlist")
function lice5_cmd_rmlist(args)
    local channel = void.channel()
    if channel == "" then
        void.echo("-!- Not in a channel.")
        return
    end
    if #args < 2 then
        void.echo("-!- Usage: /rmlist [bans|exceptions|invites] [pattern|all]")
        return
    end
    local list_type = args[1]:lower()
    local pattern = args[2]
    if list_type == "bans" or list_type == "ban" then
        if pattern == "all" then
            void.echo("-!- Removing all bans from " .. channel)
            void.mode(channel, "-b")
        else
            void.unban(channel, pattern)
        end
    elseif list_type == "exceptions" or list_type == "exc" then
        if pattern == "all" then
            void.echo("-!- Removing all exceptions from " .. channel)
            void.mode(channel, "-e")
        else
            void.mode(channel, "-e " .. pattern)
        end
    elseif list_type == "invites" or list_type == "inv" then
        if pattern == "all" then
            void.echo("-!- Removing all invite exceptions from " .. channel)
            void.mode(channel, "-I")
        else
            void.mode(channel, "-I " .. pattern)
        end
    else
        void.echo("-!- Unknown list type: " .. list_type)
    end
end
